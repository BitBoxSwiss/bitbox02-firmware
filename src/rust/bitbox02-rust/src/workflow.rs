// SPDX-License-Identifier: Apache-2.0

pub mod cancel;
pub mod confirm;
pub mod menu;
#[cfg_attr(
    all(feature = "c-unit-testing", not(feature = "testing")),
    path = "workflow/mnemonic_c_unit_tests.rs"
)]
pub mod mnemonic;
pub mod orientation_screen;
pub mod pairing;
pub mod password;
pub mod sdcard;
pub mod transaction;
pub mod trinary_choice;
pub mod trinary_input_string;
pub mod unlock;
pub mod unlock_animation;
pub mod verify_message;

// Active in production firmware.
#[cfg(all(
    feature = "app-u2f",
    not(any(feature = "c-unit-testing", feature = "simulator-graphical"))
))]
pub mod u2f_c_api;

// Stubs for C unit tests and C simulator - these are currently compiled and linked but they don't
// actually have to spawn/poll futures. The C simulator does not contain U2F, and the unit tests
// don't contain an executor.
#[cfg(all(
    feature = "app-u2f",
    any(feature = "c-unit-testing", feature = "simulator-graphical")
))]
pub mod u2f_c_api {
    #![allow(clippy::missing_safety_doc)]

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn rust_workflow_spawn_unlock() {}
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn rust_workflow_spawn_confirm(
        _title: *const core::ffi::c_char,
        _body: *const core::ffi::c_char,
    ) {
        panic!("unused");
    }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn rust_workflow_unlock_poll(_result_out: &mut bool) -> bool {
        panic!("unused");
    }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn rust_workflow_confirm_poll(_result_out: &mut bool) -> bool {
        panic!("unused");
    }
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn rust_workflow_abort_current() {
        panic!("unused");
    }
}
