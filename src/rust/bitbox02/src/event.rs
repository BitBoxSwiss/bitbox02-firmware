// SPDX-License-Identifier: Apache-2.0

pub use bitbox02_sys::event_slider_data_t;
pub use bitbox02_sys::event_t;
pub use bitbox02_sys::event_types;

pub fn emit_event(event: &event_t) {
    unsafe { bitbox02_sys::emit_event(event as *const _) }
}
