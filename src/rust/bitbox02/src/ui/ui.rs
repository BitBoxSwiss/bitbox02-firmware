// SPDX-License-Identifier: Apache-2.0

pub use super::types::{
    AcceptRejectCb, ConfirmParams, ContinueCancelCb, Font, MenuParams, SelectWordCb, TrinaryChoice,
    TrinaryChoiceCb, TrinaryInputStringParams,
};

use core::ffi::{c_char, c_void};

extern crate alloc;
use alloc::rc::Rc;
use alloc::string::String;
use alloc::vec::Vec;
use core::cell::RefCell;
use core::task::{Poll, Waker};

/// Wraps the C component_t to be used in Rust.
pub struct Component {
    component: *mut bitbox02_sys::component_t,
    is_pushed: bool,
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
    }
}

pub async fn trinary_input_string(
    params: &TrinaryInputStringParams<'_>,
    can_cancel: bool,
    preset: &str,
) -> Result<zeroize::Zeroizing<String>, ()> {
    let _no_screensaver = crate::screen_saver::ScreensaverInhibitor::new();

    // Shared between the async context and the c callback
    struct SharedState {
        waker: Option<Waker>,
        result: Option<Result<zeroize::Zeroizing<String>, ()>>,
    }
    let shared_state = Rc::new(RefCell::new(SharedState {
        waker: None,
        result: None,
    }));

    unsafe extern "C" fn cancel_cb(user_data: *mut c_void) {
        let shared_state: Rc<RefCell<SharedState>> = unsafe { Rc::from_raw(user_data as *mut _) };
        let mut shared_state = shared_state.borrow_mut();
        shared_state.result = Some(Err(()));
        if let Some(waker) = shared_state.waker.as_ref() {
            waker.wake_by_ref();
        }
    }

    unsafe extern "C" fn confirm_cb(password: *const c_char, user_data: *mut c_void) {
        let shared_state: Rc<RefCell<SharedState>> = unsafe { Rc::from_raw(user_data as *mut _) };
        let mut shared_state = shared_state.borrow_mut();
        let pw: zeroize::Zeroizing<String> = zeroize::Zeroizing::new(
            unsafe { util::strings::str_from_null_terminated_ptr(password) }
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
            Rc::into_raw(Rc::clone(&shared_state)) as *mut _,
        )
    } else {
        (None, core::ptr::null_mut())
    };

    // We truncate at a bit higher than MAX_LABEL_SIZE, so the label component will correctly
    // truncate and append '...'.
    const TRUNCATE_SIZE: usize = super::types::MAX_LABEL_SIZE + 1;
    let title =
        util::strings::str_to_cstr_vec(util::strings::truncate_str(params.title, TRUNCATE_SIZE))
            .unwrap();
    let c_params = bitbox02_sys::trinary_input_string_params_t {
        title: title.as_ptr().cast(),
        wordlist: match params.wordlist {
            None => core::ptr::null(),
            Some(wordlist) => wordlist.as_ptr(),
        },
        wordlist_size: match params.wordlist {
            None => 0,
            Some(wordlist) => wordlist.len() as _,
        },
        number_input: params.number_input,
        hide: params.hide,
        special_chars: params.special_chars,
        longtouch: params.longtouch,
        cancel_is_backbutton: params.cancel_is_backbutton,
        default_to_digits: params.default_to_digits,
    };
    let component = unsafe {
        bitbox02_sys::trinary_input_string_create(
            &c_params, // title copied in C
            Some(confirm_cb),
            Rc::into_raw(Rc::clone(&shared_state)) as *mut _, // passed to confirm_cb as `user_data`.
            actual_cancel_cb,
            cancel_shared_state, // passed to cancel_cb as `user_data`.
        )
    };
    if !preset.is_empty() {
        unsafe {
            bitbox02_sys::trinary_input_string_set_input(
                component,
                util::strings::str_to_cstr_vec(preset).unwrap().as_ptr(),
            )
        }
    }

    let mut component = Component {
        component,
        is_pushed: false,
    };
    component.screen_stack_push();

    core::future::poll_fn({
        let shared_state = Rc::clone(&shared_state);
        move |cx| {
            let mut shared_state = shared_state.borrow_mut();

            if let Some(result) = shared_state.result.take() {
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

/// Returns true if the user accepts, false if the user rejects.
pub async fn confirm(params: &ConfirmParams<'_>) -> bool {
    let _no_screensaver = crate::screen_saver::ScreensaverInhibitor::new();

    // Shared between the async context and the c callback
    struct SharedState {
        waker: Option<Waker>,
        result: Option<bool>,
    }
    let shared_state = Rc::new(RefCell::new(SharedState {
        waker: None,
        result: None,
    }));

    unsafe extern "C" fn callback(result: bool, user_data: *mut c_void) {
        let shared_state: Rc<RefCell<SharedState>> = unsafe { Rc::from_raw(user_data as *mut _) };
        let mut shared_state = shared_state.borrow_mut();
        shared_state.result = Some(result);
        if let Some(waker) = shared_state.waker.as_ref() {
            waker.wake_by_ref();
        }
    }

    // We truncate at a bit higher than MAX_LABEL_SIZE, so the label component will correctly
    // truncate and append '...'.
    const TRUNCATE_SIZE: usize = bitbox02_sys::MAX_LABEL_SIZE as usize + 1;
    let title =
        util::strings::str_to_cstr_vec(util::strings::truncate_str(params.title, TRUNCATE_SIZE))
            .unwrap();
    let body =
        util::strings::str_to_cstr_vec(util::strings::truncate_str(params.body, TRUNCATE_SIZE))
            .unwrap();
    let c_params = bitbox02_sys::confirm_params_t {
        title: title.as_ptr().cast(),
        title_autowrap: params.title_autowrap,
        body: body.as_ptr().cast(),
        font: params.font.as_ptr(),
        scrollable: params.scrollable,
        longtouch: params.longtouch,
        accept_only: params.accept_only,
        accept_is_nextarrow: params.accept_is_nextarrow,
        display_size: params.display_size as _,
    };
    let component = unsafe {
        bitbox02_sys::confirm_create(
            &c_params,
            Some(callback),
            Rc::into_raw(Rc::clone(&shared_state)) as *mut _, // passed to callback as `user_data`.
        )
    };

    let mut component = Component {
        component,
        is_pushed: false,
    };
    component.screen_stack_push();

    core::future::poll_fn({
        let shared_state = Rc::clone(&shared_state);
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

pub fn status_create(text: &str, status_success: bool) -> Component {
    let component = unsafe {
        bitbox02_sys::status_create(
            util::strings::str_to_cstr_vec(text).unwrap().as_ptr(), // copied in C
            status_success,
        )
    };
    Component {
        component,
        is_pushed: false,
    }
}

pub async fn sdcard() -> bool {
    let _no_screensaver = crate::screen_saver::ScreensaverInhibitor::new();

    // Shared between the async context and the c callback
    struct SharedState {
        waker: Option<Waker>,
        result: Option<bool>,
    }
    let shared_state = Rc::new(RefCell::new(SharedState {
        waker: None,
        result: None,
    }));

    unsafe extern "C" fn callback(result: bool, user_data: *mut c_void) {
        let shared_state: Rc<RefCell<SharedState>> = unsafe { Rc::from_raw(user_data as *mut _) };
        let mut shared_state = shared_state.borrow_mut();
        shared_state.result = Some(result);
        if let Some(waker) = shared_state.waker.as_ref() {
            waker.wake_by_ref();
        }
    }

    let component = unsafe {
        bitbox02_sys::sdcard_create(
            Some(callback),
            Rc::into_raw(Rc::clone(&shared_state)) as *mut _, // passed to callback as `user_data`.
        )
    };

    let mut component = Component {
        component,
        is_pushed: false,
    };
    component.screen_stack_push();

    core::future::poll_fn({
        let shared_state = Rc::clone(&shared_state);
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
    let _no_screensaver = crate::screen_saver::ScreensaverInhibitor::new();

    // Shared between the async context and the c callback
    struct SharedState {
        waker: Option<Waker>,
        result: Option<Result<u8, ()>>,
    }
    let shared_state = Rc::new(RefCell::new(SharedState {
        waker: None,
        result: None,
    }));

    unsafe extern "C" fn select_word_cb(word_idx: u8, user_data: *mut c_void) {
        let shared_state: Rc<RefCell<SharedState>> = unsafe { Rc::from_raw(user_data as *mut _) };
        let mut shared_state = shared_state.borrow_mut();
        shared_state.result = Some(Ok(word_idx));
        if let Some(waker) = shared_state.waker.as_ref() {
            waker.wake_by_ref();
        }
    }

    unsafe extern "C" fn continue_on_last_cb(user_data: *mut c_void) {
        let shared_state: Rc<RefCell<SharedState>> = unsafe { Rc::from_raw(user_data as *mut _) };
        let mut shared_state = shared_state.borrow_mut();
        shared_state.result = Some(Ok(0));
        if let Some(waker) = shared_state.waker.as_ref() {
            waker.wake_by_ref();
        }
    }

    unsafe extern "C" fn cancel_cb(user_data: *mut c_void) {
        let shared_state: Rc<RefCell<SharedState>> = unsafe { Rc::from_raw(user_data as *mut _) };
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
        .map(|word| util::strings::str_to_cstr_vec(word).unwrap())
        .collect();
    // Step two: collect pointers. This var also has to be valid until menu_create() finishes, or
    // the pointer will be invalid.
    let c_words: Vec<*const core::ffi::c_char> =
        words.iter().map(|word| word.as_ptr() as _).collect();

    let (select_word_cb, select_word_user_data) = match params.select_word {
        false => (None, core::ptr::null_mut()),
        true => (
            Some(select_word_cb as _),
            Rc::into_raw(Rc::clone(&shared_state)) as *mut _, // passed to select_word_cb as `user_data`.
        ),
    };

    let (continue_on_last_cb, continue_on_last_user_data) = match params.continue_on_last {
        false => (None, core::ptr::null_mut()),
        true => (
            Some(continue_on_last_cb as _),
            Rc::into_raw(Rc::clone(&shared_state)) as *mut _, // passed to continue_on_last_cb as `user_data`.
        ),
    };

    let (cancel_cb, cancel_user_data) = match params.cancel {
        false => (None, core::ptr::null_mut()),
        true => (
            Some(cancel_cb as _),
            Rc::into_raw(Rc::clone(&shared_state)) as *mut _, // passed to cancel_cb as `user_data`.
        ),
    };
    let title = params
        .title
        .map(|title| util::strings::str_to_cstr_vec(title).unwrap());
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
    };
    component.screen_stack_push();

    core::future::poll_fn({
        let shared_state = Rc::clone(&shared_state);
        move |cx| {
            let mut shared_state = shared_state.borrow_mut();

            if let Some(result) = shared_state.result.take() {
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
    let _no_screensaver = crate::screen_saver::ScreensaverInhibitor::new();

    // Shared between the async context and the c callback
    struct SharedState {
        waker: Option<Waker>,
        result: Option<TrinaryChoice>,
    }
    let shared_state = Rc::new(RefCell::new(SharedState {
        waker: None,
        result: None,
    }));

    unsafe extern "C" fn callback(choice: TrinaryChoice, user_data: *mut c_void) {
        let shared_state: Rc<RefCell<SharedState>> = unsafe { Rc::from_raw(user_data as *mut _) };
        let mut shared_state = shared_state.borrow_mut();
        shared_state.result = Some(choice);
        if let Some(waker) = shared_state.waker.as_ref() {
            waker.wake_by_ref();
        }
    }

    let label_left = label_left.map(|label| util::strings::str_to_cstr_vec(label).unwrap());
    let label_middle = label_middle.map(|label| util::strings::str_to_cstr_vec(label).unwrap());
    let label_right = label_right.map(|label| util::strings::str_to_cstr_vec(label).unwrap());

    let component = unsafe {
        bitbox02_sys::trinary_choice_create(
            util::strings::str_to_cstr_vec(message).unwrap().as_ptr(), // copied in C
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
            Some(callback),
            Rc::into_raw(Rc::clone(&shared_state)) as *mut _, // passed to callback as `user_data`.
            core::ptr::null_mut(), // parent component, there is no parent.
        )
    };

    let mut component = Component {
        component,
        is_pushed: false,
    };
    component.screen_stack_push();

    core::future::poll_fn({
        let shared_state = Rc::clone(&shared_state);
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

pub async fn confirm_transaction_address_create(amount: &str, address: &str) -> bool {
    let _no_screensaver = crate::screen_saver::ScreensaverInhibitor::new();

    // Shared between the async context and the c callback
    struct SharedState {
        waker: Option<Waker>,
        result: Option<bool>,
    }
    let shared_state = Rc::new(RefCell::new(SharedState {
        waker: None,
        result: None,
    }));

    unsafe extern "C" fn callback(result: bool, user_data: *mut c_void) {
        let shared_state: Rc<RefCell<SharedState>> = unsafe { Rc::from_raw(user_data as *mut _) };
        let mut shared_state = shared_state.borrow_mut();
        shared_state.result = Some(result);
        if let Some(waker) = shared_state.waker.as_ref() {
            waker.wake_by_ref();
        }
    }

    let component = unsafe {
        bitbox02_sys::confirm_transaction_address_create(
            util::strings::str_to_cstr_vec(amount).unwrap().as_ptr(), // copied in C
            util::strings::str_to_cstr_vec(address).unwrap().as_ptr(), // copied in C
            Some(callback),
            Rc::into_raw(Rc::clone(&shared_state)) as *mut _, // passed to callback as `user_data`.
        )
    };

    let mut component = Component {
        component,
        is_pushed: false,
    };
    component.screen_stack_push();

    core::future::poll_fn({
        let shared_state = Rc::clone(&shared_state);
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

pub async fn confirm_transaction_fee_create(amount: &str, fee: &str, longtouch: bool) -> bool {
    let _no_screensaver = crate::screen_saver::ScreensaverInhibitor::new();

    // Shared between the async context and the c callback
    struct SharedState {
        waker: Option<Waker>,
        result: Option<bool>,
    }
    let shared_state = Rc::new(RefCell::new(SharedState {
        waker: None,
        result: None,
    }));

    unsafe extern "C" fn callback(result: bool, user_data: *mut c_void) {
        let shared_state: Rc<RefCell<SharedState>> = unsafe { Rc::from_raw(user_data as *mut _) };
        let mut shared_state = shared_state.borrow_mut();
        shared_state.result = Some(result);
        if let Some(waker) = shared_state.waker.as_ref() {
            waker.wake_by_ref();
        }
    }

    let component = unsafe {
        bitbox02_sys::confirm_transaction_fee_create(
            util::strings::str_to_cstr_vec(amount).unwrap().as_ptr(), // copied in C
            util::strings::str_to_cstr_vec(fee).unwrap().as_ptr(),    // copied in C
            longtouch,
            Some(callback),
            Rc::into_raw(Rc::clone(&shared_state)) as *mut _, // passed to callback as `user_data`.
        )
    };

    let mut component = Component {
        component,
        is_pushed: false,
    };
    component.screen_stack_push();

    core::future::poll_fn({
        let shared_state = Rc::clone(&shared_state);
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

pub fn trinary_input_string_set_input(component: &mut Component, word: &str) {
    unsafe {
        bitbox02_sys::trinary_input_string_set_input(
            component.component,
            util::strings::str_to_cstr_vec(word).unwrap().as_ptr(),
        )
    }
}

pub fn screen_stack_pop_all() {
    unsafe {
        bitbox02_sys::ui_screen_stack_pop_all();
    }
}

pub fn progress_create(title: &str) -> Component {
    let component = unsafe {
        bitbox02_sys::progress_create(
            util::strings::str_to_cstr_vec(title).unwrap().as_ptr(), // copied in C
        )
    };

    Component {
        component,
        is_pushed: false,
    }
}

pub fn progress_set(component: &mut Component, progress: f32) {
    unsafe { bitbox02_sys::progress_set(component.component, progress) }
}

pub fn empty_create() -> Component {
    Component {
        component: unsafe { bitbox02_sys::empty_create() },
        is_pushed: false,
    }
}

pub async fn unlock_animation() {
    let _no_screensaver = crate::screen_saver::ScreensaverInhibitor::new();

    // Shared between the async context and the c callback
    struct SharedState {
        waker: Option<Waker>,
        result: Option<()>,
    }
    let shared_state = Rc::new(RefCell::new(SharedState {
        waker: None,
        result: None,
    }));

    unsafe extern "C" fn callback(user_data: *mut c_void) {
        let shared_state: Rc<RefCell<SharedState>> = unsafe { Rc::from_raw(user_data as *mut _) };
        let mut shared_state = shared_state.borrow_mut();
        shared_state.result = Some(());
        if let Some(waker) = shared_state.waker.as_ref() {
            waker.wake_by_ref();
        }
    }

    let component = unsafe {
        bitbox02_sys::unlock_animation_create(
            Some(callback),
            Rc::into_raw(Rc::clone(&shared_state)) as *mut _, // passed to callback as `user_data`.
        )
    };

    let mut component = Component {
        component,
        is_pushed: false,
    };
    component.screen_stack_push();

    core::future::poll_fn({
        let shared_state = Rc::clone(&shared_state);
        move |cx| {
            let mut shared_state = shared_state.borrow_mut();

            if shared_state.result.is_some() {
                Poll::Ready(())
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
    let shared_state = Rc::new(RefCell::new(SharedState {
        waker: None,
        result: None,
    }));

    unsafe extern "C" fn callback(upside_down: bool, user_data: *mut c_void) {
        let shared_state: Rc<RefCell<SharedState>> = unsafe { Rc::from_raw(user_data as *mut _) };
        let mut shared_state = shared_state.borrow_mut();
        shared_state.result = Some(upside_down);
        if let Some(waker) = shared_state.waker.as_ref() {
            waker.wake_by_ref();
        }
    }

    let component = unsafe {
        bitbox02_sys::orientation_arrows_create(
            Some(callback),
            Rc::into_raw(Rc::clone(&shared_state)) as *mut _, // passed to callback as `user_data`.
        )
    };

    let mut component = Component {
        component,
        is_pushed: false,
    };
    component.screen_stack_push();

    core::future::poll_fn({
        let shared_state = Rc::clone(&shared_state);
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
