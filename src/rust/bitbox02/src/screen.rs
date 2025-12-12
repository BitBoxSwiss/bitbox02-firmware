// SPDX-License-Identifier: Apache-2.0

use crate::ui::ugui::UG_COLOR;
pub use bitbox02_sys::{SCREEN_HEIGHT, SCREEN_WIDTH};
use util::cell::SyncCell;

type PixelFn = fn(i16, i16, UG_COLOR);
type MirrorFn = fn(bool);
type ClearFn = fn();

static PIXEL_FN: SyncCell<Option<PixelFn>> = SyncCell::new(None);
static MIRROR_FN: SyncCell<Option<MirrorFn>> = SyncCell::new(None);
static CLEAR_FN: SyncCell<Option<ClearFn>> = SyncCell::new(None);

unsafe extern "C" fn _pixel_fn(x: i16, y: i16, c: UG_COLOR) {
    PIXEL_FN.read().as_ref().unwrap()(x, y, c);
}

unsafe extern "C" fn _clear_fn() {
    CLEAR_FN.read().as_ref().unwrap()();
}

unsafe extern "C" fn _mirror_fn(mirror: bool) {
    MIRROR_FN.read().as_ref().unwrap()(mirror);
}

/// Can only be called once
pub fn init(pixel_fn: PixelFn, mirror_fn: MirrorFn, clear_fn: ClearFn) {
    PIXEL_FN.write(Some(pixel_fn));
    MIRROR_FN.write(Some(mirror_fn));
    CLEAR_FN.write(Some(clear_fn));
    unsafe { bitbox02_sys::screen_init(Some(_pixel_fn), Some(_mirror_fn), Some(_clear_fn)) }
}

pub fn splash() {
    unsafe { bitbox02_sys::screen_splash() }
}

pub fn process() {
    unsafe { bitbox02_sys::screen_process() }
}
