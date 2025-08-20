use core::fmt;

#[cfg(feature = "chinese-simplified")]
mod chinese_simplified;
#[cfg(feature = "chinese-traditional")]
mod chinese_traditional;
#[cfg(feature = "czech")]
mod czech;
mod english;
#[cfg(feature = "french")]
mod french;
#[cfg(feature = "italian")]
mod italian;
#[cfg(feature = "japanese")]
mod japanese;
#[cfg(feature = "korean")]
mod korean;
#[cfg(feature = "portuguese")]
mod portuguese;
#[cfg(feature = "spanish")]
mod spanish;

/// The maximum number of languages enabled.
pub(crate) const MAX_NB_LANGUAGES: usize = 10;

/// Language to be used for the mnemonic phrase.
///
/// The English language is always available, other languages are enabled using
/// the compilation features.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Language {
	/// The English language.
	English,
	#[cfg(feature = "chinese-simplified")]
	/// The Simplified Chinese language.
	SimplifiedChinese,
	#[cfg(feature = "chinese-traditional")]
	/// The Traditional Chinese language.
	TraditionalChinese,
	#[cfg(feature = "czech")]
	/// The Czech language.
	Czech,
	#[cfg(feature = "french")]
	/// The French language.
	French,
	#[cfg(feature = "italian")]
	/// The Italian language.
	Italian,
	#[cfg(feature = "japanese")]
	/// The Japanese language.
	Japanese,
	#[cfg(feature = "korean")]
	/// The Korean language.
	Korean,
	#[cfg(feature = "portuguese")]
	/// The Portuguese language.
	Portuguese,
	#[cfg(feature = "spanish")]
	/// The Spanish language.
	Spanish,
}

impl Default for Language {
	fn default() -> Self {
		Language::English
	}
}

impl Language {
	/// The list of supported languages.
	/// Language support is managed by compile features.
	pub const ALL: &'static [Language] = &[
		Language::English,
		#[cfg(feature = "chinese-simplified")]
		Language::SimplifiedChinese,
		#[cfg(feature = "chinese-traditional")]
		Language::TraditionalChinese,
		#[cfg(feature = "czech")]
		Language::Czech,
		#[cfg(feature = "french")]
		Language::French,
		#[cfg(feature = "italian")]
		Language::Italian,
		#[cfg(feature = "japanese")]
		Language::Japanese,
		#[cfg(feature = "korean")]
		Language::Korean,
		#[cfg(feature = "portuguese")]
		Language::Portuguese,
		#[cfg(feature = "spanish")]
		Language::Spanish,
	];

	/// The list of supported languages.
	/// Language support is managed by compile features.
	#[deprecated(since = "2.1.0", note = "use constant Language::ALL instead")]
	pub fn all() -> &'static [Language] {
		Language::ALL
	}

	/// The word list for this language.
	#[inline]
	pub fn word_list(self) -> &'static [&'static str; 2048] {
		match self {
			Language::English => &english::WORDS,
			#[cfg(feature = "chinese-simplified")]
			Language::SimplifiedChinese => &chinese_simplified::WORDS,
			#[cfg(feature = "chinese-traditional")]
			Language::TraditionalChinese => &chinese_traditional::WORDS,
			#[cfg(feature = "czech")]
			Language::Czech => &czech::WORDS,
			#[cfg(feature = "french")]
			Language::French => &french::WORDS,
			#[cfg(feature = "italian")]
			Language::Italian => &italian::WORDS,
			#[cfg(feature = "japanese")]
			Language::Japanese => &japanese::WORDS,
			#[cfg(feature = "korean")]
			Language::Korean => &korean::WORDS,
			#[cfg(feature = "portuguese")]
			Language::Portuguese => &portuguese::WORDS,
			#[cfg(feature = "spanish")]
			Language::Spanish => &spanish::WORDS,
		}
	}

	/// Returns true if all words in the list are guaranteed to
	/// only be in this list and not in any other.
	#[inline]
	pub(crate) fn unique_words(self) -> bool {
		match self {
			Language::English => false,
			#[cfg(feature = "chinese-simplified")]
			Language::SimplifiedChinese => false,
			#[cfg(feature = "chinese-traditional")]
			Language::TraditionalChinese => false,
			#[cfg(feature = "czech")]
			Language::Czech => true,
			#[cfg(feature = "french")]
			Language::French => false,
			#[cfg(feature = "italian")]
			Language::Italian => true,
			#[cfg(feature = "japanese")]
			Language::Japanese => true,
			#[cfg(feature = "korean")]
			Language::Korean => true,
			#[cfg(feature = "portuguese")]
			Language::Portuguese => true,
			#[cfg(feature = "spanish")]
			Language::Spanish => true,
		}
	}

	/// Get words from the word list that start with the given prefix.
	pub fn words_by_prefix(self, prefix: &str) -> &[&'static str] {
		// The words in the word list are ordered lexicographically. This means
		// that we cannot use `binary_search` to find words more efficiently,
		// because the Rust ordering is based on the byte values. However, it
		// does mean that words that share a prefix will follow each other.

		let first = match self.word_list().iter().position(|w| w.starts_with(prefix)) {
			Some(i) => i,
			None => return &[],
		};
		let count = self.word_list()[first..].iter().take_while(|w| w.starts_with(prefix)).count();
		&self.word_list()[first..first + count]
	}

	/// Get the index of the word in the word list.
	#[inline]
	pub fn find_word(self, word: &str) -> Option<u16> {
		match self {
			// English, Portuguese, Italian, and Korean wordlists are already lexicographically
			// sorted, so they are candidates for optimization via binary_search
			Self::English => self.word_list().binary_search(&word).map(|x| x as _).ok(),
			#[cfg(feature = "portuguese")]
			Self::Portuguese => self.word_list().binary_search(&word).map(|x| x as _).ok(),
			#[cfg(feature = "italian")]
			Self::Italian => self.word_list().binary_search(&word).map(|x| x as _).ok(),
			#[cfg(feature = "korean")]
			Self::Korean => self.word_list().binary_search(&word).map(|x| x as _).ok(),

			// All other languages' wordlists are not lexicographically sorted, so we have to
			// resort to linear search
			#[cfg(any(
				feature = "chinese-simplified",
				feature = "chinese-traditional",
				feature = "czech",
				feature = "french",
				feature = "japanese",
				feature = "spanish",
			))]
			_ => self.word_list().iter().position(|w| *w == word).map(|i| i as u16),
		}
	}
}

