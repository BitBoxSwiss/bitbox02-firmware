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

//! This module is a quick workaround to use async workflows from U2F/FIDO2, where the root of the
//! usb message proessing is not ported to Rust. If that happens, the `async_usb` module can be
//! used and this can be deleted.

// TODO: figure out how to deal with the static muts below.
// https://doc.rust-lang.org/nightly/edition-guide/rust-2024/static-mut-references.html
#![allow(static_mut_refs)]
#![allow(clippy::missing_safety_doc)]

extern crate alloc;

use crate::workflow::confirm;
use alloc::string::String;
use async_channel::{Receiver, TryRecvError};
use core::ffi::CStr;
use grounded::uninit::GroundedCell;

enum TaskState<O> {
    Nothing,
    Running(Receiver<O>),
}

static UNLOCK_STATE: GroundedCell<TaskState<Result<(), ()>>> = GroundedCell::uninit();
static CONFIRM_STATE: GroundedCell<TaskState<Result<(), confirm::UserAbort>>> =
    GroundedCell::uninit();

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_workflow_spawn_unlock() {
    static BITBOX02_HAL: GroundedCell<crate::hal::BitBox02Hal> = GroundedCell::const_init();
    unsafe {
        UNLOCK_STATE
            .get()
            .write(TaskState::Running(crate::main_loop::spawn(
                crate::workflow::unlock::unlock(BITBOX02_HAL.get().as_mut().unwrap()),
            )));
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_workflow_spawn_confirm(
    title: *const core::ffi::c_char,
    body: *const core::ffi::c_char,
) {
    static CONFIRM_TITLE: GroundedCell<String> = GroundedCell::uninit();
    static CONFIRM_BODY: GroundedCell<String> = GroundedCell::uninit();
    static CONFIRM_PARAMS: GroundedCell<confirm::Params> = GroundedCell::uninit();
    unsafe {
        CONFIRM_TITLE
            .get()
            .write(CStr::from_ptr(title).to_str().unwrap().into());
        CONFIRM_BODY
            .get()
            .write(CStr::from_ptr(body).to_str().unwrap().into());
        CONFIRM_PARAMS.get().write(confirm::Params {
            title: CONFIRM_TITLE.get().as_ref().unwrap(),
            body: CONFIRM_BODY.get().as_ref().unwrap(),
            accept_only: true,
            ..Default::default()
        });

        CONFIRM_STATE
            .get()
            .write(TaskState::Running(crate::main_loop::spawn(
                confirm::confirm(CONFIRM_PARAMS.get().as_ref().unwrap()),
            )));
    }
}

/// Returns true if there was a result.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_workflow_unlock_poll(result_out: &mut bool) -> bool {
    unsafe {
        match UNLOCK_STATE.get().as_ref().unwrap() {
            TaskState::Running(recv) => {
                match recv.try_recv() {
                    Ok(result) => {
                        UNLOCK_STATE.get().write(TaskState::Nothing);
                        match result {
                            Ok(()) => *result_out = true,
                            Err(()) => *result_out = false,
                        }
                        true
                    }
                    Err(TryRecvError::Empty) => false, // No result yet
                    Err(TryRecvError::Closed) => panic!("internal error"),
                }
            }
            TaskState::Nothing => panic!("polled non-existing future"),
        }
    }
}

/// Returns true if there was a result.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_workflow_confirm_poll(result_out: &mut bool) -> bool {
    unsafe {
        match CONFIRM_STATE.get().as_ref().unwrap() {
            TaskState::Running(recv) => {
                match recv.try_recv() {
                    Ok(result) => {
                        CONFIRM_STATE.get().write(TaskState::Nothing);
                        *result_out = result.is_ok();
                        true
                    }
                    Err(TryRecvError::Empty) => false, //No result yet
                    Err(TryRecvError::Closed) => panic!("internal error"),
                }
            }
            _ => false,
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_workflow_abort_current() {
    unsafe {
        UNLOCK_STATE.get().write(TaskState::Nothing);
        CONFIRM_STATE.get().write(TaskState::Nothing);
    }
}
