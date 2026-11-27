// SPDX-License-Identifier: Apache-2.0

//! Minimal wrapper over LVGL's animation engine: animate a widget's translate style, for short
//! move-into-place transitions.
//!
//! The animation holds a raw pointer to the widget, but LVGL deletes an object's running
//! animations when the object is deleted, so the pointer cannot dangle.

use crate::ObjExt;
use crate::ffi;

unsafe extern "C" fn set_translate_x(var: *mut core::ffi::c_void, value: i32) {
    unsafe { ffi::lv_obj_set_style_translate_x(var as *mut ffi::lv_obj_t, value, 0) };
}

unsafe extern "C" fn set_translate_y(var: *mut core::ffi::c_void, value: i32) {
    unsafe { ffi::lv_obj_set_style_translate_y(var as *mut ffi::lv_obj_t, value, 0) };
}

fn start(
    obj: &impl ObjExt,
    exec_cb: unsafe extern "C" fn(*mut core::ffi::c_void, i32),
    from: i32,
    to: i32,
    duration_ms: u32,
) {
    unsafe {
        let mut anim: ffi::lv_anim_t = core::mem::zeroed();
        ffi::lv_anim_init(&mut anim);
        ffi::lv_anim_set_var(&mut anim, obj.as_ptr() as *mut core::ffi::c_void);
        ffi::lv_anim_set_exec_cb(&mut anim, Some(exec_cb));
        ffi::lv_anim_set_values(&mut anim, from, to);
        ffi::lv_anim_set_duration(&mut anim, duration_ms);
        ffi::lv_anim_set_path_cb(&mut anim, Some(ffi::lv_anim_path_ease_out));
        // `lv_anim_start` copies the descriptor; it also deletes a running animation of the
        // same object and callback, so restarting a move mid-flight is safe.
        ffi::lv_anim_start(&anim);
    }
}

/// Animates `obj`'s translate style from `from` to `to` (x, y pairs) over `duration_ms`, easing
/// out. The translate is set to `from` immediately, so the first rendered frame is already at
/// the start position. Starting a new translate animation on the same object replaces a running
/// one.
pub fn animate_translate(obj: &impl ObjExt, from: (i32, i32), to: (i32, i32), duration_ms: u32) {
    obj.set_style_translate_x(from.0, 0);
    obj.set_style_translate_y(from.1, 0);
    start(obj, set_translate_x, from.0, to.0, duration_ms);
    start(obj, set_translate_y, from.1, to.1, duration_ms);
}
