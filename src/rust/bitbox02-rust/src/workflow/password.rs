// Copyright 2020 Shift Cryptosecurity AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::{confirm, trinary_input_string, Workflows};

pub use trinary_input_string::{CanCancel, Error};

use alloc::string::String;

async fn prompt_cancel(hal: &mut impl crate::hal::Hal) -> Result<(), confirm::UserAbort> {
    hal.ui()
        .confirm(&confirm::Params {
            body: "Do you really\nwant to cancel?",
            ..Default::default()
        })
        .await
}

/// If `can_cancel` is `Yes`, the workflow can be cancelled.
/// If it is no, the result is always `Ok(())`.
///
/// Example:
/// ```no_run
/// let pw = enter("Enter password", true, CanCancel::No).await.unwrap();
/// // use pw.
/// ```
pub async fn enter(
    hal: &mut impl crate::hal::Hal,
    title: &str,
    special_chars: bool,
    can_cancel: CanCancel,
) -> Result<zeroize::Zeroizing<String>, Error> {
    let params = trinary_input_string::Params {
        title,
        hide: true,
        special_chars,
        longtouch: true,
        ..Default::default()
    };

    loop {
        match hal.ui().enter_string(&params, can_cancel, "").await {
            o @ Ok(_) => return o,
            Err(Error::Cancelled) => match prompt_cancel(hal).await {
                Ok(()) => return Err(Error::Cancelled),
                Err(confirm::UserAbort) => {}
            },
        }
    }
}

pub enum EnterTwiceError {
    DoNotMatch,
    Cancelled,
}

impl core::convert::From<Error> for EnterTwiceError {
    fn from(error: Error) -> Self {
        match error {
            Error::Cancelled => EnterTwiceError::Cancelled,
        }
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
    let password = enter(hal, "Set password", false, CanCancel::Yes).await?;
    let password_repeat = enter(hal, "Repeat password", false, CanCancel::Yes).await?;
    if password.as_str() != password_repeat.as_str() {
        hal.ui().status("Passwords\ndo not match", false).await;
        return Err(EnterTwiceError::DoNotMatch);
    }
    if password.as_str().len() < 4 {
        loop {
            match hal
                .ui()
                .confirm(&confirm::Params {
                    title: "WARNING",
                    body: "Your password\n has fewer than\n 4 characters.\nContinue?",
                    longtouch: true,
                    ..Default::default()
                })
                .await
            {
                Ok(()) => break,
                Err(confirm::UserAbort) => match prompt_cancel(hal).await {
                    Ok(()) => return Err(EnterTwiceError::Cancelled),
                    Err(confirm::UserAbort) => {}
                },
            }
        }
    }
    hal.ui().status("Success", true).await;
    Ok(password)
}
