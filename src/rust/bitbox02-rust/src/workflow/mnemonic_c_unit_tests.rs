// Copyright 2024 Shift Crypto AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub use super::cancel::Error as CancelError;

use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;

pub async fn show_and_confirm_mnemonic(words: &[&str]) -> Result<(), CancelError> {
    for word in words.iter() {
        bitbox02::print_stdout(&format!("{} ", word));
    }
    bitbox02::println_stdout("\nWords confirmed");

    Ok(())
}

pub async fn get() -> Result<zeroize::Zeroizing<String>, CancelError> {
    let words = "boring mistake dish oyster truth pigeon viable emerge sort crash wire portion cannon couple enact box walk height pull today solid off enable tide";
    bitbox02::println_stdout("Restored from recovery words below:");
    bitbox02::println_stdout(words);

    Ok(zeroize::Zeroizing::new(words.to_string()))
}

pub async fn get_shamir() -> Result<Vec<zeroize::Zeroizing<String>>, CancelError> {
    let mnemonics:Vec<&str> = [
        "abandon abandon able grab timber satoshi effort humble educate renew measure door hero mandate charge length win deny weather decline hawk increase youth wash lava flee fit",
        "abandon abandon above ozone wheat very slow clever fish quit park cat heavy anger smoke guard estate season puppy dynamic hospital melt release enact promote alien toilet",
        // two shards are enough
        // "abandon abandon absorb sight abandon crop reopen oak flame youth rhythm world grape jelly river pitch stamp heart glory muffin spend pencil mind casino toddler slow idea",

        // 15 words options for the main mnemonic "spawn nest ability mammal beyond stay wish dragon retreat calm index trap"
        // "abandon abandon about script abandon horror dynamic sight pond invest toddler hidden north awful tattoo",
        // "abandon abandon absent genius hello sting tuna uncover remain trial evil dynamic distance license upper",
        // "abandon abandon abstract ability duck return physical august roof gospel click garden trophy injury wisdom"
    ].to_vec();
    let mut result: Vec<zeroize::Zeroizing<String>> = Vec::new();
    bitbox02::println_stdout("Restored from recovery words below:");
    for mnemonic in mnemonics {
        bitbox02::println_stdout(mnemonic);
        result.push(zeroize::Zeroizing::new(mnemonic.to_string()));
    }

    Ok(result)
}
