// SPDX-License-Identifier: Apache-2.0

use alloc::boxed::Box;
use alloc::rc::Rc;
use core::cell::RefCell;
use core::ffi::c_void;
use core::ptr::NonNull;

use crate::{LvHandle, class, ffi};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LvTextError {
    ContainsNul,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LvMapError {
    ContainsNul,
    EventRegistrationFailed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LvEventRegistrationError {
    RegistrationFailed,
}

type EventCallback = RefCell<Box<dyn FnMut() + 'static>>;

unsafe extern "C" fn on_delete_drop_attachment<T>(e: *mut ffi::lv_event_t) {
    if unsafe { ffi::lv_event_get_code(e) } != ffi::lv_event_code_t::LV_EVENT_DELETE {
        return;
    }

    let user_data = unsafe { ffi::lv_event_get_user_data(e) };
    if user_data.is_null() {
        return;
    }

    drop(unsafe { Box::from_raw(user_data as *mut T) });
}

unsafe extern "C" fn event_callback_trampoline(e: *mut ffi::lv_event_t) {
    let user_data = unsafe { ffi::lv_event_get_user_data(e) };
    if user_data.is_null() {
        return;
    }

    let callback_ptr = user_data.cast::<EventCallback>().cast_const();
    unsafe {
        Rc::increment_strong_count(callback_ptr);
    }
    let callback = unsafe { Rc::from_raw(callback_ptr) };
    let mut callback = callback.borrow_mut();
    callback.as_mut()();
}

unsafe extern "C" fn on_delete_drop_event_callback(e: *mut ffi::lv_event_t) {
    if unsafe { ffi::lv_event_get_code(e) } != ffi::lv_event_code_t::LV_EVENT_DELETE {
        return;
    }

    let user_data = unsafe { ffi::lv_event_get_user_data(e) };
    if user_data.is_null() {
        return;
    }

    drop(unsafe { Rc::from_raw(user_data.cast::<EventCallback>().cast_const()) });
}

pub(crate) fn attach_to_object<T, C: class::LvClass>(
    obj: &LvHandle<C>,
    attachment: T,
) -> Result<NonNull<T>, ()> {
    let attachment = Box::new(attachment);
    let attachment_ptr = Box::into_raw(attachment);
    let event = unsafe {
        ffi::lv_obj_add_event_cb(
            obj.as_ptr(),
            Some(on_delete_drop_attachment::<T>),
            ffi::lv_event_code_t::LV_EVENT_DELETE,
            attachment_ptr as *mut c_void,
        )
    };
    if event.is_null() {
        drop(unsafe { Box::from_raw(attachment_ptr) });
        return Err(());
    }
    Ok(unsafe { NonNull::new_unchecked(attachment_ptr) })
}

pub(crate) fn add_event_cb<F>(
    obj: *mut ffi::lv_obj_t,
    filter: ffi::lv_event_code_t,
    cb: F,
) -> Result<(), LvEventRegistrationError>
where
    F: FnMut() + 'static,
{
    let callback: Rc<EventCallback> = Rc::new(RefCell::new(Box::new(cb)));
    let callback_ptr = Rc::into_raw(callback);
    let user_data = callback_ptr.cast_mut().cast::<c_void>();

    let event_dsc = unsafe {
        ffi::lv_obj_add_event_cb(obj, Some(event_callback_trampoline), filter, user_data)
    };
    if event_dsc.is_null() {
        drop(unsafe { Rc::from_raw(callback_ptr) });
        return Err(LvEventRegistrationError::RegistrationFailed);
    }

    let cleanup_dsc = unsafe {
        ffi::lv_obj_add_event_cb(
            obj,
            Some(on_delete_drop_event_callback),
            ffi::lv_event_code_t::LV_EVENT_DELETE,
            user_data,
        )
    };
    if cleanup_dsc.is_null() {
        let removed = unsafe { ffi::lv_obj_remove_event_dsc(obj, event_dsc) };
        assert!(removed, "failed to roll back event registration");
        drop(unsafe { Rc::from_raw(callback_ptr) });
        return Err(LvEventRegistrationError::RegistrationFailed);
    }

    Ok(())
}
