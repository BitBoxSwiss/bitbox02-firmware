// Copyright 2020 Shift Cryptosecurity AG
// Copyright 2020 Shift Crypto AG
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

// Taking the constant straight from C, as it's excluding the null terminator.
const MAX_LABEL_SIZE: usize = bitbox02_sys::MAX_LABEL_SIZE as _;

/// Wraps the C component_t to be used in Rust.
pub use core::marker::PhantomData;

/// All C UI functions needed in Rust. All methods have a default implementation to make mocking it
/// easier.
pub trait UI: Sized {
    fn trinary_input_string_create_password<'a, F>(
        _title: &str,
        _special_chars: bool,
        _confirm_callback: F,
    ) -> Component<'a, Self>
    where
        // Callback must outlive component.
        F: FnMut(Password) + 'a,
    {
        panic!("not implemented");
    }
    fn confirm_create<'a, F>(_params: &ConfirmParams, _result_callback: F) -> Component<'a, Self>
    where
        // Callback must outlive component.
        F: FnMut(bool) + 'a,
    {
        panic!("not implemented");
    }

    fn screen_process() {}
    fn screen_stack_push(_component: &Component<Self>) {}
    fn screen_stack_pop() {}
    fn status_create<'a, F>(_text: &str, _status_success: bool, _callback: F) -> Component<'a, Self>
    where
        // Callback must outlive component.
        F: FnMut() + 'a,
    {
        panic!("not implemented");
    }
    fn with_lock_animation<F: Fn()>(_f: F) {
        panic!("not implemented");
    }
    fn screen_stack_pop_all() {
        panic!("not implemented");
    }
}

/// Exposes the C functions safely to Rust.
pub enum CUI {}

pub struct Component<'a, U: UI> {
    component: *mut bitbox02_sys::component_t,
    is_pushed: bool,
    // This is used to have the result callbacks outlive the component.
    _p: PhantomData<&'a U>,
}

impl<'a, U: UI> Component<'a, U> {
    pub fn screen_stack_push(&mut self) {
        if self.is_pushed {
            panic!("component pushed twice");
        }
        U::screen_stack_push(&self);
        self.is_pushed = true;
    }
}

impl<'a, U: UI> Drop for Component<'a, U> {
    fn drop(&mut self) {
        if !self.is_pushed {
            panic!("component not pushed");
        }
        U::screen_stack_pop()
    }
}

pub enum Font {
    Default,
    Password11X12,
    Monogram5X9,
}

impl Font {
    fn as_ptr(&self) -> *const bitbox02_sys::UG_FONT {
        match self {
            Font::Default => core::ptr::null() as *const _,
            Font::Password11X12 => unsafe { &bitbox02_sys::font_password_11X12 },
            Font::Monogram5X9 => unsafe { &bitbox02_sys::font_monogram_5X9 },
        }
    }
}

impl Default for Font {
    fn default() -> Self {
        Font::Default
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
    /// Print the value of this variable in the corner. Will not print when 0
    pub display_size: usize,
}

impl UI for CUI {
    /// Creates a password input component.
    /// `title` - Shown before any input is entered as the screen title. **Panics** if more than 100 bytes.
    /// `special_chars` - whether to enable the special characters keyboard.
    /// `result` - will be asynchronously set to `Some(<password>)` once the user confirms.
    fn trinary_input_string_create_password<'a, F>(
        title: &str,
        special_chars: bool,
        confirm_callback: F,
    ) -> Component<'a, Self>
    where
        // Callback must outlive component.
        F: FnMut(Password) + 'a,
    {
        unsafe extern "C" fn c_confirm_callback<F2>(password: *const c_char, param: *mut c_void)
        where
            F2: FnMut(Password),
        {
            let mut password_out = Password::new();
            let cap = password_out.cap();
            password_out
                .as_mut()
                .copy_from_slice(core::slice::from_raw_parts(password, cap));
            // The callback is dropped afterwards. This is safe because
            // this C callback is guaranteed to be called only once.
            let mut callback = Box::from_raw(param as *mut F2);
            callback(password_out);
        }

        let component = unsafe {
            bitbox02_sys::trinary_input_string_create_password(
                crate::str_to_cstr_force!(title, MAX_LABEL_SIZE).as_ptr(),
                special_chars,
                Some(c_confirm_callback::<F>),
                // passed to c_confirm_callback as `param`.
                Box::into_raw(Box::new(confirm_callback)) as *mut _,
                None,
                core::ptr::null_mut(),
            )
        };
        Component {
            component,
            is_pushed: false,
            _p: PhantomData,
        }
    }

