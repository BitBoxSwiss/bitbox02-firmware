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

#[no_mangle]
pub extern "C" fn rust_workflow_async_DEMO() {
    extern crate alloc;
    use bitbox02::password::Password;
    use bitbox02_rust::bb02_async::{spin, Task};
    use bitbox02_rust::workflow::confirm::{confirm, Params};
    use bitbox02_rust::workflow::password_enter::password_enter;

    async fn demo() {
        let params = Params {
            title: "demo",
            body: "Proceed to pw entry?",
            ..Default::default()
        };

        if !confirm(&params).await {
            return;
        }

        let mut pw1 = Password::new();
        password_enter("Enter 1st PW", true, &mut pw1).await;
        bitbox02::screen_print_debug(
            unsafe { core::str::from_utf8_unchecked(&pw1.as_ref()[..]) },
            2000,
        );
        let mut pw2 = Password::new();
        password_enter("Enter 2nd PW", false, &mut pw2).await;
        bitbox02::screen_print_debug(
            unsafe { core::str::from_utf8_unchecked(&pw2.as_ref()[..]) },
            2000,
        );
    }

    let future = demo();

    let mut task: Task<()> = alloc::boxed::Box::pin(future);

    // poll task in loop, processing screen asynchronously.
    loop {
        bitbox02::ui::screen_process();
        if spin(&mut task).is_ready() {
            break;
        }
    }
}
