// SPDX-License-Identifier: Apache-2.0

use alloc::boxed::Box;
use alloc::ffi::CString;
use alloc::vec::Vec;
use core::ffi::{CStr, c_char, c_void};
use core::ptr::NonNull;

use crate::ffi;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LvTextError {
    ContainsNul,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LvMapError {
    ContainsNul,
    EventRegistrationFailed,
}

pub(crate) fn cstring(value: &str) -> Result<CString, LvTextError> {
    CString::new(value).map_err(|_| LvTextError::ContainsNul)
}

pub(crate) fn optional_cstr_from_ptr<'a>(ptr: *const c_char) -> Option<&'a CStr> {
    if ptr.is_null() {
        None
    } else {
        Some(unsafe { CStr::from_ptr(ptr) })
    }
}

pub(crate) fn cstr_array_from_ptr<'a>(ptr: *const *const c_char) -> Vec<&'a CStr> {
    if ptr.is_null() {
        return Vec::new();
    }

    let mut result = Vec::new();
    let mut index = 0;
    loop {
        let entry = unsafe { *ptr.add(index) };
        if entry.is_null() {
            break;
        }
        let entry = unsafe { CStr::from_ptr(entry) };
        if entry.to_bytes().is_empty() {
            break;
        }
        result.push(entry);
        index += 1;
    }
    result
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

pub(crate) fn attach_to_object<T>(
    obj: *mut ffi::lv_obj_t,
    attachment: T,
) -> Result<NonNull<T>, ()> {
    let attachment = Box::new(attachment);
    let attachment_ptr = Box::into_raw(attachment);
    let event = unsafe {
        ffi::lv_obj_add_event_cb(
            obj,
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

pub(crate) struct CStringArrayAttachment {
    _strings: Vec<CString>,
    ptrs: Vec<*const c_char>,
}

impl CStringArrayAttachment {
    pub(crate) fn new(entries: &[&str]) -> Result<Self, LvMapError> {
        let needs_terminator = entries.last().copied() != Some("");
        let mut strings = Vec::with_capacity(entries.len() + usize::from(needs_terminator));
        for entry in entries {
            strings.push(CString::new(*entry).map_err(|_| LvMapError::ContainsNul)?);
        }
        if needs_terminator {
            strings.push(CString::new("").unwrap());
        }

        let ptrs = strings.iter().map(|entry| entry.as_ptr()).collect();
        Ok(Self {
            _strings: strings,
            ptrs,
        })
    }

    pub(crate) fn as_ptr(&self) -> *const *const c_char {
        self.ptrs.as_ptr()
    }
}
