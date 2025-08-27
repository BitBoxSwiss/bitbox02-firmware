// Rust Bitcoin Library
// Written in 2020 by
//	 Steven Roose <steven@stevenroose.org>
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

//! # BIP39 Mnemonic Codes
//!
//! Library crate implementing [BIP39](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki)
//!

#![deny(non_upper_case_globals)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![deny(unused_mut)]
#![deny(dead_code)]
#![deny(unused_imports)]
#![deny(missing_docs)]
#![cfg_attr(all(not(test), not(feature = "std")), no_std)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(any(test, feature = "std"))]
pub extern crate core;

#[cfg(feature = "alloc")]
extern crate alloc;

extern crate bitcoin_hashes;

#[cfg(feature = "unicode-normalization")]
extern crate unicode_normalization;

#[cfg(feature = "rand")]
pub extern crate crate_rand as rand;
#[cfg(feature = "rand_core")]
pub extern crate rand_core;
#[cfg(feature = "serde")]
pub extern crate serde;

#[cfg(feature = "alloc")]
use alloc::borrow::Cow;
use core::{fmt, str};

#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::{string::ToString, vec::Vec};

/// We support a wide range of dependency versions for `rand` and `rand_core` and not
/// all versions play nicely together. These re-exports fix that.
#[cfg(all(feature = "rand", feature = "rand_core"))]
use rand::{CryptoRng, RngCore};
#[cfg(all(not(feature = "rand"), feature = "rand_core"))]
use rand_core::{CryptoRng, RngCore};

#[cfg(feature = "std")]
use std::error;

use bitcoin_hashes::{sha256, Hash};

#[cfg(feature = "unicode-normalization")]
use unicode_normalization::UnicodeNormalization;

#[cfg(feature = "zeroize")]
extern crate zeroize;
#[cfg(feature = "zeroize")]
use zeroize::{Zeroize, ZeroizeOnDrop};

#[macro_use]
mod internal_macros;
mod language;
mod pbkdf2;

pub use language::Language;

/// The minimum number of words in a mnemonic.
#[allow(unused)]
const MIN_NB_WORDS: usize = 12;

/// The maximum number of words in a mnemonic.
const MAX_NB_WORDS: usize = 24;

/// The index used to indicate the mnemonic ended.
const EOF: u16 = u16::max_value();

/// A structured used in the [Error::AmbiguousLanguages] variant that iterates
/// over the possible languages.
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct AmbiguousLanguages([bool; language::MAX_NB_LANGUAGES]);

impl AmbiguousLanguages {
	/// Presents the possible languages in the form of a slice of booleans
	/// that correspond to the occurrences in [Language::ALL].
	pub fn as_bools(&self) -> &[bool; language::MAX_NB_LANGUAGES] {
		&self.0
	}

	/// An iterator over the possible languages.
	pub fn iter(&self) -> impl Iterator<Item = Language> + '_ {
		Language::ALL.iter().enumerate().filter(move |(i, _)| self.0[*i]).map(|(_, l)| *l)
	}

	/// Returns a vector of the possible languages.
	#[cfg(feature = "alloc")]
	pub fn to_vec(&self) -> Vec<Language> {
		self.iter().collect()
	}
}

/// A BIP39 error.
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Error {
	/// Mnemonic has a word count that is not a multiple of 6.
	BadWordCount(usize),
	/// Mnemonic contains an unknown word.
	/// Error contains the index of the word.
	/// Use `mnemonic.split_whitespace().get(i)` to get the word.
	UnknownWord(usize),
	/// Entropy was not a multiple of 32 bits or between 128-256n bits in length.
	BadEntropyBitCount(usize),
	/// The mnemonic has an invalid checksum.
	InvalidChecksum,
	/// The mnemonic can be interpreted as multiple languages.
	/// Use the helper methods of the inner struct to inspect
	/// which languages are possible.
	AmbiguousLanguages(AmbiguousLanguages),
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Error::BadWordCount(c) => {
				write!(
					f,
					"mnemonic has an invalid word count: {}. Word count must be 12, 15, 18, 21, \
					or 24",
					c
				)
			}
			Error::UnknownWord(i) => write!(f, "mnemonic contains an unknown word (word {})", i,),
			Error::BadEntropyBitCount(c) => write!(
				f,
				"entropy was not between 128-256 bits or not a multiple of 32 bits: {} bits",
				c,
			),
			Error::InvalidChecksum => write!(f, "the mnemonic has an invalid checksum"),
			Error::AmbiguousLanguages(a) => {
				write!(f, "ambiguous word list: ")?;
				for (i, lang) in a.iter().enumerate() {
					if i == 0 {
						write!(f, "{}", lang)?;
					} else {
						write!(f, ", {}", lang)?;
					}
				}
				Ok(())
			}
		}
	}
}

#[cfg(feature = "std")]
impl error::Error for Error {}

/// A mnemonic code.
///
/// The [core::str::FromStr] implementation will try to determine the language of the
/// mnemonic from all the supported languages. (Languages have to be explicitly enabled using
/// the Cargo features.)
///
/// Supported number of words are 12, 15, 18, 21, and 24.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "zeroize", derive(Zeroize, ZeroizeOnDrop))]
pub struct Mnemonic {
	/// The language the mnemonic.
	lang: Language,
	/// The indices of the words.
	/// Mnemonics with less than the max nb of words are terminated with EOF.
	words: [u16; MAX_NB_WORDS],
}

#[cfg(feature = "zeroize")]
impl zeroize::DefaultIsZeroes for Language {}

serde_string_impl!(Mnemonic, "a BIP-39 Mnemonic Code");

