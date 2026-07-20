//! C functions not provided by compiler-builtins
//!
//! Use this instead of linking to libc if you only need a handful of free functions

use core::ffi::{c_char, c_void};

// see core::ffi::c_size_t: currently, size_t is always usize
#[allow(non_camel_case_types)]
type c_size_t = usize;

extern "C" {
    // provided by `compiler-builtins`
    fn memcpy(dst: *mut c_void, src: *const c_void, n: c_size_t) -> *mut c_void;
}

/// # Safety
/// - `src` must be a valid C string (null terminated)
/// - `dst` must be large enough to hold `src`
#[no_mangle]
unsafe fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char {
    memcpy(dst as *mut c_void, src as *const c_void, strlen(src)) as *mut c_char
}

/// # Safety
/// `s` must point to valid memory; `s` will be treated as a null terminated string
pub unsafe fn strlen(mut s: *const c_char) -> c_size_t {
    let mut n = 0;
    while *s != 0 {
        s = s.add(1);
        n += 1;
    }
    n
}
