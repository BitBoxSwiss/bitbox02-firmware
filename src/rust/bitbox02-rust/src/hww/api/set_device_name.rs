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

use super::pb;
use super::Error;

use pb::response::Response;

use crate::workflow::confirm;

pub async fn process(
    pb::SetDeviceNameRequest { name }: &pb::SetDeviceNameRequest,
) -> Result<Response, Error> {
    let params = confirm::Params {
        title: "Name",
        body: &name,
        scrollable: true,
        ..Default::default()
    };

    if !confirm::confirm(&params).await {
        return Err(Error::COMMANDER_ERR_USER_ABORT);
    }

    if bitbox02::memory::set_device_name(&name).is_err() {
        return Err(Error::COMMANDER_ERR_MEMORY);
    }

    Ok(Response::Success(pb::Success {}))
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;

    use crate::bb02_async::block_on;
    use bitbox02::testing::{mock, Data, MUTEX};
    use std::boxed::Box;

    #[test]
    pub fn test_set_device_name() {
        let _guard = MUTEX.lock().unwrap();

        static SOME_NAME: &str = "foo";

        // All good.
        mock(Data {
            ui_confirm_create_body: Some(SOME_NAME.into()),
            ui_confirm_create_result: Some(true),
            memory_set_device_name: Some(Box::new(|name| {
                assert_eq!(name, SOME_NAME);
                Ok(())
            })),
            ..Default::default()
        });
        assert_eq!(
            block_on(process(&pb::SetDeviceNameRequest {
                name: SOME_NAME.into()
            })),
            Ok(Response::Success(pb::Success {}))
        );

        // User aborted confirmation.
        mock(Data {
            ui_confirm_create_body: Some(SOME_NAME.into()),
            ui_confirm_create_result: Some(false),
            ..Default::default()
        });
        assert_eq!(
            block_on(process(&pb::SetDeviceNameRequest {
                name: SOME_NAME.into()
            })),
            Err(Error::COMMANDER_ERR_USER_ABORT)
        );

        // Memory write error.
        mock(Data {
            ui_confirm_create_body: Some(SOME_NAME.into()),
            ui_confirm_create_result: Some(true),
            memory_set_device_name: Some(Box::new(|_| Err(()))),
            ..Default::default()
        });
        assert_eq!(
            block_on(process(&pb::SetDeviceNameRequest {
                name: SOME_NAME.into()
            })),
            Err(Error::COMMANDER_ERR_MEMORY)
        );
    }
}