impl Mnemonic {
	/// Ensure the content of the [Cow] is normalized UTF8.
	/// Performing this on a [Cow] means that all allocations for normalization
	/// can be avoided for languages without special UTF8 characters.
	#[inline]
	#[cfg(feature = "unicode-normalization")]
	pub fn normalize_utf8_cow<'a>(cow: &mut Cow<'a, str>) {
		let is_nfkd = unicode_normalization::is_nfkd_quick(cow.as_ref().chars());
		if is_nfkd != unicode_normalization::IsNormalized::Yes {
			*cow = Cow::Owned(cow.as_ref().nfkd().to_string());
		}
	}

	/// Create a new [Mnemonic] in the specified language from the given entropy.
	/// Entropy must be a multiple of 32 bits (4 bytes) and 128-256 bits in length.
	pub fn from_entropy_in(language: Language, entropy: &[u8]) -> Result<Mnemonic, Error> {
		const MAX_ENTROPY_BITS: usize = 256;
		const MIN_ENTROPY_BITS: usize = 128;
		const MAX_CHECKSUM_BITS: usize = 8;

		let nb_bytes = entropy.len();
		let nb_bits = nb_bytes * 8;

		if nb_bits % 32 != 0 {
			return Err(Error::BadEntropyBitCount(nb_bits));
		}
		if nb_bits < MIN_ENTROPY_BITS || nb_bits > MAX_ENTROPY_BITS {
			return Err(Error::BadEntropyBitCount(nb_bits));
		}

		let check = sha256::Hash::hash(&entropy);
		let mut bits = [false; MAX_ENTROPY_BITS + MAX_CHECKSUM_BITS];
		for i in 0..nb_bytes {
			for j in 0..8 {
				bits[i * 8 + j] = (entropy[i] & (1 << (7 - j))) > 0;
			}
		}
		for i in 0..nb_bytes / 4 {
			bits[8 * nb_bytes + i] = (check[i / 8] & (1 << (7 - (i % 8)))) > 0;
		}

		let mut words = [EOF; MAX_NB_WORDS];
		let nb_words = nb_bytes * 3 / 4;
		for i in 0..nb_words {
			let mut idx = 0;
			for j in 0..11 {
				if bits[i * 11 + j] {
					idx += 1 << (10 - j);
				}
			}
			words[i] = idx;
		}

		Ok(Mnemonic {
			lang: language,
			words: words,
		})
	}

	/// Create a new English [Mnemonic] from the given entropy.
	/// Entropy must be a multiple of 32 bits (4 bytes) and 128-256 bits in length.
	pub fn from_entropy(entropy: &[u8]) -> Result<Mnemonic, Error> {
		Mnemonic::from_entropy_in(Language::English, entropy)
	}

	/// Generate a new [Mnemonic] in the given language
	/// with the given randomness source.
	/// For the different supported word counts, see documentation on [Mnemonic].
	///
	/// Example:
	///
	/// ```
	/// use bip39::{Mnemonic, Language};
	///
	/// let mut rng = bip39::rand::thread_rng();
	/// let m = Mnemonic::generate_in_with(&mut rng, Language::English, 24).unwrap();
	/// ```
	#[cfg(feature = "rand_core")]
	pub fn generate_in_with<R>(
		rng: &mut R,
		language: Language,
		word_count: usize,
	) -> Result<Mnemonic, Error>
	where
		R: RngCore + CryptoRng,
	{
		if is_invalid_word_count(word_count) {
			return Err(Error::BadWordCount(word_count));
		}

		let entropy_bytes = (word_count / 3) * 4;
		let mut entropy = [0u8; (MAX_NB_WORDS / 3) * 4];
		RngCore::fill_bytes(rng, &mut entropy[0..entropy_bytes]);
		Mnemonic::from_entropy_in(language, &entropy[0..entropy_bytes])
	}

	/// Generate a new [Mnemonic] in the given language.
	/// For the different supported word counts, see documentation on [Mnemonic].
	///
	/// Example:
	///
	/// ```
	/// use bip39::{Mnemonic, Language};
	///
	/// let m = Mnemonic::generate_in(Language::English, 24).unwrap();
	/// ```
	#[cfg(feature = "rand")]
	pub fn generate_in(language: Language, word_count: usize) -> Result<Mnemonic, Error> {
		Mnemonic::generate_in_with(&mut rand::thread_rng(), language, word_count)
	}

	/// Generate a new [Mnemonic] in English.
	/// For the different supported word counts, see documentation on [Mnemonic].
	///
	/// Example:
	///
	/// ```
	/// use bip39::Mnemonic;
	///
	/// let m = Mnemonic::generate(24).unwrap();
	/// ```
	#[cfg(feature = "rand")]
	pub fn generate(word_count: usize) -> Result<Mnemonic, Error> {
		Mnemonic::generate_in(Language::English, word_count)
	}

	/// Get the language of the [Mnemonic].
	pub fn language(&self) -> Language {
		self.lang
	}

	/// Returns an iterator over the words of the [Mnemonic].
	///
	/// # Examples
	///
	/// Basic usage:
	///
	/// ```
	/// use bip39::Mnemonic;
	///
	/// let mnemonic = Mnemonic::from_entropy(&[0; 32]).unwrap();
	/// for (i, word) in mnemonic.words().enumerate() {
	///     println!("{}. {}", i, word);
	/// }
	/// ```
	pub fn words(&self) -> impl Iterator<Item = &'static str> + Clone + '_ {
		let list = self.lang.word_list();
		self.word_indices().map(move |i| list[i])
	}

	/// Returns an iterator over the words of the [Mnemonic].
	#[deprecated(since = "2.1.0", note = "Use Mnemonic::words instead")]
	pub fn word_iter(&self) -> impl Iterator<Item = &'static str> + Clone + '_ {
		self.words()
	}

	/// Returns an iterator over [Mnemonic] word indices.
	///
	/// # Examples
	///
	/// Basic usage:
	///
	/// ```
	/// use bip39::{Language, Mnemonic};
	///
	/// let list = Language::English.word_list();
	/// let mnemonic = Mnemonic::from_entropy(&[0; 32]).unwrap();
	/// for i in mnemonic.word_indices() {
	/// 	println!("{} ({})", list[i], i);
	/// }
	/// ```
	pub fn word_indices(&self) -> impl Iterator<Item = usize> + Clone + '_ {
		self.words.iter().take_while(|&&w| w != EOF).map(|w| *w as usize)
	}

	/// Determine the language of the mnemonic as a word iterator.
	/// See documentation on [Mnemonic::language_of] for more info.
	fn language_of_iter<'a, W: Iterator<Item = &'a str>>(words: W) -> Result<Language, Error> {
		let mut words = words.peekable();
		let langs = Language::ALL;
		{
			// Start scope to drop first_word so that words can be reborrowed later.
			let first_word = words.peek().ok_or(Error::BadWordCount(0))?;
			if first_word.len() == 0 {
				return Err(Error::BadWordCount(0));
			}

			// We first try find the first word in wordlists that
			// have guaranteed unique words.
			for language in langs.iter().filter(|l| l.unique_words()) {
				if language.find_word(first_word).is_some() {
					return Ok(*language);
				}
			}
		}

		// If that didn't work, we start with all possible languages
		// (those without unique words), and eliminate until there is
		// just one left.
		let mut possible = [false; language::MAX_NB_LANGUAGES];
		for (i, lang) in langs.iter().enumerate() {
			// To start, only consider lists that don't have unique words.
			// Those were considered above.
			possible[i] = !lang.unique_words();
		}
		for (idx, word) in words.enumerate() {
			// Scrap languages that don't have this word.
			for (i, lang) in langs.iter().enumerate() {
				possible[i] &= lang.find_word(word).is_some();
			}

			// Get an iterator over remaining possible languages.
			let mut iter = possible.iter().zip(langs.iter()).filter(|(p, _)| **p).map(|(_, l)| l);

			match iter.next() {
				// If all languages were eliminated, it's an invalid word.
				None => return Err(Error::UnknownWord(idx)),
				// If not, see if there is a second one remaining.
				Some(remaining) => {
					if iter.next().is_none() {
						// No second remaining, we found our language.
						return Ok(*remaining);
					}
				}
			}
		}

		return Err(Error::AmbiguousLanguages(AmbiguousLanguages(possible)));
	}

	/// Determine the language of the mnemonic.
	///
	/// NOTE: This method only guarantees that the returned language is the
	/// correct language on the assumption that the mnemonic is valid.
	/// It does not itself validate the mnemonic.
	///
	/// Some word lists don't guarantee that their words don't occur in other
	/// word lists. In the extremely unlikely case that a word list can be
	/// interpreted in multiple languages, an [Error::AmbiguousLanguages] is
	/// returned, containing the possible languages.
	pub fn language_of<S: AsRef<str>>(mnemonic: S) -> Result<Language, Error> {
		Mnemonic::language_of_iter(mnemonic.as_ref().split_whitespace())
	}

	/// Parse a mnemonic in normalized UTF8 in the given language.
	pub fn parse_in_normalized(language: Language, s: &str) -> Result<Mnemonic, Error> {
		let nb_words = s.split_whitespace().count();
		if is_invalid_word_count(nb_words) {
			return Err(Error::BadWordCount(nb_words));
		}

		// Here we will store the eventual words.
		let mut words = [EOF; MAX_NB_WORDS];

		// And here we keep track of the bits to calculate and validate the checksum.
		// We only use `nb_words * 11` elements in this array.
		let mut bits = [false; MAX_NB_WORDS * 11];

		for (i, word) in s.split_whitespace().enumerate() {
			let idx = language.find_word(word).ok_or(Error::UnknownWord(i))?;

			words[i] = idx;

			for j in 0..11 {
				bits[i * 11 + j] = idx >> (10 - j) & 1 == 1;
			}
		}

		// Verify the checksum.
		// We only use `nb_words / 3 * 4` elements in this array.
		let mut entropy = [0u8; MAX_NB_WORDS / 3 * 4];
		let nb_bytes_entropy = nb_words / 3 * 4;
		for i in 0..nb_bytes_entropy {
			for j in 0..8 {
				if bits[i * 8 + j] {
					entropy[i] += 1 << (7 - j);
				}
			}
		}
		let check = sha256::Hash::hash(&entropy[0..nb_bytes_entropy]);
		for i in 0..nb_bytes_entropy / 4 {
			if bits[8 * nb_bytes_entropy + i] != ((check[i / 8] & (1 << (7 - (i % 8)))) > 0) {
				return Err(Error::InvalidChecksum);
			}
		}

		Ok(Mnemonic {
			lang: language,
			words: words,
		})
	}

	/// Parse a mnemonic in normalized UTF8 in the given language without checksum check.
	///
	/// It is advised to use this method together with the utility methods
	/// - [Mnemonic::normalize_utf8_cow]
	/// - [Mnemonic::language_of]
	pub fn parse_in_normalized_without_checksum_check(
		language: Language,
		s: &str,
	) -> Result<Mnemonic, Error> {
		let nb_words = s.split_whitespace().count();
		if is_invalid_word_count(nb_words) {
			return Err(Error::BadWordCount(nb_words));
		}

		// Here we will store the eventual words.
		let mut words = [EOF; MAX_NB_WORDS];

		for (i, word) in s.split_whitespace().enumerate() {
			let idx = language.find_word(word).ok_or(Error::UnknownWord(i))?;

			words[i] = idx;
		}

		Ok(Mnemonic {
			lang: language,
			words: words,
		})
	}

	/// Parse a mnemonic in normalized UTF8.
	pub fn parse_normalized(s: &str) -> Result<Mnemonic, Error> {
		let lang = Mnemonic::language_of(s)?;
		Mnemonic::parse_in_normalized(lang, s)
	}

	/// Parse a mnemonic in the given language.
	#[cfg(feature = "unicode-normalization")]
	pub fn parse_in<'a, S: Into<Cow<'a, str>>>(
		language: Language,
		s: S,
	) -> Result<Mnemonic, Error> {
		let mut cow = s.into();
		Mnemonic::normalize_utf8_cow(&mut cow);
		Ok(Mnemonic::parse_in_normalized(language, cow.as_ref())?)
	}

	/// Parse a mnemonic and detect the language from the enabled languages.
	#[cfg(feature = "unicode-normalization")]
	pub fn parse<'a, S: Into<Cow<'a, str>>>(s: S) -> Result<Mnemonic, Error> {
		let mut cow = s.into();
		Mnemonic::normalize_utf8_cow(&mut cow);

		let language = if Language::ALL.len() == 1 {
			Language::ALL[0]
		} else {
			Mnemonic::language_of(cow.as_ref())?
		};

		Ok(Mnemonic::parse_in_normalized(language, cow.as_ref())?)
	}

	/// Get the number of words in the mnemonic.
	pub fn word_count(&self) -> usize {
		self.word_indices().count()
	}

	/// Convert to seed bytes with a passphrase in normalized UTF8.
	pub fn to_seed_normalized(&self, normalized_passphrase: &str) -> [u8; 64] {
		const PBKDF2_ROUNDS: usize = 2048;
		const PBKDF2_BYTES: usize = 64;

		let mut seed = [0u8; PBKDF2_BYTES];
		pbkdf2::pbkdf2(self.words(), normalized_passphrase.as_bytes(), PBKDF2_ROUNDS, &mut seed);
		seed
	}

	/// Convert to seed bytes.
	#[cfg(feature = "unicode-normalization")]
	pub fn to_seed<'a, P: Into<Cow<'a, str>>>(&self, passphrase: P) -> [u8; 64] {
		let normalized_passphrase = {
			let mut cow = passphrase.into();
			Mnemonic::normalize_utf8_cow(&mut cow);
			cow
		};
		self.to_seed_normalized(normalized_passphrase.as_ref())
	}

	/// Convert the mnemonic back to the entropy used to generate it.
	/// The return value is a byte array and the size.
	/// Use [Mnemonic::to_entropy] (needs `std`) to get a [`Vec<u8>`].
	pub fn to_entropy_array(&self) -> ([u8; 33], usize) {
		// We unwrap errors here because this method can only be called on
		// values that were already previously validated.

		let language = Mnemonic::language_of_iter(self.words()).unwrap();

		// Preallocate enough space for the longest possible word list
		let mut entropy = [0; 33];
		let mut cursor = 0;
		let mut offset = 0;
		let mut remainder = 0;

		let nb_words = self.word_count();
		for word in self.words() {
			let idx = language.find_word(word).expect("invalid mnemonic");

			remainder |= ((idx as u32) << (32 - 11)) >> offset;
			offset += 11;

			while offset >= 8 {
				entropy[cursor] = (remainder >> 24) as u8;
				cursor += 1;
				remainder <<= 8;
				offset -= 8;
			}
		}

		if offset != 0 {
			entropy[cursor] = (remainder >> 24) as u8;
		}

		let entropy_bytes = (nb_words / 3) * 4;
		(entropy, entropy_bytes)
	}

	/// Convert the mnemonic back to the entropy used to generate it.
	#[cfg(feature = "alloc")]
	pub fn to_entropy(&self) -> Vec<u8> {
		let (arr, len) = self.to_entropy_array();
		arr[0..len].to_vec()
	}

	/// Return checksum value for the Mnemonic.
	///
	/// The checksum value is the numerical value of the first `self.word_count() / 3` bits of the
	/// [SHA256](https://en.wikipedia.org/wiki/SHA-2) digest of the Mnemonic's entropy, and is
	/// encoded by the last word of the mnemonic sentence.
	///
	/// This is useful for validating the integrity of a mnemonic: For a valid mnemonic `m`, the
	/// following assertion should hold:
	///
	/// ```rust
	/// # use bip39::Mnemonic;
	/// # use bitcoin_hashes::{Hash, sha256, hex::FromHex};
	/// # let ent = Vec::from_hex("98FE3D0FF6E955A484B0A1D0C9CE10F6").unwrap();
	/// # let m = Mnemonic::from_entropy(&ent).unwrap();
	/// let checksum_width = m.word_count() / 3;
	/// let shift_width = 8 - checksum_width;
	/// assert_eq!(sha256::Hash::hash(&m.to_entropy())[0] >> shift_width, m.checksum());
	/// ```
	///
	/// Note that since this library constrains initialization of `Mnemonic` instances through an
	/// API that guarantees validity, all `Mnemonic` instances should be valid and the above
	/// condition should hold.
	pub fn checksum(&self) -> u8 {
		let word_count = self.word_count();
		let last_word = self.words[word_count - 1];
		let mask = 0xFF >> (8 - word_count / 3);
		last_word as u8 & mask
	}
}

