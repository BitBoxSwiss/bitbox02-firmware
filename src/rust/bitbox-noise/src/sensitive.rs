// SPDX-License-Identifier: Apache-2.0

// Derived from noise-rust-crypto 0.6.2, originally licensed under the Unlicense.
// https://github.com/sopium/noise-rust/blob/36bc85bd05db04b0d469843275ccafd848cddb99/noise-rust-crypto/src/sensitive.rs

use noise_protocol::U8Array;
use zeroize::{Zeroize, Zeroizing};

/// Struct holding a value that is safely zeroed on drop.
pub struct Sensitive<A: U8Array + Zeroize>(Zeroizing<A>);

impl<A: U8Array + Zeroize> Sensitive<A> {
    pub fn from(a: Zeroizing<A>) -> Self {
        Sensitive(a)
    }
}

impl<A: U8Array + Zeroize> core::ops::Deref for Sensitive<A> {
    type Target = A;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<A: U8Array + Zeroize> core::ops::DerefMut for Sensitive<A> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<A: U8Array + Zeroize> U8Array for Sensitive<A> {
    fn new() -> Self {
        Sensitive::from(Zeroizing::new(A::new()))
    }

    fn new_with(v: u8) -> Self {
        Sensitive::from(Zeroizing::new(A::new_with(v)))
    }

    fn from_slice(s: &[u8]) -> Self {
        Sensitive::from(Zeroizing::new(A::from_slice(s)))
    }

    fn len() -> usize {
        A::len()
    }

    fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }

    fn as_mut(&mut self) -> &mut [u8] {
        self.0.as_mut()
    }
}
