// SPDX-License-Identifier: Apache-2.0

use core::cell::RefCell;

/// Disables the screensaver while waiting for an option to contain a value. Afterwards, it returns that value
pub async fn option_no_screensaver<O>(opt: &RefCell<Option<O>>) -> O {
    bitbox02::screen_saver::screen_saver_disable();
    let result = util::bb02_async::option(opt).await;
    bitbox02::screen_saver::screen_saver_enable();
    result
}
