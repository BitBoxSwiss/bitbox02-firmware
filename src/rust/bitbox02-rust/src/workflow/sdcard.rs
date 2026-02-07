// SPDX-License-Identifier: Apache-2.0

pub struct UserAbort;

pub async fn sdcard() -> Result<(), UserAbort> {
    if bitbox02::ui::sdcard().await {
        Ok(())
    } else {
        Err(UserAbort)
    }
}
