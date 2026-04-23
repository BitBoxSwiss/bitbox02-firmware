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
