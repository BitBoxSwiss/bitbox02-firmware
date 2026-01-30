// SPDX-License-Identifier: Apache-2.0

use alloc::boxed::Box;
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
