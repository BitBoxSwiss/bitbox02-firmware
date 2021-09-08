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

use super::error::Error;
use crate::pb;

use pb::response::Response;

use crate::workflow::confirm;

pub async fn process() -> Result<Response, Error> {
    let params = confirm::Params {
        title: "RESET",
        body: "Proceed to\nfactory reset?",
        longtouch: true,
        ..Default::default()
    };

    confirm::confirm(&params)
        .await
        .map_err(Error::err_generic)?;

    bitbox02::reset(true);

    Ok(Response::Success(pb::Success {}))
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;

    use crate::bb02_async::block_on;
    use crate::hww::api::error::ErrorKind;
    use bitbox02::testing::{mock, Data, MUTEX};
    use std::boxed::Box;

    #[test]
    pub fn test_reset() {
        let _guard = MUTEX.lock().unwrap();

        // All good.
        mock(Data {
            ui_confirm_create: Some(Box::new(|params| {
                assert_eq!(params.body, "Proceed to\nfactory reset?");
                true
            })),
            reset: Some(Box::new(|status| {
                assert_eq!(status, true);
            })),
            ..Default::default()
        });
        assert_eq!(block_on(process()), Ok(Response::Success(pb::Success {})));

        // User aborted confirmation.
        mock(Data {
            ui_confirm_create: Some(Box::new(|params| {
                assert_eq!(params.body, "Proceed to\nfactory reset?");
                false
            })),
            ..Default::default()
        });
        assert_eq!(block_on(process()).unwrap_err().kind, ErrorKind::Generic);
    }
}
