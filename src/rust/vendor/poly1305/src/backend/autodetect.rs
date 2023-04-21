//! Autodetection support for AVX2 CPU intrinsics on x86 CPUs, with fallback
//! to the "soft" backend when it's unavailable.

use crate::{backend, Block, Key, Tag};
use core::mem::ManuallyDrop;

cpufeatures::new!(avx2_cpuid, "avx2");

pub struct State {
    inner: Inner,
    token: avx2_cpuid::InitToken,
}

union Inner {
    avx2: ManuallyDrop<backend::avx2::State>,
    soft: ManuallyDrop<backend::soft::State>,
}

impl State {
    /// Initialize Poly1305 [`State`] with the given key
    #[inline]
    pub(crate) fn new(key: &Key) -> State {
        let (token, avx2_present) = avx2_cpuid::init_get();

        let inner = if avx2_present {
            Inner {
                avx2: ManuallyDrop::new(backend::avx2::State::new(key)),
            }
        } else {
            Inner {
                soft: ManuallyDrop::new(backend::soft::State::new(key)),
            }
        };

        Self { inner, token }
    }

    /// Reset internal state
    #[inline]
    pub(crate) fn reset(&mut self) {
        if self.token.get() {
            unsafe { (*self.inner.avx2).reset() }
        } else {
            unsafe { (*self.inner.soft).reset() }
        }
    }

    /// Compute a Poly1305 block
    #[inline]
    pub(crate) fn compute_block(&mut self, block: &Block, partial: bool) {
        if self.token.get() {
            unsafe { (*self.inner.avx2).compute_block(block, partial) }
        } else {
            unsafe { (*self.inner.soft).compute_block(block, partial) }
        }
    }

    /// Finalize output producing a [`Tag`]
    #[inline]
    pub(crate) fn finalize(&mut self) -> Tag {
        if self.token.get() {
            unsafe { (*self.inner.avx2).finalize() }
        } else {
            unsafe { (*self.inner.soft).finalize() }
        }
    }
}

impl Clone for State {
    fn clone(&self) -> Self {
        let inner = if self.token.get() {
            Inner {
                avx2: ManuallyDrop::new(unsafe { (*self.inner.avx2).clone() }),
            }
        } else {
            Inner {
                soft: ManuallyDrop::new(unsafe { (*self.inner.soft).clone() }),
            }
        };

        Self {
            inner,
            token: self.token,
        }
    }
}

#[cfg(feature = "zeroize")]
impl Drop for State {
    fn drop(&mut self) {
        use zeroize::Zeroize;
        const SIZE: usize = core::mem::size_of::<State>();
        let state = unsafe { &mut *(self as *mut State as *mut [u8; SIZE]) };
        state.zeroize();
    }
}
