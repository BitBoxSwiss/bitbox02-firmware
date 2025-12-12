// SPDX-License-Identifier: Apache-2.0

use crate::bb02_async::option_no_screensaver;
use core::cell::RefCell;

/// Performs the unlock animation. Its duration is determined by the component render rate, see
/// unlock_animation.c
pub async fn animate() {
    let result = RefCell::new(None as Option<()>);
    let mut component = bitbox02::ui::unlock_animation_create(|| {
        *result.borrow_mut() = Some(());
    });
    component.screen_stack_push();
    option_no_screensaver(&result).await
}
