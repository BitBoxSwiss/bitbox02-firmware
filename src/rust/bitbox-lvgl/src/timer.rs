// SPDX-License-Identifier: Apache-2.0

use alloc::{boxed::Box, rc::Rc};
use core::{cell::RefCell, ffi::c_void, ptr::NonNull};

use crate::ffi;

pub fn handler() {
    unsafe {
        ffi::lv_timer_handler();
    }
}

type TimerCallback = RefCell<Box<dyn FnMut() + 'static>>;

pub struct LvTimer {
    raw: NonNull<ffi::lv_timer_t>,
    _callback: Rc<TimerCallback>,
}

unsafe extern "C" fn timer_cb_trampoline(timer: *mut ffi::lv_timer_t) {
    let user_data = unsafe { ffi::lv_timer_get_user_data(timer) };
    if user_data.is_null() {
        return;
    }

    let callback_ptr = user_data.cast::<TimerCallback>().cast_const();
    // Keep the callback alive for the duration of this call, even if the timer drops itself.
    unsafe {
        Rc::increment_strong_count(callback_ptr);
    }
    let callback = unsafe { Rc::from_raw(callback_ptr) };
    let mut callback = callback.borrow_mut();
    callback.as_mut()();
}

impl LvTimer {
    /// Creates a timer that invokes `cb` every `period_ms`.
    ///
    /// LVGL's auto-delete is disabled so the timer lifetime is fully owned by this Rust wrapper
    /// and cleanup remains explicit through `Drop`.
    pub fn new<F>(period_ms: u32, cb: F) -> Option<Self>
    where
        F: FnMut() + 'static,
    {
        let callback: Rc<TimerCallback> = Rc::new(RefCell::new(Box::new(cb)));
        let callback_ptr = Rc::as_ptr(&callback);
        let raw = NonNull::new(unsafe {
            ffi::lv_timer_create(
                Some(timer_cb_trampoline),
                period_ms,
                callback_ptr.cast_mut().cast::<c_void>(),
            )
        })?;
        unsafe {
            ffi::lv_timer_set_auto_delete(raw.as_ptr(), false);
        }
        Some(Self {
            raw,
            _callback: callback,
        })
    }

    pub fn pause(&self) {
        unsafe {
            ffi::lv_timer_pause(self.raw.as_ptr());
        }
    }

    pub fn resume(&self) {
        unsafe {
            ffi::lv_timer_resume(self.raw.as_ptr());
        }
    }

    pub fn set_period(&self, period_ms: u32) {
        unsafe {
            ffi::lv_timer_set_period(self.raw.as_ptr(), period_ms);
        }
    }

    pub fn ready(&self) {
        unsafe {
            ffi::lv_timer_ready(self.raw.as_ptr());
        }
    }

    pub fn set_repeat_count(&self, repeat_count: i32) {
        unsafe {
            ffi::lv_timer_set_repeat_count(self.raw.as_ptr(), repeat_count);
        }
    }
}

impl Drop for LvTimer {
    fn drop(&mut self) {
        unsafe {
            ffi::lv_timer_delete(self.raw.as_ptr());
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use alloc::rc::Rc;
    use core::cell::{Cell, RefCell};

    use super::*;

    #[test]
    fn test_drop_during_callback_is_safe() {
        let _lock = crate::test_util::lock_and_init();

        let called = Rc::new(Cell::new(false));
        let called_cb = Rc::clone(&called);
        let timer_slot = Rc::new(RefCell::new(None));
        let timer_slot_cb = Rc::clone(&timer_slot);

        let timer = LvTimer::new(0, move || {
            called_cb.set(true);
            let dropped = timer_slot_cb.borrow_mut().take();
            drop(dropped);
        })
        .unwrap();
        *timer_slot.borrow_mut() = Some(timer);

        handler();

        assert!(called.get());
        assert!(timer_slot.borrow().is_none());
    }
}
