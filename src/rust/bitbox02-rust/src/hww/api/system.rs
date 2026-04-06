// SPDX-License-Identifier: Apache-2.0

use super::Error;
use crate::hal::ui::ConfirmParams;
use crate::pb;

use pb::reboot_request::Purpose;
use pb::response::Response;

use crate::hal::{System, Ui};

pub async fn reboot_to_bootloader(
    hal: &mut impl crate::hal::Hal,
    &pb::RebootRequest { purpose }: &pb::RebootRequest,
) -> Result<Response, Error> {
    hal.ui()
        .confirm(&ConfirmParams {
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
    hal.system().reboot_to_bootloader()
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;

    use crate::hal::testing::TestingHal;

    #[async_test::test]
    #[should_panic(expected = "reboot_to_bootloader called")]
    async fn test_reboot_to_bootloader() {
        reboot_to_bootloader(
            &mut TestingHal::new(),
            &pb::RebootRequest {
                purpose: Purpose::Upgrade as _,
            },
        )
        .await
        .unwrap();
    }

    #[async_test::test]
    async fn test_reboot_to_bootloader_aborted() {
        let mut mock_hal = TestingHal::new();
        mock_hal.ui.abort_nth(0);
        assert_eq!(
            reboot_to_bootloader(
                &mut mock_hal,
                &pb::RebootRequest {
                    purpose: Purpose::Upgrade as _
                }
            )
            .await,
            Err(Error::UserAbort),
        );
    }
}