impl fmt::Display for Mnemonic {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for (i, word) in self.words().enumerate() {
			if i > 0 {
				f.write_str(" ")?;
			}
			f.write_str(word)?;
		}
		Ok(())
	}
}

impl str::FromStr for Mnemonic {
	type Err = Error;

	fn from_str(s: &str) -> Result<Mnemonic, Error> {
		#[cfg(feature = "unicode-normalization")]
		{
			Mnemonic::parse(s)
		}
		#[cfg(not(feature = "unicode-normalization"))]
		{
			Mnemonic::parse_normalized(s)
		}
	}
}

fn is_invalid_word_count(word_count: usize) -> bool {
	word_count < MIN_NB_WORDS || word_count % 3 != 0 || word_count > MAX_NB_WORDS
}

#[cfg(test)]
mod tests {
	use super::*;

	use bitcoin_hashes::hex::FromHex;

	#[cfg(feature = "rand")]
	#[test]
	fn test_language_of() {
		for lang in Language::ALL {
			let m = Mnemonic::generate_in(*lang, 24).unwrap();
			assert_eq!(*lang, Mnemonic::language_of_iter(m.words()).unwrap());
			assert_eq!(
				*lang,
				Mnemonic::language_of_iter(m.to_string().split_whitespace()).unwrap()
			);
			assert_eq!(*lang, Mnemonic::language_of(m.to_string()).unwrap());
			assert_eq!(*lang, Mnemonic::language_of(&m.to_string()).unwrap());
		}
	}

