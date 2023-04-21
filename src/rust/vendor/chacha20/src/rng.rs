//! Block RNG based on rand_core::BlockRng

use rand_core::{
    block::{BlockRng, BlockRngCore},
    CryptoRng, Error, RngCore, SeedableRng,
};

use crate::{
    backend::{Core, BUFFER_SIZE},
    rounds::{R12, R20, R8},
    KEY_SIZE,
};
use core::convert::TryInto;

/// Array wrapper used for `BlockRngCore::Results` associated types.
#[repr(transparent)]
pub struct BlockRngResults([u32; BUFFER_SIZE / 4]);

impl Default for BlockRngResults {
    fn default() -> Self {
        BlockRngResults([u32::default(); BUFFER_SIZE / 4])
    }
}

impl AsRef<[u32]> for BlockRngResults {
    fn as_ref(&self) -> &[u32] {
        &self.0
    }
}

impl AsMut<[u32]> for BlockRngResults {
    fn as_mut(&mut self) -> &mut [u32] {
        &mut self.0
    }
}

macro_rules! impl_chacha_rng {
    ($name:ident, $core:ident, $rounds:ident, $doc:expr) => {
        #[doc = $doc]
        #[cfg_attr(docsrs, doc(cfg(feature = "rng")))]
        pub struct $name(BlockRng<$core>);

        impl SeedableRng for $name {
            type Seed = [u8; KEY_SIZE];

            #[inline]
            fn from_seed(seed: Self::Seed) -> Self {
                let core = $core::from_seed(seed);
                Self(BlockRng::new(core))
            }
        }

        impl RngCore for $name {
            #[inline]
            fn next_u32(&mut self) -> u32 {
                self.0.next_u32()
            }

            #[inline]
            fn next_u64(&mut self) -> u64 {
                self.0.next_u64()
            }

            #[inline]
            fn fill_bytes(&mut self, bytes: &mut [u8]) {
                self.0.fill_bytes(bytes)
            }

            #[inline]
            fn try_fill_bytes(&mut self, bytes: &mut [u8]) -> Result<(), Error> {
                self.0.try_fill_bytes(bytes)
            }
        }

        impl CryptoRng for $name {}

        #[doc = "Core random number generator, for use with [`rand_core::block::BlockRng`]"]
        #[cfg_attr(docsrs, doc(cfg(feature = "rng")))]
        pub struct $core {
            block: Core<$rounds>,
            counter: u64,
        }

        impl SeedableRng for $core {
            type Seed = [u8; KEY_SIZE];

            #[inline]
            fn from_seed(seed: Self::Seed) -> Self {
                let block = Core::new(&seed, Default::default());
                Self { block, counter: 0 }
            }
        }

        impl BlockRngCore for $core {
            type Item = u32;
            type Results = BlockRngResults;

            fn generate(&mut self, results: &mut Self::Results) {
                // is this necessary?
                assert!(
                    self.counter < u32::MAX as u64,
                    "maximum number of allowed ChaCha blocks exceeded"
                );

                let mut buffer = [0u8; BUFFER_SIZE];
                self.block.generate(self.counter, &mut buffer);

                for (n, chunk) in results.as_mut().iter_mut().zip(buffer.chunks_exact(4)) {
                    *n = u32::from_le_bytes(chunk.try_into().unwrap());
                }

                self.counter += 1;
            }
        }

        impl CryptoRng for $core {}
    };
}

impl_chacha_rng!(
    ChaCha8Rng,
    ChaCha8RngCore,
    R8,
    "Random number generator over the ChaCha8 stream cipher."
);

impl_chacha_rng!(
    ChaCha12Rng,
    ChaCha12RngCore,
    R12,
    "Random number generator over the ChaCha12 stream cipher."
);

impl_chacha_rng!(
    ChaCha20Rng,
    ChaCha20RngCore,
    R20,
    "Random number generator over the ChaCha20 stream cipher."
);

#[cfg(test)]
mod tests {
    use super::ChaCha20Rng;
    use crate::KEY_SIZE;
    use rand_core::{RngCore, SeedableRng};

    const KEY: [u8; KEY_SIZE] = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32,
    ];

    #[test]
    fn test_rng_output() {
        let mut rng = ChaCha20Rng::from_seed(KEY);
        let mut bytes = [0u8; 13];

        rng.fill_bytes(&mut bytes);
        assert_eq!(
            bytes,
            [177, 105, 126, 159, 198, 70, 30, 25, 131, 209, 49, 207, 105]
        );

        rng.fill_bytes(&mut bytes);
        assert_eq!(
            bytes,
            [167, 163, 252, 19, 79, 20, 152, 128, 232, 187, 43, 93, 35]
        );
    }
}
