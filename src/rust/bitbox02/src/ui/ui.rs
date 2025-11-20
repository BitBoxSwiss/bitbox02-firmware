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

use core::ffi::{c_char, c_void};

extern crate alloc;
use crate::util::str_to_cstr_vec;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::cell::RefCell;
//use core::pin::Pin;
use core::task::{Poll, Waker};

//use core::marker::PhantomPinned;

/// Wraps the C component_t to be used in Rust.
pub struct Component {
    component: *mut bitbox02_sys::component_t,
    is_pushed: bool,
    on_drop: Option<Box<dyn FnMut()>>,
}

impl Component {
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

impl Drop for Component {
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

pub async fn trinary_input_string<'a>(
    params: &TrinaryInputStringParams<'a>,
    can_cancel: bool,
    preset: &str,
) -> Result<zeroize::Zeroizing<String>, ()> {
    // Shared between the async context and the c callback
    struct SharedState {
        waker: Option<Waker>,
        result: Option<Result<zeroize::Zeroizing<String>, ()>>,
    }
    let shared_state = Arc::new(RefCell::new(SharedState {
        waker: None,
        result: None,
    }));

    unsafe extern "C" fn cancel_cb(user_data: *mut c_void) {
        let shared_state: Arc<RefCell<SharedState>> = unsafe { Arc::from_raw(user_data as *mut _) };
        let mut shared_state = shared_state.borrow_mut();
        shared_state.result = Some(Err(()));
        if let Some(waker) = shared_state.waker.as_ref() {
            waker.wake_by_ref();
        }
    }

    unsafe extern "C" fn confirm_cb(password: *const c_char, user_data: *mut c_void) {
        let shared_state: Arc<RefCell<SharedState>> = unsafe { Arc::from_raw(user_data as *mut _) };
        let mut shared_state = shared_state.borrow_mut();
        let pw: zeroize::Zeroizing<String> = zeroize::Zeroizing::new(
            unsafe { crate::util::str_from_null_terminated_ptr(password) }
                .unwrap()
                .into(),
        );
        shared_state.result = Some(Ok(pw));
        if let Some(waker) = shared_state.waker.as_ref() {
            waker.wake_by_ref();
        }
    }

    let (actual_cancel_cb, cancel_shared_state) = if can_cancel {
        (
            Some(cancel_cb as unsafe extern "C" fn(*mut c_void)),
            Arc::into_raw(Arc::clone(&shared_state)) as *mut _,
        )
    } else {
        (None, core::ptr::null_mut())
    };

    let mut title_scratch = Vec::new();
    let component = unsafe {
        bitbox02_sys::trinary_input_string_create(
            &params.to_c_params(&mut title_scratch).data, // title copied in C
            Some(confirm_cb),
            Arc::into_raw(Arc::clone(&shared_state)) as *mut _, // passed to confirm_cb as `user_data`.
            actual_cancel_cb,
            cancel_shared_state, // passed to cancel_cb as `user_data`.
        )
    };
    if !preset.is_empty() {
        unsafe {
            bitbox02_sys::trinary_input_string_set_input(
                component,
                crate::util::str_to_cstr_vec(preset).unwrap().as_ptr(),
            )
        }
    }

    let mut component = Component {
        component,
        is_pushed: false,
        on_drop: None,
    };
    component.screen_stack_push();

    core::future::poll_fn({
        let shared_state = Arc::clone(&shared_state);
        move |cx| {
            let mut shared_state = shared_state.borrow_mut();

            if let Some(result) = shared_state.result.clone() {
                Poll::Ready(result)
            } else {
                // Store the waker so the callback can wake up this task
                shared_state.waker = Some(cx.waker().clone());
                Poll::Pending
            }
        }
    })
    .await
}

pub async fn confirm<'a>(params: &ConfirmParams<'a>) -> bool {
    // Shared between the async context and the c callback
    struct SharedState {
        waker: Option<Waker>,
        result: Option<bool>,
    }
    let shared_state = Arc::new(RefCell::new(SharedState {
        waker: None,
        result: None,
    }));

    unsafe extern "C" fn callback(result: bool, user_data: *mut c_void) {
        let shared_state: Arc<RefCell<SharedState>> = unsafe { Arc::from_raw(user_data as *mut _) };
        let mut shared_state = shared_state.borrow_mut();
        shared_state.result = Some(result);
        if let Some(waker) = shared_state.waker.as_ref() {
            waker.wake_by_ref();
        }
    }

    let mut title_scratch = Vec::new();
    let mut body_scratch = Vec::new();
    let component = unsafe {
        bitbox02_sys::confirm_create(
            &params
                .to_c_params(&mut title_scratch, &mut body_scratch)
                .data,
            Some(callback),
            Arc::into_raw(Arc::clone(&shared_state)) as *mut _, // passed to callback as `user_data`.
        )
    };

    let mut component = Component {
        component,
        is_pushed: false,
        on_drop: None,
    };
    component.screen_stack_push();

    core::future::poll_fn({
        let shared_state = Arc::clone(&shared_state);
        move |cx| {
            let mut shared_state = shared_state.borrow_mut();

            if let Some(result) = shared_state.result {
                Poll::Ready(result)
            } else {
                // Store the waker so the callback can wake up this task
                shared_state.waker = Some(cx.waker().clone());
                Poll::Pending
            }
        }
    })
    .await
}