	#[cfg(feature = "std")]
	#[test]
	fn test_ambiguous_languages() {
		let mut present = [false; language::MAX_NB_LANGUAGES];
		let mut present_vec = Vec::new();
		let mut alternate = true;
		for i in 0..Language::ALL.len() {
			present[i] = alternate;
			if alternate {
				present_vec.push(Language::ALL[i]);
			}
			alternate = !alternate;
		}
		let amb = AmbiguousLanguages(present);
		assert_eq!(amb.to_vec(), present_vec);
		assert_eq!(amb.iter().collect::<Vec<_>>(), present_vec);
	}

	#[cfg(feature = "rand")]
	#[test]
	fn test_generate() {
		let _ = Mnemonic::generate(24).unwrap();
		let _ = Mnemonic::generate_in(Language::English, 24).unwrap();
		let _ = Mnemonic::generate_in_with(&mut rand::thread_rng(), Language::English, 24).unwrap();
	}

	#[cfg(feature = "rand")]
	#[test]
	fn test_generate_word_counts() {
		for word_count in [12, 15, 18, 21, 24].iter() {
			let _ = Mnemonic::generate(*word_count).unwrap();
		}
	}

	#[test]
	fn test_vectors_english() {
		// These vectors are tuples of
		// (entropy, mnemonic, seed)
		let test_vectors = [
			(
				"00000000000000000000000000000000",
				"abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about",
				"c55257c360c07c72029aebc1b53c05ed0362ada38ead3e3e9efa3708e53495531f09a6987599d18264c1e1c92f2cf141630c7a3c4ab7c81b2f001698e7463b04",
			),
			(
				"7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f",
				"legal winner thank year wave sausage worth useful legal winner thank yellow",
				"2e8905819b8723fe2c1d161860e5ee1830318dbf49a83bd451cfb8440c28bd6fa457fe1296106559a3c80937a1c1069be3a3a5bd381ee6260e8d9739fce1f607",
			),
			(
				"80808080808080808080808080808080",
				"letter advice cage absurd amount doctor acoustic avoid letter advice cage above",
				"d71de856f81a8acc65e6fc851a38d4d7ec216fd0796d0a6827a3ad6ed5511a30fa280f12eb2e47ed2ac03b5c462a0358d18d69fe4f985ec81778c1b370b652a8",
			),
			(
				"ffffffffffffffffffffffffffffffff",
				"zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong",
				"ac27495480225222079d7be181583751e86f571027b0497b5b5d11218e0a8a13332572917f0f8e5a589620c6f15b11c61dee327651a14c34e18231052e48c069",
			),
			(
				"000000000000000000000000000000000000000000000000",
				"abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon agent",
				"035895f2f481b1b0f01fcf8c289c794660b289981a78f8106447707fdd9666ca06da5a9a565181599b79f53b844d8a71dd9f439c52a3d7b3e8a79c906ac845fa",
			),
			(
				"7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f",
				"legal winner thank year wave sausage worth useful legal winner thank year wave sausage worth useful legal will",
				"f2b94508732bcbacbcc020faefecfc89feafa6649a5491b8c952cede496c214a0c7b3c392d168748f2d4a612bada0753b52a1c7ac53c1e93abd5c6320b9e95dd",
			),
			(
				"808080808080808080808080808080808080808080808080",
				"letter advice cage absurd amount doctor acoustic avoid letter advice cage absurd amount doctor acoustic avoid letter always",
				"107d7c02a5aa6f38c58083ff74f04c607c2d2c0ecc55501dadd72d025b751bc27fe913ffb796f841c49b1d33b610cf0e91d3aa239027f5e99fe4ce9e5088cd65",
			),
			(
				"ffffffffffffffffffffffffffffffffffffffffffffffff",
				"zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo when",
				"0cd6e5d827bb62eb8fc1e262254223817fd068a74b5b449cc2f667c3f1f985a76379b43348d952e2265b4cd129090758b3e3c2c49103b5051aac2eaeb890a528",
			),
			(
				"0000000000000000000000000000000000000000000000000000000000000000",
				"abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon art",
				"bda85446c68413707090a52022edd26a1c9462295029f2e60cd7c4f2bbd3097170af7a4d73245cafa9c3cca8d561a7c3de6f5d4a10be8ed2a5e608d68f92fcc8",
			),
			(
				"7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f",
				"legal winner thank year wave sausage worth useful legal winner thank year wave sausage worth useful legal winner thank year wave sausage worth title",
				"bc09fca1804f7e69da93c2f2028eb238c227f2e9dda30cd63699232578480a4021b146ad717fbb7e451ce9eb835f43620bf5c514db0f8add49f5d121449d3e87",
			),
			(
				"8080808080808080808080808080808080808080808080808080808080808080",
				"letter advice cage absurd amount doctor acoustic avoid letter advice cage absurd amount doctor acoustic avoid letter advice cage absurd amount doctor acoustic bless",
				"c0c519bd0e91a2ed54357d9d1ebef6f5af218a153624cf4f2da911a0ed8f7a09e2ef61af0aca007096df430022f7a2b6fb91661a9589097069720d015e4e982f",
			),
			(
				"ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
				"zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo vote",
				"dd48c104698c30cfe2b6142103248622fb7bb0ff692eebb00089b32d22484e1613912f0a5b694407be899ffd31ed3992c456cdf60f5d4564b8ba3f05a69890ad",
			),
			(
				"9e885d952ad362caeb4efe34a8e91bd2",
				"ozone drill grab fiber curtain grace pudding thank cruise elder eight picnic",
				"274ddc525802f7c828d8ef7ddbcdc5304e87ac3535913611fbbfa986d0c9e5476c91689f9c8a54fd55bd38606aa6a8595ad213d4c9c9f9aca3fb217069a41028",
			),
			(
				"6610b25967cdcca9d59875f5cb50b0ea75433311869e930b",
				"gravity machine north sort system female filter attitude volume fold club stay feature office ecology stable narrow fog",
				"628c3827a8823298ee685db84f55caa34b5cc195a778e52d45f59bcf75aba68e4d7590e101dc414bc1bbd5737666fbbef35d1f1903953b66624f910feef245ac",
			),
			(
				"68a79eaca2324873eacc50cb9c6eca8cc68ea5d936f98787c60c7ebc74e6ce7c",
				"hamster diagram private dutch cause delay private meat slide toddler razor book happy fancy gospel tennis maple dilemma loan word shrug inflict delay length",
				"64c87cde7e12ecf6704ab95bb1408bef047c22db4cc7491c4271d170a1b213d20b385bc1588d9c7b38f1b39d415665b8a9030c9ec653d75e65f847d8fc1fc440",
			),
			(
				"c0ba5a8e914111210f2bd131f3d5e08d",
				"scheme spot photo card baby mountain device kick cradle pact join borrow",
				"ea725895aaae8d4c1cf682c1bfd2d358d52ed9f0f0591131b559e2724bb234fca05aa9c02c57407e04ee9dc3b454aa63fbff483a8b11de949624b9f1831a9612",
			),
			(
				"6d9be1ee6ebd27a258115aad99b7317b9c8d28b6d76431c3",
				"horn tenant knee talent sponsor spell gate clip pulse soap slush warm silver nephew swap uncle crack brave",
				"fd579828af3da1d32544ce4db5c73d53fc8acc4ddb1e3b251a31179cdb71e853c56d2fcb11aed39898ce6c34b10b5382772db8796e52837b54468aeb312cfc3d",
			),
			(
				"9f6a2878b2520799a44ef18bc7df394e7061a224d2c33cd015b157d746869863",
				"panda eyebrow bullet gorilla call smoke muffin taste mesh discover soft ostrich alcohol speed nation flash devote level hobby quick inner drive ghost inside",
				"72be8e052fc4919d2adf28d5306b5474b0069df35b02303de8c1729c9538dbb6fc2d731d5f832193cd9fb6aeecbc469594a70e3dd50811b5067f3b88b28c3e8d",
			),
			(
				"23db8160a31d3e0dca3688ed941adbf3",
				"cat swing flag economy stadium alone churn speed unique patch report train",
				"deb5f45449e615feff5640f2e49f933ff51895de3b4381832b3139941c57b59205a42480c52175b6efcffaa58a2503887c1e8b363a707256bdd2b587b46541f5",
			),
			(
				"8197a4a47f0425faeaa69deebc05ca29c0a5b5cc76ceacc0",
				"light rule cinnamon wrap drastic word pride squirrel upgrade then income fatal apart sustain crack supply proud access",
				"4cbdff1ca2db800fd61cae72a57475fdc6bab03e441fd63f96dabd1f183ef5b782925f00105f318309a7e9c3ea6967c7801e46c8a58082674c860a37b93eda02",
			),
			(
				"066dca1a2bb7e8a1db2832148ce9933eea0f3ac9548d793112d9a95c9407efad",
				"all hour make first leader extend hole alien behind guard gospel lava path output census museum junior mass reopen famous sing advance salt reform",
				"26e975ec644423f4a4c4f4215ef09b4bd7ef924e85d1d17c4cf3f136c2863cf6df0a475045652c57eb5fb41513ca2a2d67722b77e954b4b3fc11f7590449191d",
			),
			(
				"f30f8c1da665478f49b001d94c5fc452",
				"vessel ladder alter error federal sibling chat ability sun glass valve picture",
				"2aaa9242daafcee6aa9d7269f17d4efe271e1b9a529178d7dc139cd18747090bf9d60295d0ce74309a78852a9caadf0af48aae1c6253839624076224374bc63f",
			),
			(
				"c10ec20dc3cd9f652c7fac2f1230f7a3c828389a14392f05",
				"scissors invite lock maple supreme raw rapid void congress muscle digital elegant little brisk hair mango congress clump",
				"7b4a10be9d98e6cba265566db7f136718e1398c71cb581e1b2f464cac1ceedf4f3e274dc270003c670ad8d02c4558b2f8e39edea2775c9e232c7cb798b069e88",
			),
			(
				"f585c11aec520db57dd353c69554b21a89b20fb0650966fa0a9d6f74fd989d8f",
				"void come effort suffer camp survey warrior heavy shoot primary clutch crush open amazing screen patrol group space point ten exist slush involve unfold",
				"01f5bced59dec48e362f2c45b5de68b9fd6c92c6634f44d6d40aab69056506f0e35524a518034ddc1192e1dacd32c1ed3eaa3c3b131c88ed8e7e54c49a5d0998",
			)
		];

		for vector in &test_vectors {
			let entropy = Vec::<u8>::from_hex(&vector.0).unwrap();
			let mnemonic_str = vector.1;
			let seed = Vec::<u8>::from_hex(&vector.2).unwrap();

			let mnemonic = Mnemonic::from_entropy(&entropy).unwrap();

			assert_eq!(
				mnemonic,
				Mnemonic::parse_in_normalized(Language::English, mnemonic_str).unwrap(),
				"failed vector: {}",
				mnemonic_str
			);
			assert_eq!(
				mnemonic,
				Mnemonic::parse_normalized(mnemonic_str).unwrap(),
				"failed vector: {}",
				mnemonic_str
			);
			assert_eq!(
				&seed[..],
				&mnemonic.to_seed_normalized("TREZOR")[..],
				"failed vector: {}",
				mnemonic_str
			);

			#[cfg(feature = "unicode-normalization")]
			{
				assert_eq!(&mnemonic.to_string(), mnemonic_str, "failed vector: {}", mnemonic_str);
				assert_eq!(
					mnemonic,
					Mnemonic::parse_in(Language::English, mnemonic_str).unwrap(),
					"failed vector: {}",
					mnemonic_str
				);
				assert_eq!(
					mnemonic,
					Mnemonic::parse(mnemonic_str).unwrap(),
					"failed vector: {}",
					mnemonic_str
				);
				assert_eq!(
					&seed[..],
					&mnemonic.to_seed("TREZOR")[..],
					"failed vector: {}",
					mnemonic_str
				);
				assert_eq!(&entropy, &mnemonic.to_entropy(), "failed vector: {}", mnemonic_str);
				assert_eq!(
					&entropy[..],
					&mnemonic.to_entropy_array().0[0..entropy.len()],
					"failed vector: {}",
					mnemonic_str
				);
			}
		}
	}

