// SPDX-License-Identifier: Apache-2.0

use alloc::vec::Vec;
use core::ffi::CStr;
use core::ptr::NonNull;

use super::buttonmatrix::{ButtonmatrixExt, LvButtonmatrix};
use super::textarea::LvTextarea;
use super::util::{self, CStringArrayAttachment};
use crate::{LvButtonmatrixCtrl, LvHandle, LvKeyboardMode, LvObj, class, ffi};

pub type LvKeyboardMapError = super::LvMapError;
pub type LvKeyboard = LvHandle<class::KeyboardTag>;

pub trait KeyboardExt: ButtonmatrixExt {
    fn set_textarea(&self, textarea: Option<&LvTextarea>) {
        unsafe {
            ffi::lv_keyboard_set_textarea(
                self.as_ptr(),
                textarea.map_or(core::ptr::null_mut(), LvHandle::as_ptr),
            )
        }
    }

    fn set_mode(&self, mode: LvKeyboardMode) {
        unsafe { ffi::lv_keyboard_set_mode(self.as_ptr(), mode) }
    }

    fn set_popovers(&self, enable: bool) {
        unsafe { ffi::lv_keyboard_set_popovers(self.as_ptr(), enable) }
    }

    fn set_map(
        &self,
        mode: LvKeyboardMode,
        map: &[&str],
        ctrl_map: &[LvButtonmatrixCtrl],
    ) -> Result<(), LvKeyboardMapError> {
        let attachment = CStringArrayAttachment::new(map)?;
        let attachment = util::attach_to_object(self.as_ptr(), attachment)
            .map_err(|_| LvKeyboardMapError::EventRegistrationFailed)?;
        unsafe {
            ffi::lv_keyboard_set_map(
                self.as_ptr(),
                mode,
                (*attachment.as_ptr()).as_ptr(),
                ctrl_map.as_ptr(),
            )
        }
        Ok(())
    }

    fn get_textarea(&self) -> Option<LvTextarea> {
        NonNull::new(unsafe { ffi::lv_keyboard_get_textarea(self.as_ptr()) })
            .map(LvHandle::from_ptr)
    }

    fn get_mode(&self) -> LvKeyboardMode {
        unsafe { ffi::lv_keyboard_get_mode(self.as_ptr()) }
    }

    fn get_popovers(&self) -> bool {
        unsafe { ffi::lv_keyboard_get_popovers(self.as_ptr()) }
    }

    fn get_map_array(&self) -> Vec<&CStr> {
        util::cstr_array_from_ptr(unsafe { ffi::lv_keyboard_get_map_array(self.as_ptr()) })
    }
}

impl LvHandle<class::KeyboardTag> {
    pub fn new<P: class::LvClass>(parent: &LvHandle<P>) -> Option<Self> {
        NonNull::new(unsafe { ffi::lv_keyboard_create(parent.as_ptr()) }).map(LvHandle::from_ptr)
    }

    pub fn get_selected_button(&self) -> u32 {
        unsafe { ffi::lv_keyboard_get_selected_button(self.as_ptr()) }
    }

    pub fn get_button_text(&self, button_id: u32) -> Option<&CStr> {
        util::optional_cstr_from_ptr(unsafe {
            ffi::lv_keyboard_get_button_text(self.as_ptr(), button_id)
        })
    }

    pub fn to_buttonmatrix(self) -> LvButtonmatrix {
        self.cast()
    }

    pub fn to_obj(self) -> LvObj {
        self.cast()
    }
}

impl KeyboardExt for LvHandle<class::KeyboardTag> {}

/// # Safety
/// `event` must point to a valid LVGL keyboard event.
pub unsafe fn keyboard_def_event_cb(event: *mut ffi::lv_event_t) {
    unsafe { ffi::lv_keyboard_def_event_cb(event) }
}
