// SPDX-License-Identifier: Apache-2.0

use core::cell::RefCell;
use util::bb02_async::option;

/// Performs the unlock animation. Its duration is determined by the component render rate, see
/// unlock_animation.c
/// Caller must ensure screen saver logic by disabling/enabling screensaver
pub async fn animate() {
    let result = RefCell::new(None as Option<()>);
    let mut component = bitbox02::ui::unlock_animation_create(|| {
        *result.borrow_mut() = Some(());
    });
    component.screen_stack_push();
    option(&result).await // no screensaver logic!
}