	#[test]
	fn checksum() {
		let vectors = [
			"00000000000000000000000000000000",
			"7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f",
			"80808080808080808080808080808080",
			"ffffffffffffffffffffffffffffffff",
			"000000000000000000000000000000000000000000000000",
			"7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f",
			"808080808080808080808080808080808080808080808080",
			"ffffffffffffffffffffffffffffffffffffffffffffffff",
			"0000000000000000000000000000000000000000000000000000000000000000",
			"7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f",
			"8080808080808080808080808080808080808080808080808080808080808080",
			"ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
			"9e885d952ad362caeb4efe34a8e91bd2",
			"6610b25967cdcca9d59875f5cb50b0ea75433311869e930b",
			"68a79eaca2324873eacc50cb9c6eca8cc68ea5d936f98787c60c7ebc74e6ce7c",
			"c0ba5a8e914111210f2bd131f3d5e08d",
			"6d9be1ee6ebd27a258115aad99b7317b9c8d28b6d76431c3",
			"9f6a2878b2520799a44ef18bc7df394e7061a224d2c33cd015b157d746869863",
			"23db8160a31d3e0dca3688ed941adbf3",
			"8197a4a47f0425faeaa69deebc05ca29c0a5b5cc76ceacc0",
			"066dca1a2bb7e8a1db2832148ce9933eea0f3ac9548d793112d9a95c9407efad",
			"f30f8c1da665478f49b001d94c5fc452",
			"c10ec20dc3cd9f652c7fac2f1230f7a3c828389a14392f05",
			"f585c11aec520db57dd353c69554b21a89b20fb0650966fa0a9d6f74fd989d8f",
			"ed3b83f0d7913a19667a1cfd7298cd57",
			"70639a4e81b151277b345476d169a3743ff3c141",
			"ba2520298b92063a7a0ee1d453ba92513af81d4f86e1d336",
			"9447d2cf44349cd88a58f5b4ff6f83b9a2d54c42f033e12b8e4d00cc",
			"38711e550dc6557df8082b2a87f7860ebbe47ea5867a7068f5f0f5b85db68be8",
		];

		for entropy_hex in &vectors {
			let ent = Vec::from_hex(entropy_hex).unwrap();
			let m = Mnemonic::from_entropy(&ent).unwrap();
			let word_count = m.word_count();
			let cs = m.checksum();
			let digest = sha256::Hash::hash(&ent);
			dbg!(digest);
			assert_eq!(digest[0] >> (8 - word_count / 3), cs);
		}
	}

