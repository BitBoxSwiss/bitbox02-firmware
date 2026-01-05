// SPDX-License-Identifier: Apache-2.0

pub use super::cancel::Error as CancelError;

use alloc::string::String;
use alloc::string::ToString;

pub async fn show_mnemonic(_words: &[&str]) -> Result<(), CancelError> {
    panic!("unused")
}

pub async fn confirm_word(_choices: &[&str], _title: &str) -> Result<u8, CancelError> {
    panic!("unused")
}

pub async fn show_and_confirm_mnemonic(
    _ui: &mut impl crate::hal::Ui,
    words: &[&str],
) -> Result<(), CancelError> {
    for word in words.iter() {
        bitbox02::println_stdout(word);
    }
    bitbox02::println_stdout("Words confirmed");

    Ok(())
}

pub async fn get(_ui: &mut impl crate::hal::Ui) -> Result<zeroize::Zeroizing<String>, CancelError> {
    let words = "boring mistake dish oyster truth pigeon viable emerge sort crash wire portion cannon couple enact box walk height pull today solid off enable tide";
    bitbox02::println_stdout("Restored from recovery words below:");
    bitbox02::println_stdout(words);

    Ok(zeroize::Zeroizing::new(words.to_string()))
}
