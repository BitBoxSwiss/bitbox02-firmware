// SPDX-License-Identifier: Apache-2.0

pub struct UserAbort;

pub async fn sdcard() -> Result<(), UserAbort> {
    match bitbox02::ui::sdcard().await {
        bitbox02::ui::SdcardResponse::Inserted => Ok(()),
        bitbox02::ui::SdcardResponse::Cancelled => Err(UserAbort),
    }
}
