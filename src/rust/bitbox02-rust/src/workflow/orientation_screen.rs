// Copyright 2025 Shift Crypto AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::bb02_async::option_no_screensaver;

pub async fn create<CB>(orientation_selected_cb: CB)
where
    CB: FnOnce(),
{
    let result = core::cell::RefCell::new(None as Option<()>);
    let mut orientation_arrows = bitbox02::ui::orientation_arrows(|upside_down| {
        if upside_down {
            bitbox02::screen_rotate()
        }
        *result.borrow_mut() = Some(());
    });
    orientation_arrows.screen_stack_push();

    // Wait until orientation has been chosen
    option_no_screensaver(&result).await;
    drop(orientation_arrows);

    // During this delay the bb02 logotype is shown
    if let Ok(delay) = bitbox02::delay::Delay::from_ms(1300) {
        delay.await;
    }

    // Switch to lockscreen that shows "See the bitbox app" and device name
    bitbox02::ui::screen_process_waiting_switch_to_lockscreen();

    orientation_selected_cb();
}
