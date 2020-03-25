// Copyright 2020 Shift Cryptosecurity AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use util::c_types::{c_char, c_void};

extern crate alloc;
use crate::password::Password;
use alloc::boxed::Box;
use core::pin::Pin;

/// Wraps the C component_t to be used in Rust.
pub struct Component(*mut bitbox02_sys::component_t);

/// Creates a password input component.
/// `title` - Shown before any input is entered as the screen title. **Panics** if more than 100 bytes.
/// `special_chars` - whether to enable the special characters keyboard.
/// `result` - will be asynchronously set to `Some(<password>)` once the user confirms.
pub fn trinary_input_string_create_password(
    title: &str,
    special_chars: bool,
    result: Pin<&mut Option<Password>>,
) -> Component {
    extern "C" fn on_done_cb(password: *const c_char, param: *mut c_void) {
        let mut out: Box<Pin<&mut Option<Password>>> = unsafe { Box::from_raw(param as *mut _) };
        let mut password_out = Password::new();
        let len = password_out.as_ref().len();
        password_out
            .as_mut()
            .copy_from_slice(unsafe { core::slice::from_raw_parts(password, len) });
        out.set(Some(password_out));
    }

    let component = unsafe {
        bitbox02_sys::trinary_input_string_create_password(
            crate::str_to_cstr_force!(title, 100).as_ptr(),
            special_chars,
            Some(on_done_cb),
            Box::into_raw(Box::new(result)) as *mut _, // passed to on_done_cb as `param`.
            None,
            core::ptr::null_mut(),
        )
    };
    Component(component)
}

pub fn screen_stack_push(component: &mut Component) {
    unsafe {
        bitbox02_sys::ui_screen_stack_push(component.0);
    }
}

pub fn screen_stack_pop() {
    unsafe {
        bitbox02_sys::ui_screen_stack_pop();
    }
}

pub fn screen_process() {
    unsafe {
        bitbox02_sys::screen_process();
    }
}
