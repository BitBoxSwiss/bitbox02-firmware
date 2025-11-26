// Copyright 2020 Shift Crypto AG
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

use crate::workflow::confirm;

#[derive(Debug)]
pub enum Error {
    Cancelled,
}

/// returns true if user cancelled and wants to exit
pub async fn cancel(title: &str) -> bool {
    let params = confirm::Params {
        title,
        body: "Do you really\nwant to cancel?",
        ..Default::default()
    };

    // Err(UserAbort) means _do not cancel_, ask again
    !matches!(confirm::confirm(&params).await, Err(confirm::UserAbort))
}

pub async fn with_cancel<F, T, E, GEN>(title: &str, future_generator: GEN) -> Result<T, Error>
where
    GEN: Fn() -> F,
    F: Future<Output = Result<T, E>>,
{
    loop {
        match future_generator().await {
            Ok(o) => return Ok(o),
            Err(_) => {
                if cancel(title).await {
                    return Err(super::cancel::Error::Cancelled);
                }
            }
        }
    }
}
