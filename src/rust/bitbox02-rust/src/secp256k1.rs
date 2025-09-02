// Copyright 2025 Shift Crypto AG
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

use bitcoin::secp256k1::{All, Secp256k1};
use core::cell::OnceCell;
use core::ops::Deref;

#[derive(Debug, Copy, Clone)]
pub struct GlobalContext {
    __private: (), // prevents direct init
}

/// Global context, initialized once.
///
/// Port of https://docs.rs/secp256k1/latest/secp256k1/global/struct.GlobalContext.html to no_std.
pub static SECP256K1: &GlobalContext = &GlobalContext { __private: () };

struct SyncWrapper(OnceCell<Secp256k1<All>>);

// SAFETY: Embedded single-threaded use only, can't use from an interrupt context.
unsafe impl Sync for SyncWrapper {}

impl Deref for GlobalContext {
    type Target = Secp256k1<All>;

    fn deref(&self) -> &Self::Target {
        static CONTEXT: SyncWrapper = SyncWrapper(OnceCell::new());

        CONTEXT.0.get_or_init(|| {
            // Initialized on first access
            Secp256k1::new()
        })
    }
}