impl fmt::Display for Language {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::Debug::fmt(self, f)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[cfg(all(
		feature = "chinese-simplified",
		feature = "chinese-traditional",
		feature = "czech",
		feature = "french",
		feature = "italian",
		feature = "japanese",
		feature = "korean",
		feature = "portuguese",
		feature = "spanish"
	))]
	#[test]
	fn validate_word_list_checksums() {
		//! In this test, we ensure that the word lists are identical.
		//!
		//! They are as follows in the bips repository:
		//! 5c5942792bd8340cb8b27cd592f1015edf56a8c5b26276ee18a482428e7c5726  chinese_simplified.txt
		//! 417b26b3d8500a4ae3d59717d7011952db6fc2fb84b807f3f94ac734e89c1b5f  chinese_traditional.txt
		//! 7e80e161c3e93d9554c2efb78d4e3cebf8fc727e9c52e03b83b94406bdcc95fc  czech.txt
		//! 2f5eed53a4727b4bf8880d8f3f199efc90e58503646d9ff8eff3a2ed3b24dbda  english.txt
		//! ebc3959ab7801a1df6bac4fa7d970652f1df76b683cd2f4003c941c63d517e59  french.txt
		//! d392c49fdb700a24cd1fceb237c1f65dcc128f6b34a8aacb58b59384b5c648c2  italian.txt
		//! 2eed0aef492291e061633d7ad8117f1a2b03eb80a29d0e4e3117ac2528d05ffd  japanese.txt
		//! 9e95f86c167de88f450f0aaf89e87f6624a57f973c67b516e338e8e8b8897f60  korean.txt
		//! 2685e9c194c82ae67e10ba59d9ea5345a23dc093e92276fc5361f6667d79cd3f  portuguese.txt
		//! 46846a5a0139d1e3cb77293e521c2865f7bcdb82c44e8d0a06a2cd0ecba48c0b  spanish.txt

		use bitcoin_hashes::{sha256, Hash, HashEngine};

		let checksums = [
			(
				"5c5942792bd8340cb8b27cd592f1015edf56a8c5b26276ee18a482428e7c5726",
				Language::SimplifiedChinese,
			),
			(
				"417b26b3d8500a4ae3d59717d7011952db6fc2fb84b807f3f94ac734e89c1b5f",
				Language::TraditionalChinese,
			),
			("7e80e161c3e93d9554c2efb78d4e3cebf8fc727e9c52e03b83b94406bdcc95fc", Language::Czech),
			("2f5eed53a4727b4bf8880d8f3f199efc90e58503646d9ff8eff3a2ed3b24dbda", Language::English),
			("ebc3959ab7801a1df6bac4fa7d970652f1df76b683cd2f4003c941c63d517e59", Language::French),
			("d392c49fdb700a24cd1fceb237c1f65dcc128f6b34a8aacb58b59384b5c648c2", Language::Italian),
			(
				"2eed0aef492291e061633d7ad8117f1a2b03eb80a29d0e4e3117ac2528d05ffd",
				Language::Japanese,
			),
			("9e95f86c167de88f450f0aaf89e87f6624a57f973c67b516e338e8e8b8897f60", Language::Korean),
			(
				"2685e9c194c82ae67e10ba59d9ea5345a23dc093e92276fc5361f6667d79cd3f",
				Language::Portuguese,
			),
			("46846a5a0139d1e3cb77293e521c2865f7bcdb82c44e8d0a06a2cd0ecba48c0b", Language::Spanish),
		];
		assert_eq!(MAX_NB_LANGUAGES, checksums.len());

		for &(_sum, lang) in &checksums {
			let mut digest = sha256::Hash::engine();
			for (_idx, word) in lang.word_list().iter().enumerate() {
				#[cfg(feature = "std")]
				assert!(::unicode_normalization::is_nfkd(&word));
				digest.input(word.as_bytes());
				digest.input("\n".as_bytes());
			}
			#[cfg(feature = "std")]
			assert_eq!(
				sha256::Hash::from_engine(digest).to_string(),
				_sum,
				"word list for language {} failed checksum check",
				lang,
			);
		}
	}

