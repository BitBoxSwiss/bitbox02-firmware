// SPDX-License-Identifier: Apache-2.0

use super::Error;
use crate::pb;

use pb::response::Response;

use crate::hal::Ui;
use crate::workflow::confirm;

pub async fn process(hal: &mut impl crate::hal::Hal) -> Result<Response, Error> {
    let params = confirm::Params {
        title: "RESET",
        body: "Proceed to\nfactory reset?",
        longtouch: true,
        ..Default::default()
    };

    hal.ui().confirm(&params).await.or(Err(Error::Generic))?;

    crate::reset::reset(hal, true).await;

    Ok(Response::Success(pb::Success {}))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::hal::testing::ui::Screen;
    use crate::hal::{Memory, testing::TestingHal};
    use alloc::boxed::Box;
    use bitbox02::testing::mock_memory;
    use util::bb02_async::block_on;

    #[test]
    pub fn test_reset() {
        mock_memory();

        // User aborted confirmation.
        let mut mock_hal = TestingHal::new();
        mock_hal.memory.set_device_name("test device name").unwrap();
        mock_hal.ui.abort_nth(0);
        assert_eq!(block_on(process(&mut mock_hal)), Err(Error::Generic));
        assert_eq!(
            mock_hal.ui.screens,
            vec![Screen::Confirm {
                title: "RESET".into(),
                body: "Proceed to\nfactory reset?".into(),
                longtouch: true,
            }],
        );
        assert_eq!(
            mock_hal.memory.get_device_name().as_str(),
            "test device name",
        );

        // All good.
        let mut mock_hal = TestingHal::new();
        mock_hal.memory.set_device_name("test device name").unwrap();
        assert_eq!(
            block_on(process(&mut mock_hal)),
            Ok(Response::Success(pb::Success {}))
        );
        assert_eq!(
            mock_hal.ui.screens,
            vec![
                Screen::Confirm {
                    title: "RESET".into(),
                    body: "Proceed to\nfactory reset?".into(),
                    longtouch: true,
                },
                Screen::Status {
                    title: "Device reset".into(),
                    success: true,
                }
            ],
        );
        assert_eq!(mock_hal.memory.get_device_name().as_str(), "My BitBox");
    }
}
