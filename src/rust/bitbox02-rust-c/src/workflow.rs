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
use bitbox02_rust::bb02_async::{spin, Task};
use core::task::Poll;

enum TaskState<'a, O> {
    Nothing,
    Running(Task<'a, O>),
    ResultAvailable(O),
}

static mut UNLOCK_STATE: TaskState<'static, Result<(), ()>> = TaskState::Nothing;

#[no_mangle]
pub unsafe extern "C" fn rust_workflow_spawn_unlock() {
    UNLOCK_STATE = TaskState::Running(Box::pin(bitbox02_rust::workflow::unlock::unlock()));
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

#[no_mangle]
pub unsafe extern "C" fn rust_workflow_abort_current() {
    UNLOCK_STATE = TaskState::Nothing;
}
