// SPDX-License-Identifier: Apache-2.0

use crate::hal::ui::UserAbort;

use alloc::string::String;
use alloc::string::ToString;

pub async fn show_and_confirm_mnemonic(
    _ui: &mut impl crate::hal::Ui,
    _random: &mut impl crate::hal::Random,
    words: &[&str],
) -> Result<(), UserAbort> {
    for word in words.iter() {
        bitbox02::println_stdout(word);
    }
    bitbox02::println_stdout("Words confirmed");

    Ok(())
}

pub async fn get(_ui: &mut impl crate::hal::Ui) -> Result<zeroize::Zeroizing<String>, UserAbort> {
    let words = "boring mistake dish oyster truth pigeon viable emerge sort crash wire portion cannon couple enact box walk height pull today solid off enable tide";
    bitbox02::println_stdout("Restored from recovery words below:");
    bitbox02::println_stdout(words);

    Ok(zeroize::Zeroizing::new(words.to_string()))
}
