// SPDX-License-Identifier: Apache-2.0

mod types;

#[cfg_attr(feature = "unit-testing", path = "ui/ui_stub.rs")]
#[cfg_attr(
    not(feature = "unit-testing"),
    cfg_attr(feature = "c-unit-testing", path = "ui/ui_stub_c_unit_tests.rs")
)]
// We don't actually use ui::ui anywhere, we re-export below.
#[allow(clippy::module_inception)]
mod ui;

pub use ui::*;
pub mod ugui;

pub fn screen_process_waiting_switch_to_logo() {
    unsafe { bitbox02_sys::screen_process_waiting_switch_to_logo() }
}

pub fn screen_process_waiting_switch_to_lockscreen() {
    unsafe { bitbox02_sys::screen_process_waiting_switch_to_lockscreen() }
}
