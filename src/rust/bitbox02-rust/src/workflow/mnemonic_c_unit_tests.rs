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

pub async fn show_and_confirm_mnemonic(
    _hal: &mut impl crate::hal::Hal,
    words: &[&str],
) -> Result<(), CancelError> {
    for word in words.iter() {
        bitbox02::println_stdout(word);
    }
    bitbox02::println_stdout("Words confirmed");

    Ok(())
}

pub async fn get(
    _hal: &mut impl crate::hal::Hal,
) -> Result<zeroize::Zeroizing<String>, CancelError> {
    let words = "boring mistake dish oyster truth pigeon viable emerge sort crash wire portion cannon couple enact box walk height pull today solid off enable tide";
    bitbox02::println_stdout("Restored from recovery words below:");
    bitbox02::println_stdout(words);

    Ok(zeroize::Zeroizing::new(words.to_string()))
}
