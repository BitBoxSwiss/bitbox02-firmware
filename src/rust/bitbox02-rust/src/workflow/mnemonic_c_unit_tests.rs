// SPDX-License-Identifier: Apache-2.0

extern crate std;

use crate::hal::ui::UserAbort;

use alloc::string::String;
use alloc::string::ToString;

pub async fn show_and_confirm_mnemonic(
    _ui: &mut impl crate::hal::Ui,
    _random: &mut impl crate::hal::Random,
    words: &[&str],
) -> Result<(), UserAbort> {
    for word in words.iter() {
        std::println!("{}", word);
    }
    std::println!("Words confirmed");

    Ok(())
}

pub async fn get(_ui: &mut impl crate::hal::Ui) -> Result<zeroize::Zeroizing<String>, UserAbort> {
    let words = "boring mistake dish oyster truth pigeon viable emerge sort crash wire portion cannon couple enact box walk height pull today solid off enable tide";
    std::println!("Restored from recovery words below:");
    std::println!("{}", words);

    Ok(zeroize::Zeroizing::new(words.to_string()))
}
