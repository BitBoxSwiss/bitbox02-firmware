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

use super::types::MAX_LABEL_SIZE;
pub use super::types::{ConfirmParams, Font, MenuParams};

use util::c_types::{c_char, c_void};

extern crate alloc;
use crate::password::Password;
use alloc::boxed::Box;
use alloc::vec::Vec;

use core::marker::PhantomData;

/// Wraps the C component_t to be used in Rust.
pub struct Component<'a> {
    component: *mut bitbox02_sys::component_t,
    is_pushed: bool,
    on_drop: Box<dyn FnMut()>,
    // This is used to have the result callbacks outlive the component.
    _p: PhantomData<&'a ()>,
}

impl<'a> Component<'a> {
    pub fn screen_stack_push(&mut self) {
        if self.is_pushed {
            panic!("component pushed twice");
        }
        unsafe {
            bitbox02_sys::ui_screen_stack_push(self.component);
        }
        self.is_pushed = true;
    }
}

impl<'a> Drop for Component<'a> {
    fn drop(&mut self) {
        if !self.is_pushed {
            panic!("component not pushed");
        }
        unsafe {
            bitbox02_sys::ui_screen_stack_pop();
        }
        (*self.on_drop)();
    }
}

/// Creates a password input component.
/// `title` - Shown before any input is entered as the screen title. **Panics** if more than 100 bytes.
/// `special_chars` - whether to enable the special characters keyboard.
/// `result` - will be asynchronously set to `Some(<password>)` once the user confirms.
pub fn trinary_input_string_create_password<'a, F>(
    title: &str,
    special_chars: bool,
    confirm_callback: F,
) -> Component<'a>
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
        on_drop: Box::new(|| {}),
        _p: PhantomData,
    }
}

/// Creates a user confirmation dialog screen.
/// `result` - will be asynchronously set to `Some(bool)` once the user accets or rejects.
pub fn confirm_create<'a, F>(params: &ConfirmParams, result_callback: F) -> Component<'a>
where
    // Callback must outlive component.
    F: FnMut(bool) + 'a,
{
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
            &params.to_c_params(),
            Some(c_callback::<F>),
            // passed to the C callback as `param`
            Box::into_raw(Box::new(result_callback)) as *mut _,
        )
    };
    Component {
        component,
        is_pushed: false,
        on_drop: Box::new(|| {}),
        _p: PhantomData,
    }
}

pub fn screen_process() {
    unsafe {
        bitbox02_sys::screen_process();
    }
}

pub fn status_create<'a, F>(text: &str, status_success: bool, callback: F) -> Component<'a>
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
        on_drop: Box::new(|| {}),
        _p: PhantomData,
    }
}

pub fn sdcard_create<'a, F>(insert: bool, continue_callback: F) -> Component<'a>
where
    // Callback must outlive component.
    F: FnMut() + 'a,
{
    unsafe extern "C" fn c_continue_callback<F2>(param: *mut c_void)
    where
        F2: FnMut(),
    {
        // The callback is dropped afterwards. This is safe because
        // this C callback is guaranteed to be called only once.
        let mut callback = Box::from_raw(param as *mut F2);
        callback();
    }

    let component = unsafe {
        bitbox02_sys::sdcard_create(
            insert,
            Some(c_continue_callback::<F>),
            // passed to the C callback as `param`
            Box::into_raw(Box::new(continue_callback)) as *mut _,
        )
    };
    Component {
        component,
        is_pushed: false,
        on_drop: Box::new(|| {}),
        _p: PhantomData,
    }
}

pub fn menu_create(params: MenuParams<'_>) -> Component<'_> {
    unsafe extern "C" fn c_select_word_cb(word_idx: u8, param: *mut c_void) {
        let callback = param as *mut Box<dyn FnMut(u8)>;
        (*callback)(word_idx);
    }

    unsafe extern "C" fn c_continue_cancel_cb(param: *mut c_void) {
        let callback = param as *mut Box<dyn FnMut()>;
        (*callback)();
    }

    // We want to turn &[&str] into a C char**.
    //
    // Step 1: create the C strings. This var has to be alive until after menu_create() finishes,
    // otherwise the pointers we send to menu_create() will be invalid.
    let words: Vec<[u8; 101]> = params
        .words
        .iter()
        .map(|word| crate::str_to_cstr_force!(word, 100))
        .collect();
    // Step two: collect pointers. This var also has to be valid until menu_create() finishes, or
    // the pointer will be invalid.
    let c_words: Vec<*const util::c_types::c_char> =
        words.iter().map(|word| word.as_ptr() as _).collect();

    let (select_word_cb, select_word_cb_param) = match params.select_word_cb {
        None => (None, core::ptr::null_mut()),
        Some(cb) => (
            Some(c_select_word_cb as _),
            Box::into_raw(Box::new(cb)) as *mut c_void,
        ),
    };

    let (continue_on_last_cb, continue_on_last_cb_param) = match params.continue_on_last_cb {
        None => (None, core::ptr::null_mut()),
        Some(cb) => (
            Some(c_continue_cancel_cb as _),
            Box::into_raw(Box::new(cb)) as *mut c_void,
        ),
    };

    let (cancel_cb, cancel_cb_param) = match params.cancel_cb {
        None => (None, core::ptr::null_mut()),
        Some(cb) => (
            Some(c_continue_cancel_cb as _),
            Box::into_raw(Box::new(cb)) as *mut c_void,
        ),
    };
    let title = params
        .title
        .map(|title| crate::str_to_cstr_force!(title, MAX_LABEL_SIZE));
    let component = unsafe {
        bitbox02_sys::menu_create(
            c_words.as_ptr(),
            select_word_cb,
            select_word_cb_param,
            words.len() as _,
            title.map_or_else(
                || core::ptr::null(),
                |title| crate::str_to_cstr_force!(title, MAX_LABEL_SIZE).as_ptr(),
            ),
            continue_on_last_cb,
            continue_on_last_cb_param,
            cancel_cb,
            cancel_cb_param,
            core::ptr::null_mut(),
        )
    };
    Component {
        component,
        is_pushed: false,
        on_drop: Box::new(move || unsafe {
            // Drop all callbacks.
            if !select_word_cb_param.is_null() {
                drop(Box::from_raw(
                    select_word_cb_param as *mut Box<dyn FnMut(u8)>,
                ));
            }
            if !continue_on_last_cb_param.is_null() {
                drop(Box::from_raw(
                    continue_on_last_cb_param as *mut Box<dyn FnMut()>,
                ));
            }
            if !cancel_cb_param.is_null() {
                drop(Box::from_raw(cancel_cb_param as *mut Box<dyn FnMut()>));
            }
        }),
        _p: PhantomData,
    }
}

pub fn with_lock_animation<F: Fn()>(f: F) {
    unsafe { bitbox02_sys::lock_animation_start() };
    f();
    unsafe { bitbox02_sys::lock_animation_stop() };
}

pub fn screen_stack_pop_all() {
    unsafe {
        bitbox02_sys::ui_screen_stack_pop_all();
    }
}