pub fn screen_process() {
    unsafe {
        bitbox02_sys::screen_process();
    }
}

pub async fn status(text: &str, status_success: bool) {
    // Shared between the async context and the c callback
    struct SharedState {
        waker: Option<Waker>,
        result: Option<()>,
    }
    let shared_state = Arc::new(RefCell::new(SharedState {
        waker: None,
        result: None,
    }));

    unsafe extern "C" fn callback(user_data: *mut c_void) {
        let shared_state: Arc<RefCell<SharedState>> = unsafe { Arc::from_raw(user_data as *mut _) };
        let mut shared_state = shared_state.borrow_mut();
        shared_state.result = Some(());
        if let Some(waker) = shared_state.waker.as_ref() {
            waker.wake_by_ref();
        }
    }

    let component = unsafe {
        bitbox02_sys::status_create(
            str_to_cstr_vec(text).unwrap().as_ptr(), // copied in C
            status_success,
            Some(callback),
            Arc::into_raw(Arc::clone(&shared_state)) as *mut _, // passed to callback as `user_data`.
        )
    };

    let mut component = Component {
        component,
        is_pushed: false,
        on_drop: None,
    };
    component.screen_stack_push();

    core::future::poll_fn({
        let shared_state = Arc::clone(&shared_state);
        move |cx| {
            let mut shared_state = shared_state.borrow_mut();

            if let Some(result) = shared_state.result {
                Poll::Ready(result)
            } else {
                // Store the waker so the callback can wake up this task
                shared_state.waker = Some(cx.waker().clone());
                Poll::Pending
            }
        }
    })
    .await
}

pub async fn sdcard() -> bool {
    // Shared between the async context and the c callback
    struct SharedState {
        waker: Option<Waker>,
        result: Option<bool>,
    }
    let shared_state = Arc::new(RefCell::new(SharedState {
        waker: None,
        result: None,
    }));

    unsafe extern "C" fn callback(result: bool, user_data: *mut c_void) {
        let shared_state: Arc<RefCell<SharedState>> = unsafe { Arc::from_raw(user_data as *mut _) };
        let mut shared_state = shared_state.borrow_mut();
        shared_state.result = Some(result);
        if let Some(waker) = shared_state.waker.as_ref() {
            waker.wake_by_ref();
        }
    }

    let component = unsafe {
        bitbox02_sys::sdcard_create(
            Some(callback),
            Arc::into_raw(Arc::clone(&shared_state)) as *mut _, // passed to callback as `user_data`.
        )
    };

    let mut component = Component {
        component,
        is_pushed: false,
        on_drop: None,
    };
    component.screen_stack_push();

    core::future::poll_fn({
        let shared_state = Arc::clone(&shared_state);
        move |cx| {
            let mut shared_state = shared_state.borrow_mut();

            if let Some(result) = shared_state.result {
                Poll::Ready(result)
            } else {
                // Store the waker so the callback can wake up this task
                shared_state.waker = Some(cx.waker().clone());
                Poll::Pending
            }
        }
    })
    .await
}

