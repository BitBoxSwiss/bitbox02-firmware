//! Autodetection support for AVX2 CPU and SSE2 intrinsics on x86 CPUs, with
//! fallback to a portable version when they're unavailable.

use super::{avx2, soft, sse2};
use crate::{rounds::Rounds, BLOCK_SIZE, IV_SIZE, KEY_SIZE};
use core::mem::ManuallyDrop;

/// Size of buffers passed to `generate` and `apply_keystream` for this
/// backend, which operates on two blocks in parallel for optimal performance.
/// The backend consumes four blocks at a time, so that the AVX2 implementation
/// can additionally pipeline the pairs of blocks for better ILP.
pub(crate) const BUFFER_SIZE: usize = BLOCK_SIZE * 4;

cpufeatures::new!(avx2_cpuid, "avx2");
cpufeatures::new!(sse2_cpuid, "sse2");

/// The ChaCha20 core function.
pub struct Core<R: Rounds> {
    inner: Inner<R>,
    avx2_token: avx2_cpuid::InitToken,
    sse2_token: sse2_cpuid::InitToken,
}

union Inner<R: Rounds> {
    avx2: ManuallyDrop<avx2::Core<R>>,
    sse2: ManuallyDrop<sse2::Core<R>>,
    soft: ManuallyDrop<soft::Core<R>>,
}

impl<R: Rounds> Core<R> {
    /// Initialize ChaCha core function with the given key size, IV, and
    /// number of rounds.
    ///
    /// Attempts to use AVX2 if present, followed by SSE2, with fallback to a
    /// portable software implementation if neither are available.
    #[inline]
    pub fn new(key: &[u8; KEY_SIZE], iv: [u8; IV_SIZE]) -> Self {
        let (avx2_token, avx2_present) = avx2_cpuid::init_get();
        let (sse2_token, sse2_present) = sse2_cpuid::init_get();

        let inner = if avx2_present {
            Inner {
                avx2: ManuallyDrop::new(avx2::Core::new(key, iv)),
            }
        } else if sse2_present {
            Inner {
                sse2: ManuallyDrop::new(sse2::Core::new(key, iv)),
            }
        } else {
            Inner {
                soft: ManuallyDrop::new(soft::Core::new(key, iv)),
            }
        };

        Self {
            inner,
            avx2_token,
            sse2_token,
        }
    }

    /// Generate output, overwriting data already in the buffer.
    #[inline]
    pub fn generate(&mut self, counter: u64, output: &mut [u8]) {
        if self.avx2_token.get() {
            unsafe { (*self.inner.avx2).generate(counter, output) }
        } else if self.sse2_token.get() {
            unsafe { (*self.inner.sse2).generate(counter, output) }
        } else {
            unsafe { (*self.inner.soft).generate(counter, output) }
        }
    }

    /// Apply generated keystream to the output buffer.
    #[inline]
    #[cfg(feature = "cipher")]
    pub fn apply_keystream(&mut self, counter: u64, output: &mut [u8]) {
        if self.avx2_token.get() {
            unsafe { (*self.inner.avx2).apply_keystream(counter, output) }
        } else if self.sse2_token.get() {
            unsafe { (*self.inner.sse2).apply_keystream(counter, output) }
        } else {
            unsafe { (*self.inner.soft).apply_keystream(counter, output) }
        }
    }
}

impl<R: Rounds> Clone for Core<R> {
    fn clone(&self) -> Self {
        let inner = if self.avx2_token.get() {
            Inner {
                avx2: ManuallyDrop::new(unsafe { (*self.inner.avx2).clone() }),
            }
        } else if self.sse2_token.get() {
            Inner {
                sse2: ManuallyDrop::new(unsafe { (*self.inner.sse2).clone() }),
            }
        } else {
            Inner {
                soft: ManuallyDrop::new(unsafe { (*self.inner.soft).clone() }),
            }
        };

        Self {
            inner,
            avx2_token: self.avx2_token,
            sse2_token: self.sse2_token,
        }
    }
}
