// SPDX-License-Identifier: Apache-2.0

use core::ptr::NonNull;

use crate::{LvHandle, LvObj, ObjExt, class, ffi};

pub type LvButton = LvHandle<class::ButtonTag>;

pub trait ButtonExt: ObjExt {}

impl LvHandle<class::ButtonTag> {
    pub fn new<P: class::LvClass>(parent: &LvHandle<P>) -> Option<Self> {
        NonNull::new(unsafe { ffi::lv_button_create(parent.as_ptr()) }).map(LvHandle::from_ptr)
    }

    pub fn to_obj(self) -> LvObj {
        self.cast()
    }
}

impl ButtonExt for LvHandle<class::ButtonTag> {}
