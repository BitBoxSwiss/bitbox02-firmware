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
use alloc::boxed::Box;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::cell::RefCell;
use core::pin::Pin;
use core::task::{Context, Poll, Waker};
use critical_section::Mutex;

use core::marker::PhantomPinned;

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

// Pinky promise to only access the c-ptr from the main thread
unsafe impl Send for Component {}
unsafe impl Sync for Component {}

/// Creates a trinary input component.
/// `result` - will be asynchronously set to `Some(<password>)` once the user confirms.
pub fn trinary_input_string_create<F>(
    params: &TrinaryInputStringParams,
    confirm_callback: F,
    cancel_callback: Option<ContinueCancelCb>,
) -> Component
where
    // Callback must outlive component.
    F: FnMut(zeroize::Zeroizing<String>),
{
    unsafe extern "C" fn c_confirm_callback<F2>(password: *const c_char, user_data: *mut c_void)
    where
        F2: FnMut(zeroize::Zeroizing<String>),
    {
        let pw: zeroize::Zeroizing<String> = zeroize::Zeroizing::new(
            unsafe { crate::util::str_from_null_terminated_ptr(password) }
                .unwrap()
                .into(),
        );
        // The callback is dropped afterwards. This is safe because
        // this C callback is guaranteed to be called only once.
        let mut callback = unsafe { Box::from_raw(user_data as *mut F2) };
        callback(pw);
    }

    unsafe extern "C" fn c_cancel_callback(user_data: *mut c_void) {
        let callback = user_data as *mut ContinueCancelCb;
        unsafe { (*callback)() };
    }

    let (cancel_cb, cancel_user_data) = match cancel_callback {
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
            // passed to c_confirm_callback as `user_data`.
            Box::into_raw(Box::new(confirm_callback)) as *mut _,
            cancel_cb,
            cancel_user_data,
        )
    };
    Component {
        component,
        is_pushed: false,
        on_drop: Some(Box::new(move || unsafe {
            // Drop all callbacks.
            if !cancel_user_data.is_null() {
                drop(Box::from_raw(cancel_user_data as *mut ContinueCancelCb));
            }
        })),
    }
}

/// Creates a user confirmation dialog screen.
/// `result` - will be asynchronously set to `Some(bool)` once the user accets or rejects.
pub fn confirm_create<F>(params: &ConfirmParams, result_callback: F) -> Component
where
    // Callback must outlive component.
    F: FnMut(bool),
{
    unsafe extern "C" fn c_callback<F2>(result: bool, user_data: *mut c_void)
    where
        F2: FnMut(bool),
    {
        // The callback is dropped afterwards. This is safe because
        // this C callback is guaranteed to be called only once.
        let mut callback = unsafe { Box::from_raw(user_data as *mut F2) };
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
            // passed to the C callback as `user_data`
            Box::into_raw(Box::new(result_callback)) as *mut _,
        )
    };
    Component {
        component,
        is_pushed: false,
        on_drop: None,
    }
}

pub fn screen_process() {
    unsafe {
        bitbox02_sys::screen_process();
    }
}

pub fn status_create<F>(text: &str, status_success: bool, callback: F) -> Component
where
    // Callback must outlive component.
    F: FnMut(),
{
    unsafe extern "C" fn c_callback<F2>(user_data: *mut c_void)
    where
        F2: FnMut(),
    {
        // The callback is dropped afterwards. This is safe because
        // this C callback is guaranteed to be called only once.
        let mut callback = unsafe { Box::from_raw(user_data as *mut F2) };
        callback();
    }

    let component = unsafe {
        bitbox02_sys::status_create(
            crate::util::str_to_cstr_vec(text).unwrap().as_ptr(), // copied in C
            status_success,
            Some(c_callback::<F>),
            Box::into_raw(Box::new(callback)) as *mut _, // passed to c_callback as `user_data`.
        )
    };
    Component {
        component,
        is_pushed: false,
        on_drop: None,
    }
}

pub struct SdCard {
    component: Option<Component>,
    result: Option<bool>,
    waker: Option<Waker>,
    _phantom: PhantomPinned,
}

impl SdCard {
    pub fn new() -> Self {
        SdCard {
            component: None,
            result: None,
            waker: None,
            _phantom: PhantomPinned,
        }
    }
}

