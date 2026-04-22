// SPDX-License-Identifier: Apache-2.0

use alloc::boxed::Box;

use bitbox_hal::{Hal, Random, SecureChip, securechip};
use digest::FixedOutput;
use sha2::Digest;

pub fn random_32_bytes_with_mixin(
    hal_random: &mut impl Random,
    mixin: &[u8; 32],
) -> Box<zeroize::Zeroizing<[u8; 32]>> {
    let mut mixed = zeroize::Zeroizing::new([0u8; 32]);
    hal_random.mcu_32_bytes(&mut mixed);

    for (byte, mixin_byte) in mixed.iter_mut().zip(mixin.iter()) {
        *byte ^= *mixin_byte;
    }

    let factory_randomness = hal_random.factory_randomness();
    for (byte, factory_randomness_byte) in mixed.iter_mut().zip(factory_randomness.iter()) {
        *byte ^= *factory_randomness_byte;
    }

    let mut result = Box::new(zeroize::Zeroizing::new([0u8; 32]));
    let mut hasher = sha2::Sha256::new();
    hasher.update(mixed.as_slice());
    FixedOutput::finalize_into(hasher, result.as_mut_slice().into());
    result
}

pub async fn random_32_bytes(
    hal_random: &mut impl Random,
    hal_securechip: &mut impl SecureChip,
) -> Result<Box<zeroize::Zeroizing<[u8; 32]>>, securechip::Error> {
    let securechip_random = hal_securechip.random().await?;
    Ok(random_32_bytes_with_mixin(
        hal_random,
        securechip_random.as_ref(),
    ))
}

pub async fn random_32_bytes_from_hal(
    hal: &mut impl Hal,
) -> Result<Box<zeroize::Zeroizing<[u8; 32]>>, securechip::Error> {
    let bitbox_hal::HalSubsystems {
        random, securechip, ..
    } = hal.as_mut();
    random_32_bytes(random, securechip).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitbox_platform_host::random::TestingRandom;
    use bitbox_platform_host::securechip::FakeSecureChip;
    use hex_lit::hex;

    #[test]
    fn test_random_32_bytes_with_mixin() {
        let mut hal_random = TestingRandom::new();
        let mcu_random = hex!("00112233445566778899aabbccddeefffedcba98765432100123456789abcdef");
        let mixin = hex!("102030405060708090a0b0c0d0e0f0000f1e2d3c4b5a69788796a5b4c3d2e1f0");
        let factory_randomness = TestingRandom::FACTORY_RANDOMNESS;

        hal_random.mock_next(mcu_random);
        assert_eq!(hal_random.factory_randomness(), &factory_randomness);

        let result = random_32_bytes_with_mixin(&mut hal_random, &mixin);

        let expected = hex!("843595519af3ac2a92cbe2be42a77d5297f64a1c98c1edbc27e1fc661f1d4ac8");
        assert_eq!(result.as_slice(), &expected);
    }

    #[async_test::test]
    async fn test_random_32_bytes() {
        let mut hal_random = TestingRandom::new();
        let mut hal_securechip = FakeSecureChip::new();
        let mcu_random = hex!("00112233445566778899aabbccddeefffedcba98765432100123456789abcdef");
        let securechip_random =
            hex!("102030405060708090a0b0c0d0e0f0000f1e2d3c4b5a69788796a5b4c3d2e1f0");
        let factory_randomness = TestingRandom::FACTORY_RANDOMNESS;

        hal_random.mock_next(mcu_random);
        hal_securechip.mock_random(securechip_random);
        assert_eq!(hal_random.factory_randomness(), &factory_randomness);

        let result = random_32_bytes(&mut hal_random, &mut hal_securechip)
            .await
            .unwrap();

        /* Reproduce expected with Python:
        import hashlib
        mcu_random = bytes.fromhex(
            "00112233445566778899aabbccddeefffedcba98765432100123456789abcdef"
        )
        securechip_random = bytes.fromhex(
            "102030405060708090a0b0c0d0e0f0000f1e2d3c4b5a69788796a5b4c3d2e1f0"
        )
        factory_randomness = bytes.fromhex(
            "f71df5932e61dbaab9b9eca90e59c4b45ec91fadf803db15578c260c608eb46b"
        )
        mixed = bytes(
            m ^ s ^ f
            for m, s, f in zip(mcu_random, securechip_random, factory_randomness)
        )
        print(hashlib.sha256(mixed).hexdigest())
        */
        let expected = hex!("843595519af3ac2a92cbe2be42a77d5297f64a1c98c1edbc27e1fc661f1d4ac8");
        assert_eq!(result.as_slice(), &expected);
    }
}
