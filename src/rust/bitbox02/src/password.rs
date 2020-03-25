// Copyright 2020 Shift Cryptosecurity AG
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

/// C-style including null terminator, as it is used in C only so far.
/// 150 corresponds to SET_PASSWORD_MAX_PASSWORD_LENGTH.
/// Does *not* implement Copy, so that we can have a Drop to zero the contents.
// TODO: use a reusable zero-on-drop buffer type
pub struct Password([u8; 150]);

impl Password {
    /// Makes a password buffer filled with 0.
    pub fn new() -> Password {
        Password([0; 150])
    }

    /// Clones the password bytes from `source` without additional allocations.
    pub fn clone_from(&mut self, source: &Self) {
        self.0.copy_from_slice(&source.0[..]);
    }
}

impl AsRef<[u8; 150]> for Password {
    fn as_ref(&self) -> &[u8; 150] {
        &self.0
    }
}

impl AsMut<[u8; 150]> for Password {
    fn as_mut(&mut self) -> &mut [u8; 150] {
        &mut self.0
    }
}

impl Drop for Password {
    fn drop(&mut self) {
        util::zero(&mut self.0[..]);
        crate::screen_print_debug(".", 100);
    }
}
