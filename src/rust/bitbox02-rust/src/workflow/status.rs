// SPDX-License-Identifier: Apache-2.0

pub async fn status(title: &str, status_success: bool) {
    bitbox02::ui::status(title, status_success).await;
}
