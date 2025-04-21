// Copyright 2019 Shift Cryptosecurity AG
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

use crate::hal::Ui;
use crate::workflow::confirm;
pub use confirm::UserAbort;

use alloc::string::String;

/// Format a pairing hash to a format that is easy for humans to visually compare.
pub fn format_hash(hash: &[u8; 32]) -> String {
    let mut encoded = [0u8; 60];
    let encoded = binascii::b32encode(&hash[..], &mut encoded).unwrap();
    // Base32 contains only utf-8 valid chars.
    let encoded = unsafe { core::str::from_utf8_unchecked(encoded) };
    format!(
        "{} {}\n{} {}",
        &encoded[0..5],
        &encoded[5..10],
        &encoded[10..15],
        &encoded[15..20]
    )
}

pub async fn confirm(hal: &mut impl crate::hal::Hal, hash: &[u8; 32]) -> Result<(), UserAbort> {
    let params = confirm::Params {
        title: "Pairing code",
        body: &format_hash(hash),
        font: confirm::Font::Monogram5X9,
        ..Default::default()
    };

    hal.ui().confirm(&params).await
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::bb02_async::block_on;
    use crate::hal::testing::TestingHal;
    use crate::workflow::testing::Screen;

    use alloc::boxed::Box;

    #[test]
    fn test_confirm() {
        let mut mock_hal = TestingHal::new();

        assert!(block_on(confirm(
            &mut mock_hal,
            b"\x59\x28\x9b\xdb\xbb\xb6\xb6\x8e\x8f\x12\x7f\x49\xa5\x25\xb0\x30\x13\x50\x0b\x3c\x1a\xf2\x62\x6f\x40\x07\xeb\xe4\x4f\x09\xc8\x6b")).is_ok());

        assert_eq!(
            mock_hal.ui.screens,
            vec![Screen::Confirm {
                title: "Pairing code".into(),
                body: "LEUJX W53W2\n3I5DY SP5E2".into(),
                longtouch: false,
            },]
        );
    }
}
