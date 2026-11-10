// SPDX-License-Identifier: Apache-2.0

use super::Error;
use crate::pb;

use crate::hal::Ui;
use crate::workflow::{password, unlock};

use crate::keystore;
use pb::response::Response;

/// Handles the SetPassword api call. This has the user enter a password twice and creates the
/// seed/keystore. After this call is finished, the keystore is fully unlocked.
///
/// `entropy` must be exactly 16 or 32 bytes and provides additional entropy used when creating the
/// seed. If 16 bytes are provided, the seed will also be 16 bytes long, corresponding to 12 BIP39
/// recovery words. If 32 bytes are provided, the seed will also be 32 bytes long, corresponding to
/// 24 BIP39 recovery words.
pub async fn process(
    hal: &mut impl crate::hal::Hal,
    pb::SetPasswordRequest { entropy }: &pb::SetPasswordRequest,
) -> Result<Response, Error> {
    if entropy.len() != 16 && entropy.len() != 32 {
        return Err(Error::InvalidInput);
    }
    let password = password::enter_twice(hal).await?;
    let unlock_animation = hal.ui().unlock_animation_create();
    if let Err(err) = keystore::create_and_store_seed(hal, &password, entropy).await {
        drop(unlock_animation);
        hal.ui()
            .status(&format!("Error\n{}", keystore::format_error(&err)), false)
            .await;
        return Err(Error::Generic);
    }
    let seed = keystore::copy_seed(hal).await?;
    unlock::unlock_bip39(hal, &seed, unlock_animation).await;
    Ok(Response::Success(pb::Success {}))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::hal::testing::TestingHal;

    use alloc::boxed::Box;
    use hex_lit::hex;

    #[async_test::test]
    async fn test_process() {
        keystore::lock();
        let mut counter = 0u32;
        let mut mock_hal = TestingHal::new();
        mock_hal.ui.set_enter_string(Box::new(|params| {
            counter += 1;
            match counter {
                1 => assert_eq!(params.title, "Set password"),
                2 => assert_eq!(params.title, "Repeat password"),
                _ => panic!("too many user inputs"),
            }
            Ok("password".into())
        }));

        mock_hal.securechip.event_counter_reset();
        assert_eq!(
            process(
                &mut mock_hal,
                &pb::SetPasswordRequest {
                    entropy: b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_vec(),
                }
            )
            .await,
            Ok(Response::Success(pb::Success {}))
        );
        assert_eq!(mock_hal.securechip.get_event_counter(), 6);
        assert!(!keystore::is_locked());
        assert!(keystore::copy_seed(&mut mock_hal).await.unwrap().len() == 32);

        drop(mock_hal); // to remove mutable borrow of counter
        assert_eq!(counter, 2);
    }

    /// Shorter host entropy results in shorter seed.
    #[async_test::test]
    async fn test_process_16_bytes() {
        keystore::lock();
        let mut mock_hal = TestingHal::new();
        mock_hal
            .ui
            .set_enter_string(Box::new(|_params| Ok("password".into())));
        assert_eq!(
            process(
                &mut mock_hal,
                &pb::SetPasswordRequest {
                    entropy: b"aaaaaaaaaaaaaaaa".to_vec(),
                }
            )
            .await,
            Ok(Response::Success(pb::Success {}))
        );
        assert!(!keystore::is_locked());
        assert!(keystore::copy_seed(&mut mock_hal).await.unwrap().len() == 16);
    }

    #[async_test::test]
    async fn test_process_seed() {
        let host_entropy = hex!("25569b9a11f9db6560459e8e48b4727a4c935300143d978989ed55db1d1b9cbe");
        let salt_root = hex!("3333333333333333444444444444444411111111111111112222222222222222");
        let mcu_random = hex!("00112233445566778899aabbccddeefffedcba98765432100123456789abcdef");
        let securechip_random =
            hex!("102030405060708090a0b0c0d0e0f0000f1e2d3c4b5a69788796a5b4c3d2e1f0");
        #[rustfmt::skip]
        /* Reproduce expected_seed with Python:
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
host_entropy = bytes.fromhex(
    "25569b9a11f9db6560459e8e48b4727a4c935300143d978989ed55db1d1b9cbe"
)
salt_root = bytes.fromhex(
    "3333333333333333444444444444444411111111111111112222222222222222"
)
purpose = b"keystore_seed_generation"
password = b"test password"

mixed_random = bytes(
    mcu ^ securechip ^ factory
    for mcu, securechip, factory in zip(mcu_random, securechip_random, factory_randomness)
)
device_random = hashlib.sha256(mixed_random).digest()
password_salted_hashed = hashlib.sha256(salt_root + purpose + password).digest()
expected_seed = bytes(
    device ^ host ^ password_byte
    for device, host, password_byte in zip(device_random, host_entropy, password_salted_hashed)
)
print(expected_seed.hex())
        */
        let expected_seed =
            hex!("69c1a051c7cc3119603c2fb80e87a54b56d08b3536aea577c8f5819249fc6032");

        for seed_len in [16, 32] {
            keystore::lock();
            let mut mock_hal = TestingHal::new();
            mock_hal
                .ui
                .set_enter_string(Box::new(|_params| Ok("test password".into())));
            mock_hal.memory.set_salt_root(&salt_root);
            mock_hal.random.mock_next(mcu_random);
            mock_hal.securechip.mock_random(securechip_random);

            assert_eq!(
                process(
                    &mut mock_hal,
                    &pb::SetPasswordRequest {
                        entropy: host_entropy[..seed_len].to_vec(),
                    }
                )
                .await,
                Ok(Response::Success(pb::Success {}))
            );
            assert_eq!(
                keystore::copy_seed(&mut mock_hal).await.unwrap().as_slice(),
                &expected_seed[..seed_len]
            );
        }
    }

    /// Invalid host entropy size.
    #[async_test::test]
    async fn test_process_invalid_host_entropy() {
        keystore::lock();
        let mut mock_hal = TestingHal::new();
        mock_hal
            .ui
            .set_enter_string(Box::new(|_params| Ok("password".into())));
        assert!(keystore::is_locked());
        assert_eq!(
            process(
                &mut mock_hal,
                &pb::SetPasswordRequest {
                    entropy: b"aaaaaaaaaaaaaaaaa".to_vec(),
                }
            )
            .await,
            Err(Error::InvalidInput),
        );
        assert!(keystore::is_locked());
    }

    #[async_test::test]
    async fn test_process_2nd_password_doesnt_match() {
        keystore::lock();
        let mut counter = 0u32;
        let mut mock_hal = TestingHal::new();
        mock_hal.ui.set_enter_string(Box::new(|_params| {
            counter += 1;
            Ok(match counter {
                1 => "password".into(),
                2 => "wrong".into(),
                _ => panic!("too many user inputs"),
            })
        }));
        assert_eq!(
            process(
                &mut mock_hal,
                &pb::SetPasswordRequest {
                    entropy: b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_vec(),
                }
            )
            .await,
            Err(Error::Generic),
        );
        assert!(keystore::is_locked());
    }
}