pub async fn menu_create(params: MenuParams<'_>) -> Result<u8, ()> {
    // Shared between the async context and the c callback
    struct SharedState {
        waker: Option<Waker>,
        result: Option<Result<u8, ()>>,
    }
    let shared_state = Arc::new(RefCell::new(SharedState {
        waker: None,
        result: None,
    }));

    unsafe extern "C" fn select_word_cb(word_idx: u8, user_data: *mut c_void) {
        let shared_state: Arc<RefCell<SharedState>> = unsafe { Arc::from_raw(user_data as *mut _) };
        let mut shared_state = shared_state.borrow_mut();
        shared_state.result = Some(Ok(word_idx));
        if let Some(waker) = shared_state.waker.as_ref() {
            waker.wake_by_ref();
        }
    }

    unsafe extern "C" fn continue_on_last_cb(user_data: *mut c_void) {
        let shared_state: Arc<RefCell<SharedState>> = unsafe { Arc::from_raw(user_data as *mut _) };
        let mut shared_state = shared_state.borrow_mut();
        shared_state.result = Some(Ok(0));
        if let Some(waker) = shared_state.waker.as_ref() {
            waker.wake_by_ref();
        }
    }

    unsafe extern "C" fn cancel_cb(user_data: *mut c_void) {
        let shared_state: Arc<RefCell<SharedState>> = unsafe { Arc::from_raw(user_data as *mut _) };
        let mut shared_state = shared_state.borrow_mut();
        shared_state.result = Some(Err(()));
        if let Some(waker) = shared_state.waker.as_ref() {
            waker.wake_by_ref();
        }
    }

    // We want to turn &[&str] into a C char**.
    //
    // Step 1: create the C strings. This var has to be alive until after menu_create() finishes,
    // otherwise the pointers we send to menu_create() will be invalid.
    let words: Vec<Vec<core::ffi::c_char>> = params
        .words
        .iter()
        .map(|word| crate::util::str_to_cstr_vec(word).unwrap())
        .collect();
    // Step two: collect pointers. This var also has to be valid until menu_create() finishes, or
    // the pointer will be invalid.
    let c_words: Vec<*const core::ffi::c_char> =
        words.iter().map(|word| word.as_ptr() as _).collect();

    let (select_word_cb, select_word_user_data) = match params.select_word {
        false => (None, core::ptr::null_mut()),
        true => (
            Some(select_word_cb as _),
            Arc::into_raw(Arc::clone(&shared_state)) as *mut _, // passed to select_word_cb as `user_data`.
        ),
    };

    let (continue_on_last_cb, continue_on_last_user_data) = match params.continue_on_last {
        false => (None, core::ptr::null_mut()),
        true => (
            Some(continue_on_last_cb as _),
            Arc::into_raw(Arc::clone(&shared_state)) as *mut _, // passed to select_word_cb as `user_data`.
        ),
    };

    let (cancel_cb, cancel_user_data) = match params.cancel {
        false => (None, core::ptr::null_mut()),
        true => (
            Some(cancel_cb as _),
            Arc::into_raw(Arc::clone(&shared_state)) as *mut _, // passed to select_word_cb as `user_data`.
        ),
    };
    let title = params
        .title
        .map(|title| crate::util::str_to_cstr_vec(title).unwrap());
    let component = unsafe {
        bitbox02_sys::menu_create(
            c_words.as_ptr(),
            select_word_cb,
            select_word_user_data,
            words.len() as _,
            // copied in C
            title
                .as_ref()
                .map_or_else(core::ptr::null, |title| title.as_ptr()),
            continue_on_last_cb,
            continue_on_last_user_data,
            cancel_cb,
            cancel_user_data,
            core::ptr::null_mut(),
        )
    };
    let mut component = Component {
        component,
        is_pushed: false,
        on_drop: None,
    };
    component.screen_stack_push();

    core::future::poll_fn({
        let shared_state = Arc::clone(&shared_state);
        move |cx| {
            let mut shared_state = shared_state.borrow_mut();

            if let Some(result) = shared_state.result.clone() {
                Poll::Ready(result)
            } else {
                // Store the waker so the callback can wake up this task
                shared_state.waker = Some(cx.waker().clone());
                Poll::Pending
            }
        }
    })
    .await
}

