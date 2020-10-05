// Copyright 2020 Shift Crypto AG
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

extern crate alloc;
use alloc::vec::Vec;

use super::confirm;

use util::ascii;

pub enum Error {
    InvalidInput,
    UserAbort,
}

/// Verify a message to be signed.
///
/// If the bytes are all printable ascii chars, the message is
/// confirmed one line at a time (the str is split into lines).
///
/// Otherwise, it is displayed as hex.
pub async fn verify(msg: &[u8]) -> Result<(), Error> {
    if ascii::is_printable_ascii(&msg, ascii::Charset::AllNewline) {
        // The message is all ascii and printable.
        let msg = core::str::from_utf8(msg).unwrap();

        let pages: Vec<&str> = msg.split('\n').filter(|line| !line.is_empty()).collect();
        if pages.is_empty() {
            return Err(Error::InvalidInput);
        }
        for (i, &page) in pages.iter().enumerate() {
            let is_last = i == pages.len() - 1;
            let title = if pages.len() == 1 {
                "Sign message".into()
            } else {
                format!("Sign {}/{}", i + 1, pages.len())
            };
            let params = confirm::Params {
                title: &title,
                body: page,
                scrollable: true,
                accept_is_nextarrow: !is_last,
                longtouch: is_last,
                ..Default::default()
            };
            if !confirm::confirm(&params).await {
                return Err(Error::UserAbort);
            }
        }
        Ok(())
    } else {
        let params = confirm::Params {
            title: "Sign message\ndata (hex)",
            body: &hex::encode(msg),
            scrollable: true,
            display_size: msg.len(),
            longtouch: true,
            ..Default::default()
        };
        if confirm::confirm(&params).await {
            Ok(())
        } else {
            Err(Error::UserAbort)
        }
    }
}
