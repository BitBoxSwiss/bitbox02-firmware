// Copyright 2020 Shift Cryptosecurity AG
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
use bitbox02::password::Password;
use core::cell::RefCell;

/// Example:
/// ```no_run
/// let mut pw = Password::new();
/// password_enter("Enter password", true, &mut pw).await;
/// // use pw.
/// ```
pub async fn password_enter(title: &str, special_chars: bool, password_out: &mut Password) {
    let result = RefCell::new(None);
    let mut component =
        bitbox02::ui::trinary_input_string_create_password(title, special_chars, |pw| {
            *result.borrow_mut() = Some(pw);
        });
    component.screen_stack_push();
    option(&result).await;
    let result: &Option<Password> = &result.borrow();
    let result: Option<&Password> = result.as_ref();
    let result: &Password = result.unwrap();
    password_out.clone_from(&result);
}
