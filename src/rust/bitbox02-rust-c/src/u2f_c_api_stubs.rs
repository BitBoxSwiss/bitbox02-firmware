// SPDX-License-Identifier: Apache-2.0

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_workflow_spawn_unlock() -> bool {
    true
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_workflow_u2f_is_active() -> bool {
    false
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_workflow_spawn_confirm(
    _title: *const core::ffi::c_char,
    _body: *const core::ffi::c_char,
) -> bool {
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
