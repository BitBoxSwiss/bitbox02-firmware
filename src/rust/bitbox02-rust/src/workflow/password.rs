// SPDX-License-Identifier: Apache-2.0

use super::trinary_input_string;
use crate::hal::Ui;
use crate::hal::ui::{ConfirmParams, UserAbort};

use crate::hal::{Memory, memory::SecurechipType};

pub use trinary_input_string::CanCancel;

use alloc::string::String;

async fn prompt_cancel(hal: &mut impl crate::hal::Hal) -> Result<(), crate::hal::ui::UserAbort> {
    hal.ui()
        .confirm(&ConfirmParams {
            body: "Do you really\nwant to cancel?",
            ..Default::default()
        })
        .await
}

pub enum PasswordType {
    /// The password to be entered is the device unlock password.
    DevicePassword,
    /// The password to be entered is the BIP39 passphrase.
    Bip39Passphrase,
}

#[derive(Debug)]
pub enum EnterError {
    Memory,
    Cancelled,
}

/// If `can_cancel` is `Yes`, the workflow can be cancelled.
/// If it is no, the result is always `Ok(())`.
///
/// Example:
/// ```no_run
/// let pw = enter(hal, "Enter password", PassswordType::DevicePassword, CanCancel::No).await.unwrap();
/// // use pw.
/// ```
pub async fn enter(
    hal: &mut impl crate::hal::Hal,
    title: &str,
    password_type: PasswordType,
    can_cancel: CanCancel,
) -> Result<zeroize::Zeroizing<String>, EnterError> {
    let params = crate::hal::ui::EnterStringParams {
        title,
        hide: true,
        special_chars: match password_type {
            PasswordType::DevicePassword => false,
            PasswordType::Bip39Passphrase => true,
        },
        longtouch: true,
        default_to_digits: match password_type {
            PasswordType::DevicePassword => {
                match hal
                    .memory()
                    .get_securechip_type()
                    .map_err(|_| EnterError::Memory)?
                {
                    SecurechipType::Atecc => false,
                    SecurechipType::Optiga => true,
                }
            }
            PasswordType::Bip39Passphrase => false,
        },
        ..Default::default()
    };

    loop {
        match hal.ui().enter_string(&params, can_cancel, "").await {
            Ok(pw) => return Ok(pw),
            Err(UserAbort) => match prompt_cancel(hal).await {
                Ok(()) => return Err(EnterError::Cancelled),
                Err(UserAbort) => {}
            },
        }
    }
}

pub enum EnterTwiceError {
    DoNotMatch,
    EnterError(EnterError),
}

impl core::convert::From<EnterError> for EnterTwiceError {
    fn from(error: EnterError) -> Self {
        EnterTwiceError::EnterError(error)
    }
}

/// Prompt the user to enter a password twice. A warning is displayed
/// if the password has fewer than 4 chars. Returns `Err` if the two
/// passwords do not match, or if the user aborts at the warning.
///
/// Example:
/// ```no_run
/// let pw = enter_twice().await.unwrap();
/// // use pw.
pub async fn enter_twice(
    hal: &mut impl crate::hal::Hal,
) -> Result<zeroize::Zeroizing<String>, EnterTwiceError> {
    let password = enter(
        hal,
        "Set password",
        PasswordType::DevicePassword,
        CanCancel::Yes,
    )
    .await?;
    let password_repeat = enter(
        hal,
        "Repeat password",
        PasswordType::DevicePassword,
        CanCancel::Yes,
    )
    .await?;
    if password.as_str() != password_repeat.as_str() {
        hal.ui().status("Passwords\ndo not match", false).await;
        return Err(EnterTwiceError::DoNotMatch);
    }
    if password.as_str().len() < 4 {
        loop {
            match hal
                .ui()
                .confirm(&ConfirmParams {
                    title: "WARNING",
                    body: "Your password\n has fewer than\n 4 characters.\nContinue?",
                    longtouch: true,
                    ..Default::default()
                })
                .await
            {
                Ok(()) => break,
                Err(UserAbort) => match prompt_cancel(hal).await {
                    Ok(()) => return Err(EnterTwiceError::EnterError(EnterError::Cancelled)),
                    Err(UserAbort) => {}
                },
            }
        }
    }
    hal.ui().status("Success", true).await;
    Ok(password)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hal::testing::TestingHal;
    use alloc::boxed::Box;
    use util::bb02_async::block_on;

    #[test]
    fn test_enter_default_to_digits_atecc() {
        let mut hal = TestingHal::new();
        hal.memory.set_securechip_type(SecurechipType::Atecc);
        hal.ui.set_enter_string(Box::new(|params| {
            assert!(!params.default_to_digits);
            Ok("pw".into())
        }));

        let password = block_on(enter(
            &mut hal,
            "Enter password",
            PasswordType::DevicePassword,
            CanCancel::No,
        ))
        .unwrap();

        assert_eq!(password.as_str(), "pw");
    }

    #[test]
    fn test_enter_default_to_digits_optiga() {
        let mut hal = TestingHal::new();
        hal.memory.set_securechip_type(SecurechipType::Optiga);
        hal.ui.set_enter_string(Box::new(|params| {
            assert!(params.default_to_digits);
            Ok("pw".into())
        }));

        let password = block_on(enter(
            &mut hal,
            "Enter password",
            PasswordType::DevicePassword,
            CanCancel::No,
        ))
        .unwrap();

        assert_eq!(password.as_str(), "pw");
    }

    #[test]
    fn test_enter_cancelled() {
        let mut hal = TestingHal::new();
        hal.memory.set_securechip_type(SecurechipType::Atecc);
        hal.ui.set_enter_string(Box::new(|_params| Err(UserAbort)));

        let result = block_on(enter(
            &mut hal,
            "Enter password",
            PasswordType::DevicePassword,
            CanCancel::Yes,
        ));

        assert!(matches!(result, Err(EnterError::Cancelled)));
        assert!(
            hal.ui
                .contains_confirm("", "Do you really\nwant to cancel?")
        );
    }
}
