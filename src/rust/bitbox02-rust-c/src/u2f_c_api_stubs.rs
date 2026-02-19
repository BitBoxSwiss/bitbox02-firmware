// SPDX-License-Identifier: Apache-2.0

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