	#[test]
	fn words_by_prefix() {
		let lang = Language::English;

		let res = lang.words_by_prefix("woo");
		assert_eq!(res, ["wood", "wool"]);

		let res = lang.words_by_prefix("");
		assert_eq!(res.len(), 2048);

		let res = lang.words_by_prefix("woof");
		assert!(res.is_empty());
	}

	#[cfg(all(
		feature = "chinese-simplified",
		feature = "chinese-traditional",
		feature = "czech",
		feature = "french",
		feature = "italian",
		feature = "japanese",
		feature = "korean",
		feature = "portuguese",
		feature = "spanish"
	))]
	#[test]
	fn words_overlaps() {
		use std::collections::HashMap;

		// We keep a map of all words and the languages they occur in.
		// Afterwards, we make sure that no word maps to multiple languages
		// if either of those is guaranteed to have unique words.
		let mut words: HashMap<&str, Vec<Language>> = HashMap::new();
		for lang in Language::ALL.iter() {
			for word in lang.word_list().iter() {
				words.entry(word).or_insert(Vec::new()).push(*lang);
			}
		}

		let mut ok = true;
		for (word, langs) in words.into_iter() {
			if langs.len() == 1 {
				continue;
			}
			if langs.iter().any(|l| l.unique_words()) {
				println!("Word {} is not unique: {:?}", word, langs);
				ok = false;
			}
		}
		assert!(ok);
	}

	#[test]
	fn test_ordered_lists() {
		let languages = [
			Language::English,
			#[cfg(feature = "portuguese")]
			Language::Portuguese,
			#[cfg(feature = "italian")]
			Language::Italian,
			#[cfg(feature = "korean")]
			Language::Korean,
		];

		for lang in languages.iter() {
			let mut list = lang.word_list().to_vec();
			list.sort();
			assert_eq!(&list[..], &lang.word_list()[..]);
		}
	}

	/// Test the full round trip from index -> word-string -> index for all langauges
	mod round_trip {
		use super::*;

		#[test]
		fn english() {
			for i in 0..0x800 {
				let word_str = Language::English.word_list()[i];
				assert_eq!(Language::English.find_word(word_str), Some(i as _));
			}
		}

		#[cfg(feature = "chinese-simplified")]
		#[test]
		fn simplified_chinese() {
			for i in 0..0x800 {
				let word_str = Language::SimplifiedChinese.word_list()[i];
				assert_eq!(Language::SimplifiedChinese.find_word(word_str), Some(i as _));
			}
		}

		#[cfg(feature = "chinese-traditional")]
		#[test]
		fn traditional_chinese() {
			for i in 0..0x800 {
				let word_str = Language::TraditionalChinese.word_list()[i];
				assert_eq!(Language::TraditionalChinese.find_word(word_str), Some(i as _));
			}
		}

		#[cfg(feature = "czech")]
		#[test]
		fn czech() {
			for i in 0..0x800 {
				let word_str = Language::Czech.word_list()[i];
				assert_eq!(Language::Czech.find_word(word_str), Some(i as _));
			}
		}

		#[cfg(feature = "french")]
		#[test]
		fn french() {
			for i in 0..0x800 {
				let word_str = Language::French.word_list()[i];
				assert_eq!(Language::French.find_word(word_str), Some(i as _));
			}
		}

		#[cfg(feature = "italian")]
		#[test]
		fn italian() {
			for i in 0..0x800 {
				let word_str = Language::Italian.word_list()[i];
				assert_eq!(Language::Italian.find_word(word_str), Some(i as _));
			}
		}

		#[cfg(feature = "japanese")]
		#[test]
		fn japanese() {
			for i in 0..0x800 {
				let word_str = Language::Japanese.word_list()[i];
				assert_eq!(Language::Japanese.find_word(word_str), Some(i as _));
			}
		}

		#[cfg(feature = "korean")]
		#[test]
		fn korean() {
			for i in 0..0x800 {
				let word_str = Language::Korean.word_list()[i];
				assert_eq!(Language::Korean.find_word(word_str), Some(i as _));
			}
		}

		#[cfg(feature = "portuguese")]
		#[test]
		fn portuguese() {
			for i in 0..0x800 {
				let word_str = Language::Portuguese.word_list()[i];
				assert_eq!(Language::Portuguese.find_word(word_str), Some(i as _));
			}
		}

		#[cfg(feature = "spanish")]
		#[test]
		fn spanish() {
			for i in 0..0x800 {
				let word_str = Language::Spanish.word_list()[i];
				assert_eq!(Language::Spanish.find_word(word_str), Some(i as _));
			}
		}
	}
}