pub fn sdcard() -> SdCard {
    SdCard::new()
}

impl Future for SdCard {
    type Output = bool;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.component.is_none() {
            unsafe extern "C" fn c_on_done(sd_done: bool, user_data: *mut c_void) {
                // SAFETY: the *mut pointer was created when the object was pinned and thus
                // immovable.
                unsafe {
                    let sdcard = user_data as *mut SdCard;
                    (*sdcard).result = Some(sd_done);
                    (*sdcard).waker.as_ref().unwrap().wake_by_ref();
                }
            }

            let this = unsafe { Pin::into_inner_unchecked(self) };
            let component = unsafe {
                bitbox02_sys::sdcard_create(
                    Some(c_on_done),
                    // passed to the C callback as `user_data`
                    &mut *this as *mut _ as *mut _, // passed to c_on_done as `param`.
                )
            };
            let mut component = Component {
                component,
                is_pushed: false,
                on_drop: None,
            };
            component.screen_stack_push();
            this.component = Some(component);
            this.waker = Some(Waker::clone(cx.waker()));
            Poll::Pending
        } else if let Some(result) = self.result {
            Poll::Ready(result)
        } else {
            Poll::Pending
        }
    }
}

pub fn menu_create(params: MenuParams<'_>) -> Component {
    unsafe extern "C" fn c_select_word_cb(word_idx: u8, user_data: *mut c_void) {
        let callback = user_data as *mut SelectWordCb;
        unsafe { (*callback)(word_idx) };
    }

    unsafe extern "C" fn c_continue_cancel_cb(user_data: *mut c_void) {
        let callback = user_data as *mut ContinueCancelCb;
        unsafe { (*callback)() };
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

    let (select_word_cb, select_word_user_data) = match params.select_word_cb {
        None => (None, core::ptr::null_mut()),
        Some(cb) => (
            Some(c_select_word_cb as _),
            Box::into_raw(Box::new(cb)) as *mut c_void,
        ),
    };

    let (continue_on_last_cb, continue_on_last_user_data) = match params.continue_on_last_cb {
        None => (None, core::ptr::null_mut()),
        Some(cb) => (
            Some(c_continue_cancel_cb as _),
            Box::into_raw(Box::new(cb)) as *mut c_void,
        ),
    };

    let (cancel_cb, cancel_user_data) = match params.cancel_cb {
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
    Component {
        component,
        is_pushed: false,
        on_drop: Some(Box::new(move || unsafe {
            // Drop all callbacks.
            if !select_word_user_data.is_null() {
                drop(Box::from_raw(select_word_user_data as *mut SelectWordCb));
            }
            if !continue_on_last_user_data.is_null() {
                drop(Box::from_raw(
                    continue_on_last_user_data as *mut ContinueCancelCb,
                ));
            }
            if !cancel_user_data.is_null() {
                drop(Box::from_raw(cancel_user_data as *mut ContinueCancelCb));
            }
        })),
    }
}

pub struct TrinaryChoiceUi {
    component: Option<Component>,
    message: String,
    label_left: Option<String>,
    label_middle: Option<String>,
    label_right: Option<String>,
    result: Option<TrinaryChoice>,
    waker: Option<Waker>,
    _phantom: PhantomPinned,
}

impl TrinaryChoiceUi {
    pub fn new(
        message: &str,
        label_left: Option<&str>,
        label_middle: Option<&str>,
        label_right: Option<&str>,
    ) -> TrinaryChoiceUi {
        TrinaryChoiceUi {
            component: None,
            message: message.into(),
            label_left: label_left.map(|s| s.into()),
            label_middle: label_middle.map(|s| s.into()),
            label_right: label_right.map(|s| s.into()),
            result: None,
            waker: None,
            _phantom: PhantomPinned,
        }
    }
}

pub fn trinary_choice(
    message: &str,
    label_left: Option<&str>,
    label_middle: Option<&str>,
    label_right: Option<&str>,
) -> TrinaryChoiceUi {
    TrinaryChoiceUi::new(message, label_left, label_middle, label_right)
}

impl Future for TrinaryChoiceUi {
    type Output = TrinaryChoice;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.component.is_none() {
            unsafe extern "C" fn c_chosen_cb(choice: TrinaryChoice, user_data: *mut c_void) {
                // SAFETY: the *mut pointer was created when the object was pinned and thus
                // immovable.
                unsafe {
                    let trinary_choice = user_data as *mut TrinaryChoiceUi;
                    (*trinary_choice).result = Some(choice);
                    (*trinary_choice).waker.as_ref().unwrap().wake_by_ref();
                }
            }
            let this = unsafe { Pin::into_inner_unchecked(self) };

            let label_left = this
                .label_left
                .as_ref()
                .map(|label| crate::util::str_to_cstr_vec(label).unwrap());
            let label_middle = this
                .label_middle
                .as_ref()
                .map(|label| crate::util::str_to_cstr_vec(label).unwrap());
            let label_right = this
                .label_right
                .as_ref()
                .map(|label| crate::util::str_to_cstr_vec(label).unwrap());

            let component = unsafe {
                bitbox02_sys::trinary_choice_create(
                    crate::util::str_to_cstr_vec(&this.message)
                        .unwrap()
                        .as_ptr(), // copied in C
                    // copied in C
                    label_left
                        .as_ref()
                        .map_or_else(core::ptr::null, |label| label.as_ptr()),
                    // copied in C
                    label_middle
                        .as_ref()
                        .map_or_else(core::ptr::null, |label| label.as_ptr()),
                    // copied in C
                    label_right
                        .as_ref()
                        .map_or_else(core::ptr::null, |label| label.as_ptr()),
                    Some(c_chosen_cb),
                    &mut *this as *mut _ as *mut _, // passed to c_chosen_cb as `user_data`.
                    core::ptr::null_mut(),          // parent component, there is no parent.
                )
            };
            let mut component = Component {
                component,
                is_pushed: false,
                on_drop: None,
            };
            component.screen_stack_push();
            this.component = Some(component);
            this.waker = Some(Waker::clone(cx.waker()));
            Poll::Pending
        } else if let Some(result) = self.result {
            Poll::Ready(result)
        } else {
            Poll::Pending
        }
    }
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

pub fn trinary_input_string_set_input(component: &mut Component, word: &str) {
    unsafe {
        bitbox02_sys::trinary_input_string_set_input(
            component.component,
            crate::util::str_to_cstr_vec(word).unwrap().as_ptr(),
        )
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

pub fn unlock_animation_create<'a, F>(on_done: F) -> Component
where
    // Callback must outlive component.
    F: FnMut(),
{
    unsafe extern "C" fn c_on_done<F2>(param: *mut c_void)
    where
        F2: FnMut(),
    {
        // The callback is dropped afterwards. This is safe because
        // this C callback is guaranteed to be called only once.
        let mut on_done = unsafe { Box::from_raw(param as *mut F2) };
        on_done();
    }
    let component = unsafe {
        bitbox02_sys::unlock_animation_create(
            Some(c_on_done::<F>),
            Box::into_raw(Box::new(on_done)) as *mut _, // passed to c_on_done as `param`.
        )
    };
    Component {
        component,
        is_pushed: false,
        on_drop: None,
    }
}

pub async fn choose_orientation() -> bool {
    // Waker and result is shared with callback
    type SharedState = (Option<Waker>, Option<bool>);
    let shared_state: Arc<RefCell<SharedState>> = Arc::new(RefCell::new((None, None)));

    unsafe extern "C" fn callback(upside_down: bool, user_data: *mut c_void) {
        let shared_state: Arc<RefCell<SharedState>> = unsafe { Arc::from_raw(user_data as *mut _) };
        let shared_state = &mut *shared_state.borrow_mut();
        shared_state.1 = Some(upside_down);
        if let Some(waker) = shared_state.0.as_ref() {
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
            let state = &mut *shared_state.borrow_mut();

            // If it is the first time we are called, set the waker
            let waker = &mut state.0;
            if waker.is_none() {
                waker.replace(Waker::clone(cx.waker()));
            }

            // If callback has set the result, return ready
            if let Some(result) = state.1 {
                return Poll::Ready(result);
            }
            Poll::Pending
        }
    })
    .await
}