pub async fn trinary_choice(
    message: &str,
    label_left: Option<&str>,
    label_middle: Option<&str>,
    label_right: Option<&str>,
) -> TrinaryChoice {
    // Shared between the async context and the c callback
    struct SharedState {
        waker: Option<Waker>,
        result: Option<TrinaryChoice>,
    }
    let shared_state = Arc::new(RefCell::new(SharedState {
        waker: None,
        result: None,
    }));

    unsafe extern "C" fn callback(choice: TrinaryChoice, user_data: *mut c_void) {
        let shared_state: Arc<RefCell<SharedState>> = unsafe { Arc::from_raw(user_data as *mut _) };
        let mut shared_state = shared_state.borrow_mut();
        shared_state.result = Some(choice);
        if let Some(waker) = shared_state.waker.as_ref() {
            waker.wake_by_ref();
        }
    }

    let component = unsafe {
        bitbox02_sys::trinary_choice_create(
            str_to_cstr_vec(&message).unwrap().as_ptr(), // copied in C
            // copied in C
            label_left
                .as_ref()
                .map(|label| str_to_cstr_vec(label).unwrap())
                .map_or_else(core::ptr::null, |label| label.as_ptr()),
            // copied in C
            label_middle
                .as_ref()
                .map(|label| str_to_cstr_vec(label).unwrap())
                .map_or_else(core::ptr::null, |label| label.as_ptr()),
            // copied in C
            label_right
                .as_ref()
                .map(|label| str_to_cstr_vec(label).unwrap())
                .map_or_else(core::ptr::null, |label| label.as_ptr()),
            Some(callback),
            Arc::into_raw(Arc::clone(&shared_state)) as *mut _, // passed to callback as `user_data`.
            core::ptr::null_mut(), // parent component, there is no parent.
        )
    };

    let mut component = Component {
        component,
        is_pushed: false,
        on_drop: None,
    };
    component.screen_stack_push();

    core::future::poll_fn({
        let shared_state = Arc::clone(&shared_state);
        move |cx| {
            let mut shared_state = shared_state.borrow_mut();

            if let Some(result) = shared_state.result {
                Poll::Ready(result)
            } else {
                // Store the waker so the callback can wake up this task
                shared_state.waker = Some(cx.waker().clone());
                Poll::Pending
            }
        }
    })
    .await
}

pub fn confirm_transaction_address_create<'a>(
    amount: &'a str,
    address: &'a str,
    callback: AcceptRejectCb,
) -> Component {
    unsafe extern "C" fn c_callback(result: bool, user_data: *mut c_void) {
        let callback = user_data as *mut AcceptRejectCb;
        unsafe { (*callback)(result) };
    }

    let user_data = Box::into_raw(Box::new(callback)) as *mut c_void;
    let component = unsafe {
        bitbox02_sys::confirm_transaction_address_create(
            crate::util::str_to_cstr_vec(amount).unwrap().as_ptr(), // copied in C
            crate::util::str_to_cstr_vec(address).unwrap().as_ptr(), // copied in C
            Some(c_callback as _),
            user_data,
        )
    };
    Component {
        component,
        is_pushed: false,
        on_drop: Some(Box::new(move || unsafe {
            // Drop all callbacks.
            drop(Box::from_raw(user_data as *mut AcceptRejectCb));
        })),
    }
}

pub fn confirm_transaction_fee_create<'a>(
    amount: &'a str,
    fee: &'a str,
    longtouch: bool,
    callback: AcceptRejectCb,
) -> Component {
    unsafe extern "C" fn c_callback(result: bool, user_data: *mut c_void) {
        let callback = user_data as *mut AcceptRejectCb;
        unsafe { (*callback)(result) };
    }

    let user_data = Box::into_raw(Box::new(callback)) as *mut c_void;
    let component = unsafe {
        bitbox02_sys::confirm_transaction_fee_create(
            crate::util::str_to_cstr_vec(amount).unwrap().as_ptr(), // copied in C
            crate::util::str_to_cstr_vec(fee).unwrap().as_ptr(),    // copied in C
            longtouch,
            Some(c_callback as _),
            user_data,
        )
    };
    Component {
        component,
        is_pushed: false,
        on_drop: Some(Box::new(move || unsafe {
            // Drop all callbacks.
            drop(Box::from_raw(user_data as *mut AcceptRejectCb));
        })),
    }
}

