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

// TODO: figure out how to deal with the static muts below.
// https://doc.rust-lang.org/nightly/edition-guide/rust-2024/static-mut-references.html
#![allow(static_mut_refs)]

extern crate alloc;

use alloc::boxed::Box;
use alloc::string::String;
use bitbox02_rust::bb02_async::{spin, Task};
use bitbox02_rust::workflow::confirm;
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
static mut BITBOX02_HAL: bitbox02_rust::hal::BitBox02Hal = bitbox02_rust::hal::BitBox02Hal::new();

#[no_mangle]
pub unsafe extern "C" fn rust_workflow_spawn_unlock() {
    UNLOCK_STATE = TaskState::Running(Box::pin(bitbox02_rust::workflow::unlock::unlock(
        &mut BITBOX02_HAL,
    )));
}

#[no_mangle]
pub unsafe extern "C" fn rust_workflow_spawn_confirm(
    title: *const core::ffi::c_char,
    body: *const core::ffi::c_char,
) {
    CONFIRM_TITLE = Some(core::ffi::CStr::from_ptr(title).to_str().unwrap().into());
    CONFIRM_BODY = Some(core::ffi::CStr::from_ptr(body).to_str().unwrap().into());
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
    match CONFIRM_STATE {
        TaskState::ResultAvailable(ref result) => {
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
