// SPDX-License-Identifier: Apache-2.0

use alloc::vec::Vec;
use core::ffi::CStr;
use core::ptr::NonNull;

use super::util::{self, CStringArrayAttachment};
use crate::{LvButtonmatrixCtrl, LvHandle, LvObj, ObjExt, class, ffi};

pub type LvButtonmatrixMapError = super::LvMapError;
pub type LvButtonmatrix = LvHandle<class::ButtonmatrixTag>;

pub trait ButtonmatrixExt: ObjExt {
    fn set_map(&self, map: &[&str]) -> Result<(), LvButtonmatrixMapError> {
        let attachment = CStringArrayAttachment::new(map)?;
        let attachment = util::attach_to_object(self.as_ptr(), attachment)
            .map_err(|_| LvButtonmatrixMapError::EventRegistrationFailed)?;
        unsafe { ffi::lv_buttonmatrix_set_map(self.as_ptr(), (*attachment.as_ptr()).as_ptr()) }
        Ok(())
    }

    fn set_ctrl_map(&self, ctrl_map: &[LvButtonmatrixCtrl]) {
        unsafe { ffi::lv_buttonmatrix_set_ctrl_map(self.as_ptr(), ctrl_map.as_ptr()) }
    }

    fn set_selected_button(&self, button_id: u32) {
        unsafe { ffi::lv_buttonmatrix_set_selected_button(self.as_ptr(), button_id) }
    }

    fn set_button_ctrl(&self, button_id: u32, ctrl: LvButtonmatrixCtrl) {
        unsafe { ffi::lv_buttonmatrix_set_button_ctrl(self.as_ptr(), button_id, ctrl) }
    }

    fn clear_button_ctrl(&self, button_id: u32, ctrl: LvButtonmatrixCtrl) {
        unsafe { ffi::lv_buttonmatrix_clear_button_ctrl(self.as_ptr(), button_id, ctrl) }
    }

    fn set_button_ctrl_all(&self, ctrl: LvButtonmatrixCtrl) {
        unsafe { ffi::lv_buttonmatrix_set_button_ctrl_all(self.as_ptr(), ctrl) }
    }

    fn clear_button_ctrl_all(&self, ctrl: LvButtonmatrixCtrl) {
        unsafe { ffi::lv_buttonmatrix_clear_button_ctrl_all(self.as_ptr(), ctrl) }
    }

    fn set_button_width(&self, button_id: u32, width: u32) {
        unsafe { ffi::lv_buttonmatrix_set_button_width(self.as_ptr(), button_id, width) }
    }

    fn set_one_checked(&self, enable: bool) {
        unsafe { ffi::lv_buttonmatrix_set_one_checked(self.as_ptr(), enable) }
    }

    fn get_map(&self) -> Vec<&CStr> {
        util::cstr_array_from_ptr(unsafe { ffi::lv_buttonmatrix_get_map(self.as_ptr()) })
    }

    fn get_selected_button(&self) -> u32 {
        unsafe { ffi::lv_buttonmatrix_get_selected_button(self.as_ptr()) }
    }

    fn get_button_text(&self, button_id: u32) -> Option<&CStr> {
        util::optional_cstr_from_ptr(unsafe {
            ffi::lv_buttonmatrix_get_button_text(self.as_ptr(), button_id)
        })
    }

    fn has_button_ctrl(&self, button_id: u32, ctrl: LvButtonmatrixCtrl) -> bool {
        unsafe { ffi::lv_buttonmatrix_has_button_ctrl(self.as_ptr(), button_id, ctrl) }
    }

    fn get_one_checked(&self) -> bool {
        unsafe { ffi::lv_buttonmatrix_get_one_checked(self.as_ptr()) }
    }
}

impl LvHandle<class::ButtonmatrixTag> {
    pub fn new<P: class::LvClass>(parent: &LvHandle<P>) -> Option<Self> {
        NonNull::new(unsafe { ffi::lv_buttonmatrix_create(parent.as_ptr()) })
            .map(LvHandle::from_ptr)
    }

    pub fn to_obj(self) -> LvObj {
        self.cast()
    }
}

impl ButtonmatrixExt for LvHandle<class::ButtonmatrixTag> {}
impl ButtonmatrixExt for LvHandle<class::KeyboardTag> {}
