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

use crate::workflow::confirm;
pub use confirm::UserAbort;

use arrayvec::ArrayString;
use core::fmt::Write;

pub async fn confirm(hash: &[u8; 32]) -> Result<(), UserAbort> {
    let mut encoded = [0u8; 60];
    let encoded = binascii::b32encode(&hash[..], &mut encoded).unwrap();

    // Base32 contains only utf-8 valid chars. unwrap is safe
    let encoded = core::str::from_utf8(encoded).expect("invalid utf-8");
    let mut formatted = ArrayString::<[_; 23]>::new();

    write!(
        formatted,
        "{} {}\n{} {}",
        &encoded[0..5],
        &encoded[5..10],
        &encoded[10..15],
        &encoded[15..20]
    )
    .expect("failed to format");

    let params = confirm::Params {
        title: "Pairing code",
        body: &formatted,
        font: confirm::Font::Monogram5X9,
        ..Default::default()
    };

    confirm::confirm(&params).await
}
