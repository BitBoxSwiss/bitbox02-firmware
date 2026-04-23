// SPDX-License-Identifier: Apache-2.0

use super::Error;
use crate::hal::ui::{CanCancel, ConfirmParams};
use crate::pb;

use pb::response::Response;

use crate::hal::Ui;
use crate::keystore;
use crate::workflow::{password, unlock};

pub async fn process(hal: &mut impl crate::hal::Hal) -> Result<Response, Error> {
    // Process confirmation and instruction for user
    hal.ui()
        .confirm(&ConfirmParams {
            title: "",
            body: "Proceed to\nchange password?",
            accept_is_nextarrow: true,
            ..Default::default()
        })
        .await?;
    // Unlock with old password
    let seed = unlock::unlock_keystore(hal, "Unlock device", CanCancel::Yes).await?;
    // Enter and confirm new password
    let new_password = password::enter_twice(hal).await?;

    // Re-encrypt seed with new password
    if let Err(err) = keystore::re_encrypt_seed(hal, &seed, &new_password).await {
        hal.ui()
            .status(&format!("Error\n{}", keystore::format_error(&err)), false)
            .await;
        return Err(Error::Generic);
    }

    Ok(Response::Success(pb::Success {}))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::hal::testing::ui::Screen;
    use crate::hal::{Memory, testing::TestingHal};
    use crate::workflow::unlock;
    use alloc::boxed::Box;
    use hex_lit::hex;

    // Test the intended success path
    #[async_test::test]
    async fn test_process_success() {
        //set up dummy (initialized, retained seed and bip39-seed)
        let seed = hex!("c7940c13479b8d9a6498f4e50d5a42e0d617bc8e8ac9f2b8cecf97e94c2b035c");
        let old_password = "old_password";
        let new_password = "new_password";

        let mut prompt_counter = 0u32;
        let mut hal = TestingHal::new();
        keystore::encrypt_and_store_seed(&mut hal, &seed, old_password)
            .await
            .unwrap();
        let unlock_animation = hal.ui.unlock_animation_create();
        unlock::unlock_bip39(&mut hal, &seed, unlock_animation).await;
        hal.memory.set_initialized().unwrap();
        hal.ui.screens.clear();

        // Allow exactly 3 prompts
        hal.ui.set_enter_string(Box::new(|params| {
            prompt_counter += 1;
            match prompt_counter {
                1 => {
                    assert_eq!(params.title, "Unlock device");
                    Ok(old_password.into())
                }
                2 => {
                    assert_eq!(params.title, "Set password");
                    Ok(new_password.into())
                }
                3 => {
                    assert_eq!(params.title, "Repeat password");
                    Ok(new_password.into())
                }
                _ => panic!("unexpected password prompt"),
            }
        }));
        // reset the chip counter
        hal.securechip.event_counter_reset();
        // call process
        let result = process(&mut hal).await;
        // assert success
        assert_eq!(result, Ok(Response::Success(pb::Success {})));
        // assert correct screens
        let screens = hal.ui.screens.clone();
        assert_eq!(
            screens,
            vec![
                Screen::Confirm {
                    title: "".into(),
                    body: "Proceed to\nchange password?".into(),
                    longtouch: false,
                },
                Screen::Status {
                    title: "Success".into(),
                    success: true,
                }
            ]
        );

        // We expect 14 secure chip events. This is intentionally brittle to catch
        // unintended changes in the number of securechip operations during password change.
        // If this fails after a legitimate change, update the expected count.
        assert_eq!(hal.securechip.get_event_counter(), 10);

        // check that the old password is no longer valid
        keystore::lock();
        assert!(matches!(
            keystore::unlock(&mut hal, old_password).await,
            Err(keystore::Error::IncorrectPassword)
        ));
        // check that the new password is valid
        assert_eq!(
            keystore::unlock(&mut hal, new_password)
                .await
                .unwrap()
                .as_slice(),
            seed.as_slice()
        );

        drop(hal);
        assert_eq!(prompt_counter, 3);
    }

    // Test that we fail if the unlock fails
    #[async_test::test]
    async fn test_process_unlock_failure() {
        let seed = hex!("c7940c13479b8d9a6498f4e50d5a42e0d617bc8e8ac9f2b8cecf97e94c2b035c");
        let correct_password = "correct_password";

        let mut prompt_counter = 0u32;
        let mut hal = TestingHal::new();
        keystore::encrypt_and_store_seed(&mut hal, &seed, correct_password)
            .await
            .unwrap();
        let unlock_animation = hal.ui.unlock_animation_create();
        unlock::unlock_bip39(&mut hal, &seed, unlock_animation).await;
        hal.memory.set_initialized().unwrap();
        keystore::lock();
        hal.ui.screens.clear();

        hal.ui.set_enter_string(Box::new(|params| {
            prompt_counter += 1;
            assert_eq!(params.title, "Unlock device");
            Ok("wrong_password".into())
        }));

        hal.securechip.event_counter_reset();
        let result = process(&mut hal).await;

        assert_eq!(result, Err(Error::Generic));
        assert_eq!(
            hal.ui.screens,
            vec![
                Screen::Confirm {
                    title: "".into(),
                    body: "Proceed to\nchange password?".into(),
                    longtouch: false,
                },
                Screen::Status {
                    title: "Wrong password".into(),
                    success: false,
                }
            ]
        );
        // We expect 5 secure chip events (sensitive to code changes)
        assert_eq!(hal.securechip.get_event_counter(), 4);

        // check that the old password is still valid
        assert_eq!(
            keystore::unlock(&mut hal, correct_password)
                .await
                .unwrap()
                .as_slice(),
            seed.as_slice()
        );

        drop(hal);
        assert_eq!(prompt_counter, 1);
    }

    // Test that we fail if the confirm password mismatch
    #[async_test::test]
    async fn test_process_confirm_password_mismatch() {
        let seed = hex!("c7940c13479b8d9a6498f4e50d5a42e0d617bc8e8ac9f2b8cecf97e94c2b035c");
        let old_password = "old_password";
        let first_password = "first_password";
        let second_password = "mismatch";

        let mut prompt_counter = 0u32;
        let mut hal = TestingHal::new();
        keystore::encrypt_and_store_seed(&mut hal, &seed, old_password)
            .await
            .unwrap();
        let unlock_animation = hal.ui.unlock_animation_create();
        unlock::unlock_bip39(&mut hal, &seed, unlock_animation).await;
        hal.memory.set_initialized().unwrap();
        keystore::lock();
        hal.ui.screens.clear();

        hal.ui.set_enter_string(Box::new(|params| {
            prompt_counter += 1;
            match prompt_counter {
                1 => {
                    assert_eq!(params.title, "Unlock device");
                    Ok(old_password.into())
                }
                2 => {
                    assert_eq!(params.title, "Set password");
                    Ok(first_password.into())
                }
                3 => {
                    assert_eq!(params.title, "Repeat password");
                    Ok(second_password.into())
                }
                _ => panic!("unexpected password prompt"),
            }
        }));
        let result = process(&mut hal).await;

        assert_eq!(result, Err(Error::Generic));
        // check that the old password is still valid
        assert_eq!(
            keystore::unlock(&mut hal, old_password)
                .await
                .unwrap()
                .as_slice(),
            seed.as_slice()
        );
        drop(hal);
    }
}
