// SPDX-License-Identifier: Apache-2.0

pub fn init() {
    unsafe { bitbox02_sys::qtouch_init() }
}

pub fn process() {
    unsafe { bitbox02_sys::qtouch_process() }
}

pub fn is_scroller_active(scroller: u16) -> bool {
    unsafe { bitbox02_sys::qtouch_is_scroller_active(scroller) }
}

pub fn get_scroller_position(scroller: u16) -> u16 {
    unsafe { bitbox02_sys::qtouch_get_scroller_position(scroller) }
}

pub fn measurement_done() -> bool {
    unsafe { bitbox02_sys::measurement_done_touch }
}