	#[test]
	fn test_invalid_engish() {
		// correct phrase:
		// "letter advice cage absurd amount doctor acoustic avoid letter advice cage above"

		assert_eq!(
			Mnemonic::parse_normalized(
				"getter advice cage absurd amount doctor acoustic avoid letter advice cage above",
			),
			Err(Error::UnknownWord(0))
		);

		assert_eq!(
			Mnemonic::parse_normalized(
				"letter advice cagex absurd amount doctor acoustic avoid letter advice cage above",
			),
			Err(Error::UnknownWord(2))
		);

		assert_eq!(
			Mnemonic::parse_normalized(
				"advice cage absurd amount doctor acoustic avoid letter advice cage above",
			),
			Err(Error::BadWordCount(11))
		);

		assert_eq!(
			Mnemonic::parse_normalized(
				"primary advice cage absurd amount doctor acoustic avoid letter advice cage above",
			),
			Err(Error::InvalidChecksum)
		);
	}

	#[test]
	fn test_invalid_entropy() {
		//between 128 and 256 bits, but not divisible by 32
		assert_eq!(Mnemonic::from_entropy(&vec![b'x'; 17]), Err(Error::BadEntropyBitCount(136)));

		//less than 128 bits
		assert_eq!(Mnemonic::from_entropy(&vec![b'x'; 4]), Err(Error::BadEntropyBitCount(32)));

		//greater than 256 bits
		assert_eq!(Mnemonic::from_entropy(&vec![b'x'; 36]), Err(Error::BadEntropyBitCount(288)));
	}

