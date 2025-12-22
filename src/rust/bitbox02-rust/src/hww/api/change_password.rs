use super::Error;
use crate::pb;

use pb::response::Response;

use crate::hal::Ui;
use crate::keystore;
use crate::workflow::{confirm, password, unlock};

pub async fn process(hal: &mut impl crate::hal::Hal) -> Result<Response, Error> {
    // Process confirmation and instruction for user
    hal.ui()
        .confirm(&confirm::Params {
            title: "",
            body: "Proceed to\nchange password?",
            accept_is_nextarrow: true,
            ..Default::default()
        })
        .await?;
    // Unlock with old password
    let seed = unlock::unlock_keystore(hal, "Unlock device", unlock::CanCancel::Yes).await?;
    // Enter and confirm new password
    let new_password = password::enter_twice(hal).await?;

    // Re-encrypt seed with new password
    if let Err(err) = keystore::re_encrypt_seed(hal, &seed, &new_password) {
        hal.ui().status(&format!("Error\n{:?}", err), false).await;
        return Err(Error::Generic);
    }

    Ok(Response::Success(pb::Success {}))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::hal::{Memory, testing::TestingHal};
    use crate::workflow::{testing::Screen, unlock};
    use alloc::boxed::Box;
    use bitbox02::testing::mock_memory;
    use hex_lit::hex;
    use util::bb02_async::block_on;

    // Test the intended success path
    #[test]
    fn test_process_success() {
        //set up dummy (initialized, retained seed and bip39-seed)
        mock_memory();
        let seed = hex!("c7940c13479b8d9a6498f4e50d5a42e0d617bc8e8ac9f2b8cecf97e94c2b035c");
        let old_password = "old_password";
        let new_password = "new_password";

        let mut hal = TestingHal::new();
        keystore::encrypt_and_store_seed(&mut hal, &seed, old_password).unwrap();
        block_on(unlock::unlock_bip39(&mut hal, &seed));
        hal.memory.set_initialized().unwrap();

        // Allow exactly 3 prompts
        let mut prompt_counter = 0u32;
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
        let result = block_on(process(&mut hal));
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
        assert_eq!(hal.securechip.get_event_counter(), 14);

        // check that the old password is no longer valid
        keystore::lock();
        assert!(matches!(
            block_on(keystore::unlock(&mut hal, old_password)),
            Err(keystore::Error::IncorrectPassword)
        ));
        // check that the new password is valid
        assert_eq!(
            block_on(keystore::unlock(&mut hal, new_password))
                .unwrap()
                .as_slice(),
            seed.as_slice()
        );

        drop(hal);
        assert_eq!(prompt_counter, 3);
    }

    // Test that we fail if the unlock fails
    #[test]
    fn test_process_unlock_failure() {
        mock_memory();

        let seed = hex!("c7940c13479b8d9a6498f4e50d5a42e0d617bc8e8ac9f2b8cecf97e94c2b035c");
        let correct_password = "correct_password";

        let mut hal = TestingHal::new();
        keystore::encrypt_and_store_seed(&mut hal, &seed, correct_password).unwrap();
        block_on(unlock::unlock_bip39(&mut hal, &seed));
        hal.memory.set_initialized().unwrap();
        keystore::lock();

        let mut prompt_counter = 0u32;
        hal.ui.set_enter_string(Box::new(|params| {
            prompt_counter += 1;
            assert_eq!(params.title, "Unlock device");
            Ok("wrong_password".into())
        }));

        hal.securechip.event_counter_reset();
        let result = block_on(process(&mut hal));

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
        assert_eq!(hal.securechip.get_event_counter(), 5);

        // check that the old password is still valid
        assert_eq!(
            block_on(keystore::unlock(&mut hal, correct_password))
                .unwrap()
                .as_slice(),
            seed.as_slice()
        );

        drop(hal);
        assert_eq!(prompt_counter, 1);
    }

    // Test that we fail if the confirm password mismatch
    #[test]
    fn test_process_confirm_password_mismatch() {
        mock_memory();

        let seed = hex!("c7940c13479b8d9a6498f4e50d5a42e0d617bc8e8ac9f2b8cecf97e94c2b035c");
        let old_password = "old_password";
        let first_password = "first_password";
        let second_password = "mismatch";

        let mut hal = TestingHal::new();
        keystore::encrypt_and_store_seed(&mut hal, &seed, old_password).unwrap();
        block_on(unlock::unlock_bip39(&mut hal, &seed));
        hal.memory.set_initialized().unwrap();
        keystore::lock();

        let mut prompt_counter = 0u32;
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
        let result = block_on(process(&mut hal));

        assert_eq!(result, Err(Error::Generic));
        // check that the old password is still valid
        assert_eq!(
            block_on(keystore::unlock(&mut hal, old_password))
                .unwrap()
                .as_slice(),
            seed.as_slice()
        );
        drop(hal);
    }
}
