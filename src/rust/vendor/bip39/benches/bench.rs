#![feature(test)]

extern crate bip39;
extern crate test;

use test::Bencher;

use bip39::*;

#[cfg(not(any(
	feature = "chinese-simplified",
	feature = "chinese-traditional",
	feature = "czech",
	feature = "french",
	feature = "italian",
	feature = "japanese",
	feature = "korean",
	feature = "portuguese",
	feature = "spanish"
)))]
const LANG: Language = Language::English;
#[cfg(feature = "chinese-simplified")]
const LANG: Language = Language::SimplifiedChinese;
#[cfg(feature = "chinese-traditional")]
const LANG: Language = Language::TraditionalChinese;
#[cfg(feature = "czech")]
const LANG: Language = Language::Czech;
#[cfg(feature = "french")]
const LANG: Language = Language::French;
#[cfg(feature = "italian")]
const LANG: Language = Language::Italian;
#[cfg(feature = "japanese")]
const LANG: Language = Language::Japanese;
#[cfg(feature = "korean")]
const LANG: Language = Language::Korean;
#[cfg(feature = "portuguese")]
const LANG: Language = Language::Portuguese;
#[cfg(feature = "spanish")]
const LANG: Language = Language::Spanish;

#[bench]
fn validate(b: &mut Bencher) {
	let entropy = "7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f".as_bytes();
	let mnemonic = Mnemonic::from_entropy_in(LANG, &entropy).unwrap();
	assert_eq!(mnemonic.word_count(), 24);
	let phrase = mnemonic.to_string();

	b.iter(|| {
		let _ = Mnemonic::parse_in(Language::English, &phrase);
	});
}

#[bench]
fn from_entropy(b: &mut Bencher) {
	let entropy = "7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f".as_bytes();

	b.iter(|| {
		let _ = Mnemonic::from_entropy_in(LANG, &entropy).unwrap();
	});
}

#[bench]
fn new_mnemonic(b: &mut Bencher) {
	b.iter(|| {
		let _ = Mnemonic::generate_in(LANG, 24);
	});
}

#[bench]
fn to_seed(b: &mut Bencher) {
	let entropy = "7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f".as_bytes();
	let m = Mnemonic::from_entropy_in(LANG, &entropy).unwrap();

	b.iter(|| {
		let _ = m.to_seed("");
	});
}
