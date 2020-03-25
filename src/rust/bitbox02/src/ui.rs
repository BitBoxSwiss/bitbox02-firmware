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

pub enum Font {
    Default,
    Password11X12,
}

impl Font {
    fn as_ptr(&self) -> *const bitbox02_sys::UG_FONT {
        match self {
            Font::Default => core::ptr::null() as *const _,
            Font::Password11X12 => unsafe { &bitbox02_sys::font_password_11X12 },
        }
    }
}

impl Default for Font {
    fn default() -> Self {
        return Font::Default;
    }
}

#[derive(Default)]
pub struct ConfirmParams<'a> {
    /// The confirmation title of the screen. Max 200 chars, otherwise **panic**.
    pub title: &'a str,
    /// The confirmation body of the screen. Max 200 chars, otherwise **panic**.
    pub body: &'a str,
    pub font: Font,
    /// If true, the body is horizontally scrollable.
    pub scrollable: bool,
    /// If true, require the hold gesture to confirm instead of tap.
    pub longtouch: bool,
    /// If true, the user can only confirm, not reject.
    pub accept_only: bool,
    /// if true, the accept icon is a right arrow instead of a checkmark (indicating going to the
    /// "next" screen).
    pub accept_is_nextarrow: bool,
    /// if true, will only show first and last 32 bytes.
    pub shorten_body: bool,
    /// Print the value of this variable in the corner. Will not print when 0
    pub display_size: usize,
}

/// Creates a user confirmation dialog screen.
/// `result` - will be asynchronously set to `Some(bool)` once the user accets or rejects.
pub fn confirm_create(params: &ConfirmParams, result: Pin<&mut Option<bool>>) -> Component {
    let params = bitbox02_sys::confirm_params_t {
        title: crate::str_to_cstr_force!(params.title, 200).as_ptr(),
        body: crate::str_to_cstr_force!(params.body, 200).as_ptr(),
        font: params.font.as_ptr(),
        scrollable: params.scrollable,
        longtouch: params.longtouch,
        accept_only: params.accept_only,
        accept_is_nextarrow: params.accept_is_nextarrow,
        shorten_body: params.shorten_body,
        display_size: params.display_size as _,
    };

    extern "C" fn on_accept_cb(param: *mut c_void) {
        let mut out: Box<Pin<&mut Option<bool>>> = unsafe { Box::from_raw(param as *mut _) };
        out.set(Some(true));
    }
    extern "C" fn on_reject_cb(param: *mut c_void) {
        let mut out: Box<Pin<&mut Option<bool>>> = unsafe { Box::from_raw(param as *mut _) };
        out.set(Some(false));
    }

    let result_ptr = Box::into_raw(Box::new(result)) as *mut _; // passed to callbacks `param`.
    let component = unsafe {
        bitbox02_sys::confirm_create(
            &params,
            Some(on_accept_cb),
            result_ptr,
            if !params.accept_only {
                Some(on_reject_cb)
            } else {
                None
            },
            result_ptr,
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
