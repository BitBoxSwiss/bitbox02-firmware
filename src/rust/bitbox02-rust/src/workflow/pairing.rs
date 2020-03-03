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

use arrayvec::ArrayString;
use binascii;
use bitbox02;
use core::fmt::Write;

/// This function takes a slice of bytes that must be at most 60 characters when base32
/// encoded.
pub fn create(bytes: &[u8]) -> bool {
    let mut encoded = [0u8; 60];
    binascii::b32encode(bytes, &mut encoded).unwrap_or_else(|_| panic!("Too short buffer"));

    // Base32 contains only utf-8 valid chars. unwrap is safe
    let encoded = core::str::from_utf8(&encoded).expect("invalid utf-8");
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

    bitbox02::workflow_confirm_blocking(
        "Base pairing code",
        &formatted,
        unsafe { &bitbox02::font_monogram_5X9 },
        false,
        false,
    )
}

pub fn extra_hash_create(bytes: &[u8]) -> bool {
    let mut buf = [0u8; 32];
    bitbox02::sha256(bytes, &mut buf).expect("sha256 failed");
    create(&buf[..])
}
