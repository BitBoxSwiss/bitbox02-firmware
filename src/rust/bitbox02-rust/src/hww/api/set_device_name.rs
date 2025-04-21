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

use super::Error;
use crate::pb;

use pb::response::Response;

use crate::hal::Ui;
use crate::workflow::confirm;

pub async fn process(
    hal: &mut impl crate::hal::Hal,
    pb::SetDeviceNameRequest { name }: &pb::SetDeviceNameRequest,
) -> Result<Response, Error> {
    if !util::name::validate(name, bitbox02::memory::DEVICE_NAME_MAX_LEN) {
        return Err(Error::InvalidInput);
    }

    let params = confirm::Params {
        title: "Name",
        body: name,
        scrollable: true,
        ..Default::default()
    };

    hal.ui().confirm(&params).await?;

    bitbox02::memory::set_device_name(name)?;

    Ok(Response::Success(pb::Success {}))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::bb02_async::block_on;
    use crate::hal::testing::TestingHal;
    use crate::workflow::testing::Screen;
    use alloc::boxed::Box;
    use bitbox02::testing::mock_memory;

    #[test]
    pub fn test_set_device_name() {
        const SOME_NAME: &str = "foo";

        // All good.
        mock_memory();
        let mut mock_hal = TestingHal::new();
        assert_eq!(
            block_on(process(
                &mut mock_hal,
                &pb::SetDeviceNameRequest {
                    name: SOME_NAME.into()
                }
            )),
            Ok(Response::Success(pb::Success {}))
        );
        assert_eq!(
            mock_hal.ui.screens,
            vec![Screen::Confirm {
                title: "Name".into(),
                body: SOME_NAME.into(),
                longtouch: false,
            }]
        );
        assert_eq!(SOME_NAME, &bitbox02::memory::get_device_name());

        // User aborted confirmation.
        let mut mock_hal = TestingHal::new();
        mock_hal.ui.abort_nth(0);
        assert_eq!(
            block_on(process(
                &mut mock_hal,
                &pb::SetDeviceNameRequest {
                    name: SOME_NAME.into()
                }
            )),
            Err(Error::UserAbort)
        );
        assert_eq!(
            mock_hal.ui.screens,
            vec![Screen::Confirm {
                title: "Name".into(),
                body: SOME_NAME.into(),
                longtouch: false,
            }]
        );

        // Non-ascii character.
        assert_eq!(
            block_on(process(
                &mut TestingHal::new(),
                &pb::SetDeviceNameRequest {
                    name: "emoji are 😃, 😭, and 😈".into()
                }
            )),
            Err(Error::InvalidInput)
        );

        // Non-printable character.
        assert_eq!(
            block_on(process(
                &mut TestingHal::new(),
                &pb::SetDeviceNameRequest {
                    name: "foo\nbar".into()
                }
            )),
            Err(Error::InvalidInput)
        );

        // Too long.
        assert_eq!(
            block_on(process(
                &mut TestingHal::new(),
                &pb::SetDeviceNameRequest {
                    name: core::str::from_utf8(&[b'a'; 500]).unwrap().into()
                }
            )),
            Err(Error::InvalidInput)
        );
    }
}
