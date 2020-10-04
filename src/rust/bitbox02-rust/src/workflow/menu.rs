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

use crate::bb02_async::option;
use bitbox02::ui;
use core::cell::RefCell;

extern crate alloc;
use alloc::boxed::Box;

pub async fn menu() {
    let result = RefCell::new(None);
    let mut component = ui::menu_create(ui::MenuParams {
        words: &["foo", "bar"],
        title: Some("demo title"),
        select_word_cb: Some(Box::new(|idx| {
            *result.borrow_mut() = Some(idx);
        })),
        continue_on_last_cb: Some(Box::new(|| crate::print_debug!(1000, "hit continue"))),
        cancel_cb: Some(Box::new(|| crate::print_debug!(1000, "hit cancel"))),
    });
    component.screen_stack_push();
    option(&result).await;
    crate::print_debug!(1000, "chosen: {}", result.borrow().unwrap());
}
