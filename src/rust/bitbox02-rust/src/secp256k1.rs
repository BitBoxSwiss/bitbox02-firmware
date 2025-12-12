// SPDX-License-Identifier: Apache-2.0

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
