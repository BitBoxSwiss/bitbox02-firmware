// SPDX-License-Identifier: Apache-2.0

use alloc::boxed::Box;
use core::ffi::{CStr, c_char};

use grounded::uninit::GroundedCell;

use crate::{LvLogLevel, ffi};

type LogCb = Box<dyn Fn(LvLogLevel, &CStr) + Send + Sync + 'static>;

static LOG_CB: GroundedCell<Option<LogCb>> = GroundedCell::const_init();

extern "C" fn print_cb_trampoline(level: LvLogLevel, buf: *const c_char) {
    if buf.is_null() {
        return;
    }

    // SAFETY: LVGL promises buf is a valid NUL-terminated string for the duration of the call.
    let cstr = unsafe { CStr::from_ptr(buf) };

    if let Some(log_cb) = unsafe { &mut *LOG_CB.get() } {
        log_cb(level, cstr);
    }
}

pub fn register_print_cb<F>(f: F)
where
    F: Fn(LvLogLevel, &CStr) + Send + Sync + 'static,
{
    let log_cb = unsafe { &mut *LOG_CB.get() };
    if log_cb.is_some() {
        panic!("Only call once");
    }
    log_cb.replace(Box::new(f));
    unsafe { ffi::lv_log_register_print_cb(Some(print_cb_trampoline)) }
}
