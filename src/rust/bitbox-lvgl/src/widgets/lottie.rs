// SPDX-License-Identifier: Apache-2.0

use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::vec;
use core::cell::RefCell;
use core::ffi::c_void;
use core::ptr::NonNull;

use super::canvas::{CanvasExt, LvCanvas};
use super::image::ImageExt;
use super::obj::ObjExt;
use crate::{LvHandle, LvObj, class, ffi};

#[derive(Debug, PartialEq, Eq)]
pub struct LvLottie {
    inner: LvHandle<class::CanvasTag>,
}

type AnimationCallback = RefCell<Box<dyn FnMut() + 'static>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LvLottieCreateError {
    InvalidDimensions,
    CreateFailed,
    EventRegistrationFailed,
}

unsafe extern "C" fn animation_completed_trampoline(anim: *mut ffi::lv_anim_t) {
    let user_data = unsafe { ffi::lv_anim_get_user_data(anim) };
    if user_data.is_null() {
        return;
    }

    let callback_ptr = user_data.cast::<AnimationCallback>().cast_const();
    unsafe {
        Rc::increment_strong_count(callback_ptr);
    }
    let callback = unsafe { Rc::from_raw(callback_ptr) };
    let mut callback = callback.borrow_mut();
    callback.as_mut()();
}

unsafe extern "C" fn animation_deleted_trampoline(anim: *mut ffi::lv_anim_t) {
    let user_data = unsafe { ffi::lv_anim_get_user_data(anim) };
    if user_data.is_null() {
        return;
    }

    drop(unsafe { Rc::from_raw(user_data.cast::<AnimationCallback>().cast_const()) });
}

pub trait LottieExt: CanvasExt {
    fn set_src_data(&self, src: &'static [u8]) {
        unsafe {
            ffi::lv_lottie_set_src_data(self.as_ptr(), src.as_ptr().cast::<c_void>(), src.len())
        }
    }

    fn pause(&self) {
        unsafe { ffi::lv_anim_pause(self.animation()) }
    }

    fn resume(&self) {
        unsafe { ffi::lv_anim_resume(self.animation()) }
    }

    fn set_repeat_count(&self, repeat_count: u32) {
        unsafe { ffi::lv_anim_set_repeat_count(self.animation(), repeat_count) }
    }

    fn set_completed_cb<F>(&self, cb: F)
    where
        F: FnMut() + 'static,
    {
        let callback: Rc<AnimationCallback> = Rc::new(RefCell::new(Box::new(cb)));
        let user_data = Rc::into_raw(callback).cast_mut().cast::<c_void>();
        unsafe {
            ffi::lv_anim_set_user_data(self.animation(), user_data);
            ffi::lv_anim_set_completed_cb(self.animation(), Some(animation_completed_trampoline));
            ffi::lv_anim_set_deleted_cb(self.animation(), Some(animation_deleted_trampoline));
        }
    }

    fn animation(&self) -> *mut ffi::lv_anim_t {
        unsafe { ffi::lv_lottie_get_anim(self.as_ptr()) }
    }
}

impl LvLottie {
    pub fn new<P: class::LvClass>(
        parent: &LvHandle<P>,
        width: u32,
        height: u32,
    ) -> Result<Self, LvLottieCreateError> {
        let Ok(width_i32) = i32::try_from(width) else {
            return Err(LvLottieCreateError::InvalidDimensions);
        };
        let Ok(height_i32) = i32::try_from(height) else {
            return Err(LvLottieCreateError::InvalidDimensions);
        };
        let Some(pixel_count) = width.checked_mul(height) else {
            return Err(LvLottieCreateError::InvalidDimensions);
        };
        let Ok(pixel_count) = usize::try_from(pixel_count) else {
            return Err(LvLottieCreateError::InvalidDimensions);
        };

        let Some(lottie) = NonNull::new(unsafe { ffi::lv_lottie_create(parent.as_ptr()) }) else {
            return Err(LvLottieCreateError::CreateFailed);
        };
        let lottie = LvLottie {
            inner: LvHandle::from_ptr(lottie),
        };

        let attachment = super::util::attach_to_object(&lottie.inner, vec![0u32; pixel_count])
            .map_err(|_| LvLottieCreateError::EventRegistrationFailed)?;

        unsafe {
            ffi::lv_lottie_set_buffer(
                lottie.as_ptr(),
                width_i32,
                height_i32,
                (*attachment.as_ptr()).as_mut_ptr().cast::<c_void>(),
            )
        };

        Ok(lottie)
    }

    pub fn to_canvas(self) -> LvCanvas {
        self.inner
    }

    pub fn to_obj(self) -> LvObj {
        self.inner.cast()
    }
}

impl ObjExt for LvLottie {
    fn as_ptr(&self) -> *mut ffi::lv_obj_t {
        self.inner.as_ptr()
    }
}

impl ImageExt for LvLottie {}
impl CanvasExt for LvLottie {}
impl LottieExt for LvLottie {}
