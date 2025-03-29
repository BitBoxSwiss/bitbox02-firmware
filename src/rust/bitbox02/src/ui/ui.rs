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

pub use super::types::{
    AcceptRejectCb, ConfirmParams, ContinueCancelCb, Font, MenuParams, SelectWordCb, TrinaryChoice,
    TrinaryChoiceCb, TrinaryInputStringParams,
};

use util::c_types::{c_char, c_void};

extern crate alloc;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

use core::marker::PhantomData;

/// Wraps the C component_t to be used in Rust.
pub struct Component<'a> {
    component: *mut bitbox02_sys::component_t,
    is_pushed: bool,
    on_drop: Option<Box<dyn FnMut()>>,
    // This is used to have the result callbacks outlive the component.
    _p: PhantomData<&'a ()>,
}

impl Component<'_> {
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

impl Drop for Component<'_> {
    fn drop(&mut self) {
        if !self.is_pushed {
            panic!("component not pushed");
        }
        unsafe {
            bitbox02_sys::ui_screen_stack_pop();
        }
        if let Some(ref mut on_drop) = self.on_drop {
            (*on_drop)();
        }
    }
}

/// Creates a trinary input component.
/// `result` - will be asynchronously set to `Some(<password>)` once the user confirms.
pub fn trinary_input_string_create<'a, F>(
    params: &TrinaryInputStringParams,
    confirm_callback: F,
    cancel_callback: Option<ContinueCancelCb<'a>>,
) -> Component<'a>
where
    // Callback must outlive component.
    F: FnMut(zeroize::Zeroizing<String>) + 'a,
{
    unsafe extern "C" fn c_confirm_callback<F2>(password: *const c_char, param: *mut c_void)
    where
        F2: FnMut(zeroize::Zeroizing<String>),
    {
        let pw: zeroize::Zeroizing<String> = zeroize::Zeroizing::new(
            crate::util::str_from_null_terminated_ptr(password)
                .unwrap()
                .into(),
        );
        // The callback is dropped afterwards. This is safe because
        // this C callback is guaranteed to be called only once.
        let mut callback = Box::from_raw(param as *mut F2);
        callback(pw);
    }

    unsafe extern "C" fn c_cancel_callback(param: *mut c_void) {
        let callback = param as *mut ContinueCancelCb;
        (*callback)();
    }

    let (cancel_cb, cancel_cb_param) = match cancel_callback {
        None => (None, core::ptr::null_mut()),
        Some(cb) => (
            Some(c_cancel_callback as _),
            Box::into_raw(Box::new(cb)) as *mut c_void,
        ),
    };
    let mut title_scratch = Vec::new();
    let component = unsafe {
        bitbox02_sys::trinary_input_string_create(
            &params.to_c_params(&mut title_scratch).data, // title copied in C
            Some(c_confirm_callback::<F>),
            // passed to c_confirm_callback as `param`.
            Box::into_raw(Box::new(confirm_callback)) as *mut _,
            cancel_cb,
            cancel_cb_param,
        )
    };
    Component {
        component,
        is_pushed: false,
        on_drop: Some(Box::new(move || unsafe {
            // Drop all callbacks.
            if !cancel_cb_param.is_null() {
                drop(Box::from_raw(cancel_cb_param as *mut ContinueCancelCb));
            }
        })),
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
    let mut title_scratch = Vec::new();
    let mut body_scratch = Vec::new();
    let component = unsafe {
        bitbox02_sys::confirm_create(
            &params
                .to_c_params(&mut title_scratch, &mut body_scratch)
                .data,
            Some(c_callback::<F>),
            // passed to the C callback as `param`
            Box::into_raw(Box::new(result_callback)) as *mut _,
        )
    };
    Component {
        component,
        is_pushed: false,
        on_drop: None,
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
            crate::util::str_to_cstr_vec(text).unwrap().as_ptr(), // copied in C
            status_success,
            Some(c_callback::<F>),
            Box::into_raw(Box::new(callback)) as *mut _, // passed to c_callback as `param`.
        )
    };
    Component {
        component,
        is_pushed: false,
        on_drop: None,
        _p: PhantomData,
    }
}