    /// Creates a user confirmation dialog screen.
    /// `result` - will be asynchronously set to `Some(bool)` once the user accets or rejects.
    fn confirm_create<'a, F>(params: &ConfirmParams, result_callback: F) -> Component<'a, Self>
    where
        // Callback must outlive component.
        F: FnMut(bool) + 'a,
    {
        // We truncate at a bit higher than MAX_LABEL_SIZE, so the label component will correctly
        // truncate and append '...'.
        const TRUNCATE_SIZE: usize = MAX_LABEL_SIZE + 1;

        let params = bitbox02_sys::confirm_params_t {
            title: crate::str_to_cstr_force!(
                crate::util::truncate_str(params.title, TRUNCATE_SIZE),
                TRUNCATE_SIZE
            )
            .as_ptr(),
            body: crate::str_to_cstr_force!(
                crate::util::truncate_str(params.body, TRUNCATE_SIZE),
                TRUNCATE_SIZE
            )
            .as_ptr(),
            font: params.font.as_ptr(),
            scrollable: params.scrollable,
            longtouch: params.longtouch,
            accept_only: params.accept_only,
            accept_is_nextarrow: params.accept_is_nextarrow,
            display_size: params.display_size as _,
        };

        unsafe extern "C" fn c_callback<F2>(result: bool, param: *mut c_void)
        where
            F2: FnMut(bool),
        {
            // The callback is dropped afterwards. This is safe because
            // this C callback is guaranteed to be called only once.
            let mut callback = Box::from_raw(param as *mut F2);
            callback(result);
        }

        let component = unsafe {
            bitbox02_sys::confirm_create(
                &params,
                Some(c_callback::<F>),
                // passed to the C callback as `param`
                Box::into_raw(Box::new(result_callback)) as *mut _,
            )
        };
        Component {
            component,
            is_pushed: false,
            _p: PhantomData,
        }
    }

    fn screen_process() {
        unsafe {
            bitbox02_sys::screen_process();
        }
    }

    fn screen_stack_push(component: &Component<Self>) {
        unsafe {
            bitbox02_sys::ui_screen_stack_push(component.component);
        }
    }

    fn screen_stack_pop() {
        unsafe {
            bitbox02_sys::ui_screen_stack_pop();
        }
    }

    fn status_create<'a, F>(text: &str, status_success: bool, callback: F) -> Component<'a, Self>
    where
        // Callback must outlive component.
        F: FnMut() + 'a,
    {
        unsafe extern "C" fn c_callback<F2>(param: *mut c_void)
        where
            F2: FnMut(),
        {
            // The callback is dropped afterwards. This is safe because
            // this C callback is guaranteed to be called only once.
            let mut callback = Box::from_raw(param as *mut F2);
            callback();
        }

        let component = unsafe {
            bitbox02_sys::status_create(
                crate::str_to_cstr_force!(text, MAX_LABEL_SIZE).as_ptr(),
                status_success,
                Some(c_callback::<F>),
                Box::into_raw(Box::new(callback)) as *mut _, // passed to c_callback as `param`.
            )
        };
        Component {
            component,
            is_pushed: false,
            _p: PhantomData,
        }
    }

    fn with_lock_animation<F: Fn()>(f: F) {
        unsafe { bitbox02_sys::lock_animation_start() };
        f();
        unsafe { bitbox02_sys::lock_animation_stop() };
    }

    fn screen_stack_pop_all() {
        unsafe {
            bitbox02_sys::ui_screen_stack_pop_all();
        }
    }
}
