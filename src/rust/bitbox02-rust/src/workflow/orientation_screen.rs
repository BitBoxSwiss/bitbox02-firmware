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

use bitbox02::delay::delay_for;
use bitbox02::ui::choose_orientation;
use core::time::Duration;

pub async fn orientation_screen() -> bool {
    let upside_down = choose_orientation().await;
    if upside_down {
        bitbox02::screen_rotate()
    }

    // During this delay the bb02 logotype is shown
    delay_for(Duration::from_millis(1300)).await;

    // Switch to lockscreen that shows "See the bitbox app" and device name
    bitbox02::ui::screen_process_waiting_switch_to_lockscreen();

    upside_down
}