pub fn sdcard_create<'a, F>(callback: F) -> Component<'a>
where
    // Callback must outlive component.
    F: FnMut(bool) + 'a,
{
    unsafe extern "C" fn c_callback<F2>(sd_done: bool, param: *mut c_void)
    where
        F2: FnMut(bool),
    {
        // The callback is dropped afterwards. This is safe because
        // this C callback is guaranteed to be called only once.
        let mut callback = Box::from_raw(param as *mut F2);
        callback(sd_done);
    }

    let component = unsafe {
        bitbox02_sys::sdcard_create(
            Some(c_callback::<F>),
            // passed to the C callback as `param`
            Box::into_raw(Box::new(callback)) as *mut _,
        )
    };
    Component {
        component,
        is_pushed: false,
        on_drop: None,
        _p: PhantomData,
    }
}

pub fn menu_create(params: MenuParams<'_>) -> Component<'_> {
    unsafe extern "C" fn c_select_word_cb(word_idx: u8, param: *mut c_void) {
        let callback = param as *mut SelectWordCb;
        (*callback)(word_idx);
    }

    unsafe extern "C" fn c_continue_cancel_cb(param: *mut c_void) {
        let callback = param as *mut ContinueCancelCb;
        (*callback)();
    }

    // We want to turn &[&str] into a C char**.
    //
    // Step 1: create the C strings. This var has to be alive until after menu_create() finishes,
    // otherwise the pointers we send to menu_create() will be invalid.
    let words: Vec<Vec<u8>> = params
        .words
        .iter()
        .map(|word| crate::util::str_to_cstr_vec(word).unwrap())
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
        .map(|title| crate::util::str_to_cstr_vec(title).unwrap());
    let component = unsafe {
        bitbox02_sys::menu_create(
            c_words.as_ptr(),
            select_word_cb,
            select_word_cb_param,
            words.len() as _,
            // copied in C
            title
                .as_ref()
                .map_or_else(|| core::ptr::null(), |title| title.as_ptr()),
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
        on_drop: Some(Box::new(move || unsafe {
            // Drop all callbacks.
            if !select_word_cb_param.is_null() {
                drop(Box::from_raw(select_word_cb_param as *mut SelectWordCb));
            }
            if !continue_on_last_cb_param.is_null() {
                drop(Box::from_raw(
                    continue_on_last_cb_param as *mut ContinueCancelCb,
                ));
            }
            if !cancel_cb_param.is_null() {
                drop(Box::from_raw(cancel_cb_param as *mut ContinueCancelCb));
            }
        })),
        _p: PhantomData,
    }
}

pub fn trinary_choice_create<'a>(
    message: &'a str,
    label_left: &'a str,
    label_middle: &'a str,
    label_right: &'a str,
    chosen_callback: TrinaryChoiceCb,
) -> Component<'a> {
    unsafe extern "C" fn c_chosen_cb(choice: TrinaryChoice, param: *mut c_void) {
        let callback = param as *mut TrinaryChoiceCb;
        (*callback)(choice);
    }

    let chosen_cb_param = Box::into_raw(Box::new(chosen_callback)) as *mut c_void;
    let component = unsafe {
        bitbox02_sys::trinary_choice_create(
            crate::util::str_to_cstr_vec(message).unwrap().as_ptr(), // copied in C
            crate::util::str_to_cstr_vec(label_left).unwrap().as_ptr(), // copied in C
            crate::util::str_to_cstr_vec(label_middle).unwrap().as_ptr(), // copied in C
            crate::util::str_to_cstr_vec(label_right).unwrap().as_ptr(), // copied in C
            Some(c_chosen_cb as _),
            chosen_cb_param,
            core::ptr::null_mut(), // parent component, there is no parent.
        )
    };
    Component {
        component,
        is_pushed: false,
        on_drop: Some(Box::new(move || unsafe {
            // Drop all callbacks.
            drop(Box::from_raw(chosen_cb_param as *mut TrinaryChoiceCb));
        })),
        _p: PhantomData,
    }
}

