// SPDX-License-Identifier: Apache-2.0

use core::ptr::NonNull;

use super::arc::{ArcExt, LvArc};
use crate::{LvHandle, LvObj, class, ffi};

pub type LvSpinner = LvHandle<class::SpinnerTag>;

pub trait SpinnerExt: ArcExt {
    fn set_anim_params(&self, spin_duration: u32, angle: u32) {
        unsafe { ffi::lv_spinner_set_anim_params(self.as_ptr(), spin_duration, angle) }
    }
}

impl LvHandle<class::SpinnerTag> {
    pub fn new<P: class::LvClass>(parent: &LvHandle<P>) -> Option<Self> {
        NonNull::new(unsafe { ffi::lv_spinner_create(parent.as_ptr()) }).map(LvHandle::from_ptr)
    }

    pub fn to_arc(self) -> LvArc {
        self.cast()
    }

    pub fn to_obj(self) -> LvObj {
        self.cast()
    }
}

impl SpinnerExt for LvHandle<class::SpinnerTag> {}
