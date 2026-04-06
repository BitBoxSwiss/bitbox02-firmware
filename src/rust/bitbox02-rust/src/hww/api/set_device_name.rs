// SPDX-License-Identifier: Apache-2.0

use super::Error;
use crate::hal::ui::ConfirmParams;
use crate::pb;

use pb::response::Response;

use crate::hal::{Memory, Ui};

pub async fn process(
    hal: &mut impl crate::hal::Hal,
    pb::SetDeviceNameRequest { name }: &pb::SetDeviceNameRequest,
) -> Result<Response, Error> {
    if !util::name::validate(name, bitbox_hal::memory::DEVICE_NAME_MAX_LEN) {
        return Err(Error::InvalidInput);
    }

    let params = ConfirmParams {
        title: "Name",
        body: name,
        scrollable: true,
        ..Default::default()
    };

    hal.ui().confirm(&params).await?;

    hal.memory().set_device_name(name)?;

    Ok(Response::Success(pb::Success {}))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::hal::testing::TestingHal;
    use crate::hal::testing::ui::Screen;
    use alloc::boxed::Box;

    #[async_test::test]
    pub async fn test_set_device_name() {
        const SOME_NAME: &str = "foo";

        // All good.
        let mut mock_hal = TestingHal::new();
        assert_eq!(
            process(
                &mut mock_hal,
                &pb::SetDeviceNameRequest {
                    name: SOME_NAME.into()
                }
            )
            .await,
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
        assert_eq!(SOME_NAME, mock_hal.memory.get_device_name());

        // User aborted confirmation.
        let mut mock_hal = TestingHal::new();
        mock_hal.ui.abort_nth(0);
        assert_eq!(
            process(
                &mut mock_hal,
                &pb::SetDeviceNameRequest {
                    name: SOME_NAME.into()
                }
            )
            .await,
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
            process(
                &mut TestingHal::new(),
                &pb::SetDeviceNameRequest {
                    name: "emoji are 😃, 😭, and 😈".into()
                }
            )
            .await,
            Err(Error::InvalidInput)
        );

        // Non-printable character.
        assert_eq!(
            process(
                &mut TestingHal::new(),
                &pb::SetDeviceNameRequest {
                    name: "foo\nbar".into()
                }
            )
            .await,
            Err(Error::InvalidInput)
        );

        // Too long.
        assert_eq!(
            process(
                &mut TestingHal::new(),
                &pb::SetDeviceNameRequest {
                    name: core::str::from_utf8(&[b'a'; 500]).unwrap().into()
                }
            )
            .await,
            Err(Error::InvalidInput)
        );
    }
}