pub fn confirm_transaction_address_create<'a, 'b>(
    amount: &'a str,
    address: &'a str,
    callback: AcceptRejectCb<'b>,
) -> Component<'b> {
    unsafe extern "C" fn c_callback(result: bool, param: *mut c_void) {
        let callback = param as *mut AcceptRejectCb;
        (*callback)(result);
    }

    let callback_param = Box::into_raw(Box::new(callback)) as *mut c_void;
    let component = unsafe {
        bitbox02_sys::confirm_transaction_address_create(
            crate::util::str_to_cstr_vec(amount).unwrap().as_ptr(), // copied in C
            crate::util::str_to_cstr_vec(address).unwrap().as_ptr(), // copied in C
            Some(c_callback as _),
            callback_param,
        )
    };
    Component {
        component,
        is_pushed: false,
        on_drop: Some(Box::new(move || unsafe {
            // Drop all callbacks.
            drop(Box::from_raw(callback_param as *mut AcceptRejectCb));
        })),
        _p: PhantomData,
    }
}

pub fn confirm_transaction_fee_create<'a, 'b>(
    amount: &'a str,
    fee: &'a str,
    longtouch: bool,
    callback: AcceptRejectCb<'b>,
) -> Component<'b> {
    unsafe extern "C" fn c_callback(result: bool, param: *mut c_void) {
        let callback = param as *mut AcceptRejectCb;
        (*callback)(result);
    }

    let callback_param = Box::into_raw(Box::new(callback)) as *mut c_void;
    let component = unsafe {
        bitbox02_sys::confirm_transaction_fee_create(
            crate::util::str_to_cstr_vec(amount).unwrap().as_ptr(), // copied in C
            crate::util::str_to_cstr_vec(fee).unwrap().as_ptr(),    // copied in C
            longtouch,
            Some(c_callback as _),
            callback_param,
        )
    };
    Component {
        component,
        is_pushed: false,
        on_drop: Some(Box::new(move || unsafe {
            // Drop all callbacks.
            drop(Box::from_raw(callback_param as *mut AcceptRejectCb));
        })),
        _p: PhantomData,
    }
}

pub fn trinary_input_string_set_input(component: &mut Component, word: &str) {
    unsafe {
        bitbox02_sys::trinary_input_string_set_input(
            component.component,
            crate::util::str_to_cstr_vec(word).unwrap().as_ptr(),
        )
    }
}

pub fn with_lock_animation<F: Fn() -> R, R>(f: F) -> R {
    unsafe { bitbox02_sys::lock_animation_start() };
    let result = f();
    unsafe { bitbox02_sys::lock_animation_stop() };
    result
}

pub fn screen_stack_pop_all() {
    unsafe {
        bitbox02_sys::ui_screen_stack_pop_all();
    }
}

pub fn progress_create<'a>(title: &str) -> Component<'a> {
    let component = unsafe {
        bitbox02_sys::progress_create(
            crate::util::str_to_cstr_vec(title).unwrap().as_ptr(), // copied in C
        )
    };

    Component {
        component,
        is_pushed: false,
        on_drop: None,
        _p: PhantomData,
    }
}

pub fn progress_set(component: &mut Component, progress: f32) {
    unsafe { bitbox02_sys::progress_set(component.component, progress) }
}

pub fn empty_create<'a>() -> Component<'a> {
    Component {
        component: unsafe { bitbox02_sys::empty_create() },
        is_pushed: false,
        on_drop: None,
        _p: PhantomData,
    }
}
