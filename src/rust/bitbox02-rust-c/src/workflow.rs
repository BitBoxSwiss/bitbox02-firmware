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

//! This module is a quick workaround to use async workflows from U2F/FIDO2, where the root of the
//! usb message proessing is not ported to Rust. If that happens, the `async_usb` module can be
//! used and this can be deleted.

extern crate alloc;

use alloc::boxed::Box;
use alloc::string::String;
use bitbox02_rust::bb02_async::{block_on, spin, Task};
use bitbox02_rust::workflow::{confirm, status};
use core::task::Poll;

enum TaskState<'a, O> {
    Nothing,
    Running(Task<'a, O>),
    ResultAvailable(O),
}

static mut UNLOCK_STATE: TaskState<'static, Result<(), ()>> = TaskState::Nothing;

static mut CONFIRM_TITLE: Option<String> = None;
static mut CONFIRM_BODY: Option<String> = None;
static mut CONFIRM_PARAMS: Option<confirm::Params> = None;
static mut CONFIRM_STATE: TaskState<'static, Result<(), confirm::UserAbort>> = TaskState::Nothing;

#[no_mangle]
pub unsafe extern "C" fn rust_workflow_spawn_unlock() {
    UNLOCK_STATE = TaskState::Running(Box::pin(bitbox02_rust::workflow::unlock::unlock()));
}

#[no_mangle]
pub unsafe extern "C" fn rust_workflow_spawn_confirm(
    title: crate::util::CStr,
    body: crate::util::CStr,
) {
    CONFIRM_TITLE = Some(title.as_ref().into());
    CONFIRM_BODY = Some(body.as_ref().into());

    CONFIRM_PARAMS = Some(confirm::Params {
        title: CONFIRM_TITLE.as_ref().unwrap(),
        body: CONFIRM_BODY.as_ref().unwrap(),
        ..Default::default()
    });

    CONFIRM_STATE =
        TaskState::Running(Box::pin(confirm::confirm(CONFIRM_PARAMS.as_ref().unwrap())));
}

#[no_mangle]
pub unsafe extern "C" fn rust_workflow_spin() {
    match UNLOCK_STATE {
        TaskState::Running(ref mut task) => {
            let result = spin(task);
            if let Poll::Ready(result) = result {
                UNLOCK_STATE = TaskState::ResultAvailable(result);
            }
        }
        _ => (),
    }
    match CONFIRM_STATE {
        TaskState::Running(ref mut task) => {
            let result = spin(task);
            if let Poll::Ready(result) = result {
                CONFIRM_STATE = TaskState::ResultAvailable(result);
            }
        }
        _ => (),
    }
}

/// Returns true if there was a result.
#[no_mangle]
pub unsafe extern "C" fn rust_workflow_unlock_poll(result_out: &mut bool) -> bool {
    match UNLOCK_STATE {
        TaskState::ResultAvailable(result) => {
            UNLOCK_STATE = TaskState::Nothing;
            match result {
                Ok(()) => *result_out = true,
                Err(()) => *result_out = false,
            }
            true
        }
        _ => false,
    }
}

/// Returns true if there was a result.
#[no_mangle]
pub unsafe extern "C" fn rust_workflow_confirm_poll(result_out: &mut bool) -> bool {
    match &CONFIRM_STATE {
        TaskState::ResultAvailable(result) => {
            CONFIRM_TITLE = None;
            CONFIRM_BODY = None;
            CONFIRM_PARAMS = None;
            CONFIRM_STATE = TaskState::Nothing;
            *result_out = result.is_ok();
            true
        }
        _ => false,
    }
}

#[no_mangle]
pub unsafe extern "C" fn rust_workflow_abort_current() {
    UNLOCK_STATE = TaskState::Nothing;

    CONFIRM_TITLE = None;
    CONFIRM_BODY = None;
    CONFIRM_PARAMS = None;
    CONFIRM_STATE = TaskState::Nothing;
}

#[no_mangle]
pub unsafe extern "C" fn rust_workflow_status_blocking(
    msg: crate::util::CStr,
    status_success: bool,
) {
    block_on(status::status(msg.as_ref(), status_success))
}

#[no_mangle]
pub unsafe extern "C" fn rust_workflow_confirm_blocking(
    params: &bitbox02::confirm_params_t,
) -> bool {
    let title = crate::util::rust_util_cstr(params.title);
    let body = crate::util::rust_util_cstr(params.body);
    if !params.font.is_null() {
        panic!("Only default font supported");
    }
    let params = confirm::Params {
        title: title.as_ref(),
        title_autowrap: params.title_autowrap,
        body: body.as_ref(),
        font: confirm::Font::Default,
        scrollable: params.scrollable,
        longtouch: params.longtouch,
        accept_only: params.accept_only,
        accept_is_nextarrow: params.accept_is_nextarrow,
        display_size: params.display_size as _,
    };
    block_on(confirm::confirm(&params)).is_ok()
}