	#[cfg(all(feature = "japanese", feature = "std"))]
	#[test]
	fn test_vectors_japanese() {
		//! Test some Japanese language test vectors.
		//! For these test vectors, we seem to generate different mnemonic phrases than the test
		//! vectors expect us to. However, our generated seeds are correct and tiny-bip39,
		//! an alternative implementation of bip39 also does not fulfill the test vectors.

		// These vectors are tuples of
		// (entropy, mnemonic, passphrase, seed)
		let vectors = [
			(
				"00000000000000000000000000000000",
				"あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あおぞら",
				"㍍ガバヴァぱばぐゞちぢ十人十色",
				"a262d6fb6122ecf45be09c50492b31f92e9beb7d9a845987a02cefda57a15f9c467a17872029a9e92299b5cbdf306e3a0ee620245cbd508959b6cb7ca637bd55",
			),
			(
				"7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f",
				"そつう　れきだい　ほんやく　わかす　りくつ　ばいか　ろせん　やちん　そつう　れきだい　ほんやく　わかめ",
				"㍍ガバヴァぱばぐゞちぢ十人十色",
				"aee025cbe6ca256862f889e48110a6a382365142f7d16f2b9545285b3af64e542143a577e9c144e101a6bdca18f8d97ec3366ebf5b088b1c1af9bc31346e60d9",
			),
			(
				"80808080808080808080808080808080",
				"そとづら　あまど　おおう　あこがれる　いくぶん　けいけん　あたえる　いよく　そとづら　あまど　おおう　あかちゃん",
				"㍍ガバヴァぱばぐゞちぢ十人十色",
				"e51736736ebdf77eda23fa17e31475fa1d9509c78f1deb6b4aacfbd760a7e2ad769c714352c95143b5c1241985bcb407df36d64e75dd5a2b78ca5d2ba82a3544",
			),
			(
				"ffffffffffffffffffffffffffffffff",
				"われる　われる　われる　われる　われる　われる　われる　われる　われる　われる　われる　ろんぶん",
				"㍍ガバヴァぱばぐゞちぢ十人十色",
				"4cd2ef49b479af5e1efbbd1e0bdc117f6a29b1010211df4f78e2ed40082865793e57949236c43b9fe591ec70e5bb4298b8b71dc4b267bb96ed4ed282c8f7761c",
			),
			(
				"000000000000000000000000000000000000000000000000",
				"あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あらいぐま",
				"㍍ガバヴァぱばぐゞちぢ十人十色",
				"d99e8f1ce2d4288d30b9c815ae981edd923c01aa4ffdc5dee1ab5fe0d4a3e13966023324d119105aff266dac32e5cd11431eeca23bbd7202ff423f30d6776d69",
			),
			(
				"7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f",
				"そつう　れきだい　ほんやく　わかす　りくつ　ばいか　ろせん　やちん　そつう　れきだい　ほんやく　わかす　りくつ　ばいか　ろせん　やちん　そつう　れいぎ",
				"㍍ガバヴァぱばぐゞちぢ十人十色",
				"eaaf171efa5de4838c758a93d6c86d2677d4ccda4a064a7136344e975f91fe61340ec8a615464b461d67baaf12b62ab5e742f944c7bd4ab6c341fbafba435716",
			),
			(
				"808080808080808080808080808080808080808080808080",
				"そとづら　あまど　おおう　あこがれる　いくぶん　けいけん　あたえる　いよく　そとづら　あまど　おおう　あこがれる　いくぶん　けいけん　あたえる　いよく　そとづら　いきなり",
				"㍍ガバヴァぱばぐゞちぢ十人十色",
				"aec0f8d3167a10683374c222e6e632f2940c0826587ea0a73ac5d0493b6a632590179a6538287641a9fc9df8e6f24e01bf1be548e1f74fd7407ccd72ecebe425",
			),
			(
				"ffffffffffffffffffffffffffffffffffffffffffffffff",
				"われる　われる　われる　われる　われる　われる　われる　われる　われる　われる　われる　われる　われる　われる　われる　われる　われる　りんご",
				"㍍ガバヴァぱばぐゞちぢ十人十色",
				"f0f738128a65b8d1854d68de50ed97ac1831fc3a978c569e415bbcb431a6a671d4377e3b56abd518daa861676c4da75a19ccb41e00c37d086941e471a4374b95",
			),
			(
				"0000000000000000000000000000000000000000000000000000000000000000",
				"あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　あいこくしん　いってい",
				"㍍ガバヴァぱばぐゞちぢ十人十色",
				"23f500eec4a563bf90cfda87b3e590b211b959985c555d17e88f46f7183590cd5793458b094a4dccc8f05807ec7bd2d19ce269e20568936a751f6f1ec7c14ddd",
			),
			(
				"7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f",
				"そつう　れきだい　ほんやく　わかす　りくつ　ばいか　ろせん　やちん　そつう　れきだい　ほんやく　わかす　りくつ　ばいか　ろせん　やちん　そつう　れきだい　ほんやく　わかす　りくつ　ばいか　ろせん　まんきつ",
				"㍍ガバヴァぱばぐゞちぢ十人十色",
				"cd354a40aa2e241e8f306b3b752781b70dfd1c69190e510bc1297a9c5738e833bcdc179e81707d57263fb7564466f73d30bf979725ff783fb3eb4baa86560b05",
			),
			(
				"8080808080808080808080808080808080808080808080808080808080808080",
				"そとづら　あまど　おおう　あこがれる　いくぶん　けいけん　あたえる　いよく　そとづら　あまど　おおう　あこがれる　いくぶん　けいけん　あたえる　いよく　そとづら　あまど　おおう　あこがれる　いくぶん　けいけん　あたえる　うめる",
				"㍍ガバヴァぱばぐゞちぢ十人十色",
				"6b7cd1b2cdfeeef8615077cadd6a0625f417f287652991c80206dbd82db17bf317d5c50a80bd9edd836b39daa1b6973359944c46d3fcc0129198dc7dc5cd0e68",
			),
			(
				"ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
				"われる　われる　われる　われる　われる　われる　われる　われる　われる　われる　われる　われる　われる　われる　われる　われる　われる　われる　われる　われる　われる　われる　われる　らいう",
				"㍍ガバヴァぱばぐゞちぢ十人十色",
				"a44ba7054ac2f9226929d56505a51e13acdaa8a9097923ca07ea465c4c7e294c038f3f4e7e4b373726ba0057191aced6e48ac8d183f3a11569c426f0de414623",
			),
			(
				"77c2b00716cec7213839159e404db50d",
				"せまい　うちがわ　あずき　かろう　めずらしい　だんち　ますく　おさめる　ていぼう　あたる　すあな　えしゃく",
				"㍍ガバヴァぱばぐゞちぢ十人十色",
				"344cef9efc37d0cb36d89def03d09144dd51167923487eec42c487f7428908546fa31a3c26b7391a2b3afe7db81b9f8c5007336b58e269ea0bd10749a87e0193",
			),
			(
				"b63a9c59a6e641f288ebc103017f1da9f8290b3da6bdef7b",
				"ぬすむ　ふっかつ　うどん　こうりつ　しつじ　りょうり　おたがい　せもたれ　あつめる　いちりゅう　はんしゃ　ごますり　そんけい　たいちょう　らしんばん　ぶんせき　やすみ　ほいく",
				"㍍ガバヴァぱばぐゞちぢ十人十色",
				"b14e7d35904cb8569af0d6a016cee7066335a21c1c67891b01b83033cadb3e8a034a726e3909139ecd8b2eb9e9b05245684558f329b38480e262c1d6bc20ecc4",
			),
			(
				"3e141609b97933b66a060dcddc71fad1d91677db872031e85f4c015c5e7e8982",
				"くのう　てぬぐい　そんかい　すろっと　ちきゅう　ほあん　とさか　はくしゅ　ひびく　みえる　そざい　てんすう　たんぴん　くしょう　すいようび　みけん　きさらぎ　げざん　ふくざつ　あつかう　はやい　くろう　おやゆび　こすう",
				"㍍ガバヴァぱばぐゞちぢ十人十色",
				"32e78dce2aff5db25aa7a4a32b493b5d10b4089923f3320c8b287a77e512455443298351beb3f7eb2390c4662a2e566eec5217e1a37467af43b46668d515e41b",
			),
			(
				"0460ef47585604c5660618db2e6a7e7f",
				"あみもの　いきおい　ふいうち　にげる　ざんしょ　じかん　ついか　はたん　ほあん　すんぽう　てちがい　わかめ",
				"㍍ガバヴァぱばぐゞちぢ十人十色",
				"0acf902cd391e30f3f5cb0605d72a4c849342f62bd6a360298c7013d714d7e58ddf9c7fdf141d0949f17a2c9c37ced1d8cb2edabab97c4199b142c829850154b",
			),
			(
				"72f60ebac5dd8add8d2a25a797102c3ce21bc029c200076f",
				"すろっと　にくしみ　なやむ　たとえる　へいこう　すくう　きない　けってい　とくべつ　ねっしん　いたみ　せんせい　おくりがな　まかい　とくい　けあな　いきおい　そそぐ",
				"㍍ガバヴァぱばぐゞちぢ十人十色",
				"9869e220bec09b6f0c0011f46e1f9032b269f096344028f5006a6e69ea5b0b8afabbb6944a23e11ebd021f182dd056d96e4e3657df241ca40babda532d364f73",
			),
			(
				"2c85efc7f24ee4573d2b81a6ec66cee209b2dcbd09d8eddc51e0215b0b68e416",
				"かほご　きうい　ゆたか　みすえる　もらう　がっこう　よそう　ずっと　ときどき　したうけ　にんか　はっこう　つみき　すうじつ　よけい　くげん　もくてき　まわり　せめる　げざい　にげる　にんたい　たんそく　ほそく",
				"㍍ガバヴァぱばぐゞちぢ十人十色",
				"713b7e70c9fbc18c831bfd1f03302422822c3727a93a5efb9659bec6ad8d6f2c1b5c8ed8b0b77775feaf606e9d1cc0a84ac416a85514ad59f5541ff5e0382481",
			),
			(
				"eaebabb2383351fd31d703840b32e9e2",
				"めいえん　さのう　めだつ　すてる　きぬごし　ろんぱ　はんこ　まける　たいおう　さかいし　ねんいり　はぶらし",
				"㍍ガバヴァぱばぐゞちぢ十人十色",
				"06e1d5289a97bcc95cb4a6360719131a786aba057d8efd603a547bd254261c2a97fcd3e8a4e766d5416437e956b388336d36c7ad2dba4ee6796f0249b10ee961",
			),
			(
				"7ac45cfe7722ee6c7ba84fbc2d5bd61b45cb2fe5eb65aa78",
				"せんぱい　おしえる　ぐんかん　もらう　きあい　きぼう　やおや　いせえび　のいず　じゅしん　よゆう　きみつ　さといも　ちんもく　ちわわ　しんせいじ　とめる　はちみつ",
				"㍍ガバヴァぱばぐゞちぢ十人十色",
				"1fef28785d08cbf41d7a20a3a6891043395779ed74503a5652760ee8c24dfe60972105ee71d5168071a35ab7b5bd2f8831f75488078a90f0926c8e9171b2bc4a",
			),
			(
				"4fa1a8bc3e6d80ee1316050e862c1812031493212b7ec3f3bb1b08f168cabeef",
				"こころ　いどう　きあつ　そうがんきょう　へいあん　せつりつ　ごうせい　はいち　いびき　きこく　あんい　おちつく　きこえる　けんとう　たいこ　すすめる　はっけん　ていど　はんおん　いんさつ　うなぎ　しねま　れいぼう　みつかる",
				"㍍ガバヴァぱばぐゞちぢ十人十色",
				"43de99b502e152d4c198542624511db3007c8f8f126a30818e856b2d8a20400d29e7a7e3fdd21f909e23be5e3c8d9aee3a739b0b65041ff0b8637276703f65c2",
			),
			(
				"18ab19a9f54a9274f03e5209a2ac8a91",
				"うりきれ　さいせい　じゆう　むろん　とどける　ぐうたら　はいれつ　ひけつ　いずれ　うちあわせ　おさめる　おたく",
				"㍍ガバヴァぱばぐゞちぢ十人十色",
				"3d711f075ee44d8b535bb4561ad76d7d5350ea0b1f5d2eac054e869ff7963cdce9581097a477d697a2a9433a0c6884bea10a2193647677977c9820dd0921cbde",
			),
			(
				"18a2e1d81b8ecfb2a333adcb0c17a5b9eb76cc5d05db91a4",
				"うりきれ　うねる　せっさたくま　きもち　めんきょ　へいたく　たまご　ぜっく　びじゅつかん　さんそ　むせる　せいじ　ねくたい　しはらい　せおう　ねんど　たんまつ　がいけん",
				"㍍ガバヴァぱばぐゞちぢ十人十色",
				"753ec9e333e616e9471482b4b70a18d413241f1e335c65cd7996f32b66cf95546612c51dcf12ead6f805f9ee3d965846b894ae99b24204954be80810d292fcdd",
			),
			(
				"15da872c95a13dd738fbf50e427583ad61f18fd99f628c417a61cf8343c90419",
				"うちゅう　ふそく　ひしょ　がちょう　うけもつ　めいそう　みかん　そざい　いばる　うけとる　さんま　さこつ　おうさま　ぱんつ　しひょう　めした　たはつ　いちぶ　つうじょう　てさぎょう　きつね　みすえる　いりぐち　かめれおん",
				"㍍ガバヴァぱばぐゞちぢ十人十色",
				"346b7321d8c04f6f37b49fdf062a2fddc8e1bf8f1d33171b65074531ec546d1d3469974beccb1a09263440fc92e1042580a557fdce314e27ee4eabb25fa5e5fe",
			)
		];

		for vector in &vectors {
			let entropy = Vec::<u8>::from_hex(&vector.0).unwrap();
			let mnemonic_str = vector.1;
			let passphrase = vector.2;
			let seed = Vec::<u8>::from_hex(&vector.3).unwrap();

			let mnemonic = Mnemonic::from_entropy_in(Language::Japanese, &entropy).unwrap();

			assert_eq!(seed, &mnemonic.to_seed(passphrase)[..], "failed vector: {}", mnemonic_str);
			let rt = Mnemonic::parse_in(Language::Japanese, mnemonic.to_string())
				.expect(&format!("vector: {}", mnemonic_str));
			assert_eq!(seed, &rt.to_seed(passphrase)[..]);

			let mnemonic = Mnemonic::parse_in(Language::Japanese, mnemonic_str)
				.expect(&format!("vector: {}", mnemonic_str));
			assert_eq!(seed, &mnemonic.to_seed(passphrase)[..], "failed vector: {}", mnemonic_str);
		}
	}
}