pub fn screen_stack_pop_all() {
    unsafe {
        bitbox02_sys::ui_screen_stack_pop_all();
    }
}

pub fn progress_create<'a>(title: &str) -> Component {
    let component = unsafe {
        bitbox02_sys::progress_create(
            crate::util::str_to_cstr_vec(title).unwrap().as_ptr(), // copied in C
        )
    };

    Component {
        component,
        is_pushed: false,
        on_drop: None,
    }
}

pub fn progress_set(component: &mut Component, progress: f32) {
    unsafe { bitbox02_sys::progress_set(component.component, progress) }
}

pub fn empty_create<'a>() -> Component {
    Component {
        component: unsafe { bitbox02_sys::empty_create() },
        is_pushed: false,
        on_drop: None,
    }
}

pub async fn unlock_animation() {
    // Shared between the async context and the c callback
    struct SharedState {
        waker: Option<Waker>,
        result: Option<()>,
    }
    let shared_state = Arc::new(RefCell::new(SharedState {
        waker: None,
        result: None,
    }));

    unsafe extern "C" fn callback(user_data: *mut c_void) {
        let shared_state: Arc<RefCell<SharedState>> = unsafe { Arc::from_raw(user_data as *mut _) };
        let mut shared_state = shared_state.borrow_mut();
        shared_state.result = Some(());
        if let Some(waker) = shared_state.waker.as_ref() {
            waker.wake_by_ref();
        }
    }

    let component = unsafe {
        bitbox02_sys::unlock_animation_create(
            Some(callback),
            Arc::into_raw(Arc::clone(&shared_state)) as *mut _, // passed to callback as `user_data`.
        )
    };

    let mut component = Component {
        component,
        is_pushed: false,
        on_drop: None,
    };
    component.screen_stack_push();

    core::future::poll_fn({
        let shared_state = Arc::clone(&shared_state);
        move |cx| {
            let mut shared_state = shared_state.borrow_mut();

            if let Some(result) = shared_state.result {
                Poll::Ready(result)
            } else {
                // Store the waker so the callback can wake up this task
                shared_state.waker = Some(cx.waker().clone());
                Poll::Pending
            }
        }
    })
    .await
}

pub async fn choose_orientation() -> bool {
    // Shared between the async context and the c callback
    struct SharedState {
        waker: Option<Waker>,
        result: Option<bool>,
    }
    let shared_state = Arc::new(RefCell::new(SharedState {
        waker: None,
        result: None,
    }));

    unsafe extern "C" fn callback(upside_down: bool, user_data: *mut c_void) {
        let shared_state: Arc<RefCell<SharedState>> = unsafe { Arc::from_raw(user_data as *mut _) };
        let mut shared_state = shared_state.borrow_mut();
        shared_state.result = Some(upside_down);
        if let Some(waker) = shared_state.waker.as_ref() {
            util::log!("wake by ref");
            waker.wake_by_ref();
        }
    }

    let component = unsafe {
        bitbox02_sys::orientation_arrows_create(
            Some(callback),
            Arc::into_raw(Arc::clone(&shared_state)) as *mut _, // passed to callback as `user_data`.
        )
    };

    let mut component = Component {
        component,
        is_pushed: false,
        on_drop: None,
    };
    component.screen_stack_push();

    core::future::poll_fn({
        let shared_state = Arc::clone(&shared_state);
        move |cx| {
            let mut shared_state = shared_state.borrow_mut();

            if let Some(result) = shared_state.result {
                Poll::Ready(result)
            } else {
                // Store the waker so the callback can wake up this task
                shared_state.waker = Some(cx.waker().clone());
                Poll::Pending
            }
        }
    })
    .await
}
