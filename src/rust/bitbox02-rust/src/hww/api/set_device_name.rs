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
    use super::*;

    use crate::bb02_async::block_on;

    #[test]
    pub fn test_set_device_name() {
        unsafe {
            // TODO: remove unsafe by using Arc<Mutex<>>
            bitbox02::memory::testing::SET_DEVICE_NAME_EXPECTED_NAME = Some("foo".into());
            bitbox02::memory::testing::SET_DEVICE_NAME_RESULT = Ok(());
        }
        assert_eq!(
            block_on(process(&pb::SetDeviceNameRequest { name: "foo".into() })),
            Ok(Response::Success(pb::Success {}))
        );

        unsafe {
            bitbox02::memory::testing::SET_DEVICE_NAME_RESULT = Err(());
        }
        assert_eq!(
            block_on(process(&pb::SetDeviceNameRequest { name: "foo".into() })),
            Err(Error::COMMANDER_ERR_MEMORY),
        );
    }
}
