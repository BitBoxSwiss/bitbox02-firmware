// SPDX-License-Identifier: Apache-2.0

//! This module is a quick workaround to use async workflows from U2F/FIDO2, where the root of the
//! usb message proessing is not ported to Rust. If that happens, the `async_usb` module can be
//! used and this can be deleted.

extern crate alloc;

use alloc::boxed::Box;
use alloc::string::String;
use bitbox_hal::ui::{ConfirmParams, UserAbort};
use bitbox_hal::{Hal, Ui};
use core::ffi::CStr;
use core::sync::atomic::{AtomicU32, Ordering};
use grounded::const_init::ConstInit;
use grounded::uninit::GroundedCell;

enum TaskState<O> {
    Nothing,
    Running(u32),
    ResultAvailable(O),
}

impl<O> ConstInit for TaskState<O> {
    const VAL: Self = Self::Nothing;
}

static NEXT_TASK_TOKEN: AtomicU32 = AtomicU32::new(0);
static ACTIVE_WORKFLOW_COUNT: AtomicU32 = AtomicU32::new(0);
static UNLOCK_STATE: GroundedCell<TaskState<Result<(), ()>>> = GroundedCell::const_init();
static CONFIRM_STATE: GroundedCell<TaskState<Result<(), UserAbort>>> = GroundedCell::const_init();
static BITBOX02_HAL: GroundedCell<crate::HalImpl> = GroundedCell::const_init();

struct ActiveWorkflowGuard;

impl ActiveWorkflowGuard {
    fn new() -> Self {
        ACTIVE_WORKFLOW_COUNT.fetch_add(1, Ordering::Relaxed);
        Self
    }
}

impl Drop for ActiveWorkflowGuard {
    fn drop(&mut self) {
        ACTIVE_WORKFLOW_COUNT.fetch_sub(1, Ordering::Relaxed);
    }
}

/// Returns whether a detached U2F workflow future is still alive.
#[unsafe(no_mangle)]
pub extern "C" fn rust_workflow_u2f_is_active() -> bool {
    ACTIVE_WORKFLOW_COUNT.load(Ordering::Relaxed) != 0
}

fn next_task_token() -> u32 {
    NEXT_TASK_TOKEN.fetch_add(1, Ordering::Relaxed)
}

/// # Safety
/// Must not be called concurrently or reentrantly with other operations that mutate unlock
/// workflow state in this module.
/// Callers must guarantee single-threaded access to this workflow.
unsafe fn complete_unlock(token: u32, result: Result<(), ()>) {
    unsafe {
        if let TaskState::Running(current_token) = UNLOCK_STATE.get().as_ref().unwrap()
            && *current_token == token
        {
            UNLOCK_STATE.get().write(TaskState::ResultAvailable(result));
        }
    }
}

/// # Safety
/// Must not be called concurrently or reentrantly with other operations that mutate confirm
/// workflow state in this module.
/// Callers must guarantee single-threaded access to this workflow.
unsafe fn complete_confirm(token: u32, result: Result<(), UserAbort>) {
    unsafe {
        if let TaskState::Running(current_token) = CONFIRM_STATE.get().as_ref().unwrap()
            && *current_token == token
        {
            CONFIRM_STATE
                .get()
                .write(TaskState::ResultAvailable(result));
        }
    }
}

/// # Safety
/// Must be called from the same single-threaded, non-reentrant execution context as all other
/// U2F workflow C API calls. In particular, do not call this from interrupts or from multiple
/// threads.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_workflow_spawn_unlock() {
    let token = next_task_token();
    let active_workflow_guard = ActiveWorkflowGuard::new();
    unsafe {
        UNLOCK_STATE.get().write(TaskState::Running(token));
    }
    bitbox02_rust::main_loop::spawn(Box::pin(async move {
        let _active_workflow_guard = active_workflow_guard;
        let result = unsafe {
            bitbox02_rust::workflow::unlock::unlock(BITBOX02_HAL.get().as_mut().unwrap()).await
        };
        unsafe { complete_unlock(token, result) };
    }));
}

/// # Safety
/// `title` and `body` must be valid non-null pointers to NUL-terminated UTF-8 strings, readable
/// for the duration of this call.
///
/// This must be called from the same single-threaded, non-reentrant execution context as all
/// other U2F workflow C API calls (no interrupts/multi-threaded callers).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_workflow_spawn_confirm(
    title: *const core::ffi::c_char,
    body: *const core::ffi::c_char,
) {
    let title: String = unsafe { CStr::from_ptr(title).to_str().unwrap().into() };
    let body: String = unsafe { CStr::from_ptr(body).to_str().unwrap().into() };
    let token = next_task_token();
    let active_workflow_guard = ActiveWorkflowGuard::new();
    unsafe {
        CONFIRM_STATE.get().write(TaskState::Running(token));
    }
    bitbox02_rust::main_loop::spawn(Box::pin(async move {
        let _active_workflow_guard = active_workflow_guard;
        let params = ConfirmParams {
            title: &title,
            body: &body,
            accept_only: true,
            ..Default::default()
        };
        let result = unsafe {
            BITBOX02_HAL
                .get()
                .as_mut()
                .unwrap()
                .ui()
                .confirm(&params)
                .await
        };
        unsafe { complete_confirm(token, result) };
    }));
}

/// Returns true if there was a result.
///
/// # Safety
/// `result_out` must be a valid, non-null writable pointer to a `bool` for the duration of this
/// call.
///
/// This must be called from the same single-threaded, non-reentrant execution context as all
/// other U2F workflow C API calls (no interrupts/multi-threaded callers).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_workflow_unlock_poll(result_out: &mut bool) -> bool {
    unsafe {
        match UNLOCK_STATE.get().as_ref().unwrap() {
            TaskState::ResultAvailable(result) => {
                *result_out = result.is_ok();
                UNLOCK_STATE.get().write(TaskState::Nothing);
                true
            }
            TaskState::Running(_) => false,
            TaskState::Nothing => panic!("polled non-existing future"),
        }
    }
}

/// Returns true if there was a result.
///
/// # Safety
/// `result_out` must be a valid, non-null writable pointer to a `bool` for the duration of this
/// call.
///
/// This must be called from the same single-threaded, non-reentrant execution context as all
/// other U2F workflow C API calls (no interrupts/multi-threaded callers).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_workflow_confirm_poll(result_out: &mut bool) -> bool {
    unsafe {
        match CONFIRM_STATE.get().as_ref().unwrap() {
            TaskState::ResultAvailable(result) => {
                CONFIRM_STATE.get().write(TaskState::Nothing);
                *result_out = result.is_ok();
                true
            }
            TaskState::Running(_) => false,
            TaskState::Nothing => false,
        }
    }
}

/// # Safety
/// Must be called from the same single-threaded, non-reentrant execution context as all other
/// U2F workflow C API calls (no interrupts/multi-threaded callers).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_workflow_abort_current() {
    unsafe {
        UNLOCK_STATE.get().write(TaskState::Nothing);
        CONFIRM_STATE.get().write(TaskState::Nothing);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_workflow_u2f_is_active() {
        assert!(!rust_workflow_u2f_is_active());

        let first_guard = ActiveWorkflowGuard::new();
        assert!(rust_workflow_u2f_is_active());

        {
            let _second_guard = ActiveWorkflowGuard::new();
            assert!(rust_workflow_u2f_is_active());
        }
        assert!(rust_workflow_u2f_is_active());

        drop(first_guard);
        assert!(!rust_workflow_u2f_is_active());
    }
}
