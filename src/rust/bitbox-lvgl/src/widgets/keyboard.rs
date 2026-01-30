// SPDX-License-Identifier: Apache-2.0

use alloc::vec::Vec;
use core::ffi::CStr;
use core::ptr::NonNull;

use super::buttonmatrix::{ButtonmatrixExt, LvButtonmatrix, LvButtonmatrixMapEntry, validate_map};
use super::textarea::LvTextarea;
use crate::{LvButtonmatrixCtrl, LvHandle, LvKeyboardMode, LvObj, class, ffi};
use util::strings::{cstr_array_from_ptr, optional_cstr_from_ptr};

pub type LvKeyboard = LvHandle<class::KeyboardTag>;
pub type LvKeyboardMapEntry = LvButtonmatrixMapEntry;

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

    /// `map` must be a `'static` LVGL keyboard map terminated by an empty string.
    fn set_map(
        &self,
        mode: LvKeyboardMode,
        map: &'static [LvKeyboardMapEntry],
        ctrl_map: &'static [LvButtonmatrixCtrl],
    ) {
        let button_count = validate_map(map);
        assert_eq!(
            ctrl_map.len(),
            button_count,
            "LVGL keyboard control map length must match the number of buttons in the map"
        );
        unsafe {
            ffi::lv_keyboard_set_map(self.as_ptr(), mode, map.as_ptr().cast(), ctrl_map.as_ptr())
        }
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
        unsafe {
            // LVGL returns either null or a valid keyboard map array terminated by a null pointer
            // or empty string sentinel, with entries alive for the duration of this borrow.
            cstr_array_from_ptr(ffi::lv_keyboard_get_map_array(self.as_ptr()))
        }
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
        unsafe {
            // LVGL returns either null or a valid NUL-terminated keyboard button text pointer
            // owned by the object and alive for the duration of this borrow.
            optional_cstr_from_ptr(ffi::lv_keyboard_get_button_text(self.as_ptr(), button_id))
        }
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

#[cfg(test)]
mod tests {
    use alloc::boxed::Box;
    use core::{mem, ptr::NonNull};

    use super::{KeyboardExt, LvKeyboard, LvKeyboardMapEntry};
    use crate::{LvButtonmatrixCtrl, LvHandle, class};

    fn dummy_keyboard() -> LvKeyboard {
        LvHandle::<class::KeyboardTag>::from_ptr(NonNull::dangling())
    }

    #[test]
    #[should_panic(
        expected = "LVGL keyboard control map length must match the number of buttons in the map"
    )]
    fn test_set_map_rejects_incorrect_ctrl_map_length() {
        let keyboard = dummy_keyboard();
        let map: &'static [LvKeyboardMapEntry] = Box::leak(Box::new([
            LvKeyboardMapEntry::new(c"1"),
            LvKeyboardMapEntry::new(c"2"),
            LvKeyboardMapEntry::new(c""),
        ]));
        let ctrl_map: &'static [LvButtonmatrixCtrl] =
            Box::leak(Box::new([unsafe { mem::zeroed() }]));

        keyboard.set_map(unsafe { mem::zeroed() }, map, ctrl_map);
    }
}
