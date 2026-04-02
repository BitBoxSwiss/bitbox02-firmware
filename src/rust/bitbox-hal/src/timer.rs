// SPDX-License-Identifier: Apache-2.0

use core::time::Duration;

#[allow(async_fn_in_trait)]
pub trait Timer {
    async fn delay_for(duration: Duration);
}
