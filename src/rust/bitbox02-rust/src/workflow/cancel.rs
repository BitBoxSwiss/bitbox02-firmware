// SPDX-License-Identifier: Apache-2.0

use crate::workflow::confirm;
use core::future::Future;

#[derive(Debug)]
pub enum Error {
    Cancelled,
}

/// Returns true if user cancelled and wants to exit.
pub async fn cancel(title: &str) -> bool {
    let params = confirm::Params {
        title,
        body: "Do you really\nwant to cancel?",
        ..Default::default()
    };

    // Err(UserAbort) means _do not cancel_, ask again.
    !matches!(confirm::confirm(&params).await, Err(confirm::UserAbort))
}

pub async fn with_cancel<F, T, E, GEN>(title: &str, future_generator: GEN) -> Result<T, Error>
where
    GEN: Fn() -> F,
    F: Future<Output = Result<T, E>>,
{
    loop {
        match future_generator().await {
            Ok(output) => return Ok(output),
            Err(_) => {
                if cancel(title).await {
                    return Err(super::cancel::Error::Cancelled);
                }
            }
        }
    }
}
