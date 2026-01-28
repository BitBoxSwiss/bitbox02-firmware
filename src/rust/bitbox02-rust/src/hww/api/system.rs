// SPDX-License-Identifier: Apache-2.0

use super::Error;
use crate::pb;

use pb::reboot_request::Purpose;
use pb::response::Response;

use crate::hal::{System, Ui};
use crate::workflow::confirm;

pub async fn reboot_to_bootloader(
    hal: &mut impl crate::hal::Hal,
    &pb::RebootRequest { purpose }: &pb::RebootRequest,
) -> Result<Response, Error> {
    hal.ui()
        .confirm(&confirm::Params {
            title: "",
            body: match Purpose::try_from(purpose) {
                Ok(Purpose::Upgrade) => "Proceed to upgrade?",
                Ok(Purpose::Settings) => "Go to\nstartup settings?",
                // No error, if new client library sends a purpose that we don't understand,
                // we reboot anyway.
                Err(_) => "Reboot?",
            },
            ..Default::default()
        })
        .await?;
    hal.system().reboot_to_bootloader();
    Ok(Response::Success(pb::Success {}))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::hal::testing::TestingHal;
    use crate::workflow::testing::Screen;
    use alloc::boxed::Box;
    use util::bb02_async::block_on;

    #[test]
    pub fn test_reboot_to_bootloader() {
        let mut mock_hal = TestingHal::new();
        let result = block_on(reboot_to_bootloader(
            &mut mock_hal,
            &pb::RebootRequest {
                purpose: Purpose::Upgrade as _,
            },
        ));
        assert_eq!(result, Ok(Response::Success(pb::Success {})));
        assert_eq!(mock_hal.system.get_reboot_to_bootloader_counter(), 1);
    }

    #[test]
    pub fn test_reboot_to_bootloader_aborted() {
        let mut mock_hal = TestingHal::new();
        mock_hal.ui.abort_nth(0);
        assert_eq!(
            block_on(reboot_to_bootloader(
                &mut mock_hal,
                &pb::RebootRequest {
                    purpose: Purpose::Upgrade as _
                }
            )),
            Err(Error::UserAbort),
        );
    }
}
