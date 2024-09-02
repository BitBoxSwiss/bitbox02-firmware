// Copyright 2020 Shift Crypto AG
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
use core::cell::RefCell;

pub struct UserAbort;

pub async fn sdcard() -> Result<(), UserAbort> {
    let result = RefCell::new(None as Option<Result<(), UserAbort>>);
    let mut component = bitbox02::ui::sdcard_create(|sd_done| {
        *result.borrow_mut() = if sd_done {
            Some(Ok(()))
        } else {
            Some(Err(UserAbort))
        };
    });
    component.screen_stack_push();
    option_no_screensaver(&result).await
}
