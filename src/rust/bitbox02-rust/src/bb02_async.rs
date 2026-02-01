// SPDX-License-Identifier: Apache-2.0

use crate::hal::Ui;
use core::cell::RefCell;

/// Disables the screensaver while waiting for an option to contain a value. Afterwards, it returns that value
pub async fn option_no_screensaver<O>(hal_ui: &mut impl Ui, opt: &RefCell<Option<O>>) -> O {
    hal_ui.screen_saver_disable();
    let result = util::bb02_async::option(opt).await;
    hal_ui.screen_saver_enable();
    result
}
