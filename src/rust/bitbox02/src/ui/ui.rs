// SPDX-License-Identifier: Apache-2.0

pub use super::types::{
    AcceptRejectCb, ConfirmParams, ContinueCancelCb, Font, MenuParams, SelectWordCb, TrinaryChoice,
    TrinaryChoiceCb, TrinaryInputStringParams,
};

use core::ffi::{c_char, c_void};

extern crate alloc;
use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::string::String;
use alloc::vec::Vec;
use core::cell::RefCell;
use core::task::{Poll, Waker};

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
    unsafe extern "C" fn c_confirm_callback<F2>(password: *const c_char, user_data: *mut c_void)
    where
        F2: FnMut(zeroize::Zeroizing<String>),
    {
        let pw: zeroize::Zeroizing<String> = zeroize::Zeroizing::new(
            unsafe { util::strings::str_from_null_terminated_ptr(password) }
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
        _p: PhantomData,
    }
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
        on_drop: None,
        _p: PhantomData,
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

pub fn status_create<'a>(text: &str, status_success: bool) -> Component<'a> {
    let component = unsafe {
        bitbox02_sys::status_create(
            util::strings::str_to_cstr_vec(text).unwrap().as_ptr(), // copied in C
            status_success,
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
    unsafe extern "C" fn c_callback<F2>(sd_done: bool, user_data: *mut c_void)
    where
        F2: FnMut(bool),
    {
        // The callback is dropped afterwards. This is safe because
        // this C callback is guaranteed to be called only once.
        let mut callback = unsafe { Box::from_raw(user_data as *mut F2) };
        callback(sd_done);
    }

    let component = unsafe {
        bitbox02_sys::sdcard_create(
            Some(c_callback::<F>),
            // passed to the C callback as `user_data`
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
        .map(|word| util::strings::str_to_cstr_vec(word).unwrap())
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
        _p: PhantomData,
    }
}

pub fn trinary_choice_create<'a>(
    message: &'a str,
    label_left: Option<&'a str>,
    label_middle: Option<&'a str>,
    label_right: Option<&'a str>,
    chosen_callback: TrinaryChoiceCb,
) -> Component<'a> {
    unsafe extern "C" fn c_chosen_cb(choice: TrinaryChoice, user_data: *mut c_void) {
        let callback = user_data as *mut TrinaryChoiceCb;
        unsafe { (*callback)(choice) };
    }

    let chosen_user_data = Box::into_raw(Box::new(chosen_callback)) as *mut c_void;

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
            Some(c_chosen_cb as _),
            chosen_user_data,
            core::ptr::null_mut(), // parent component, there is no parent.
        )
    };
    Component {
        component,
        is_pushed: false,
        on_drop: Some(Box::new(move || unsafe {
            // Drop all callbacks.
            drop(Box::from_raw(chosen_user_data as *mut TrinaryChoiceCb));
        })),
        _p: PhantomData,
    }
}

pub fn confirm_transaction_address_create<'a, 'b>(
    amount: &'a str,
    address: &'a str,
    callback: AcceptRejectCb<'b>,
) -> Component<'b> {
    unsafe extern "C" fn c_callback(result: bool, user_data: *mut c_void) {
        let callback = user_data as *mut AcceptRejectCb;
        unsafe { (*callback)(result) };
    }

    let user_data = Box::into_raw(Box::new(callback)) as *mut c_void;
    let component = unsafe {
        bitbox02_sys::confirm_transaction_address_create(
            util::strings::str_to_cstr_vec(amount).unwrap().as_ptr(), // copied in C
            util::strings::str_to_cstr_vec(address).unwrap().as_ptr(), // copied in C
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
        _p: PhantomData,
    }
}

pub fn confirm_transaction_fee_create<'a, 'b>(
    amount: &'a str,
    fee: &'a str,
    longtouch: bool,
    callback: AcceptRejectCb<'b>,
) -> Component<'b> {
    unsafe extern "C" fn c_callback(result: bool, user_data: *mut c_void) {
        let callback = user_data as *mut AcceptRejectCb;
        unsafe { (*callback)(result) };
    }

    let user_data = Box::into_raw(Box::new(callback)) as *mut c_void;
    let component = unsafe {
        bitbox02_sys::confirm_transaction_fee_create(
            util::strings::str_to_cstr_vec(amount).unwrap().as_ptr(), // copied in C
            util::strings::str_to_cstr_vec(fee).unwrap().as_ptr(),    // copied in C
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
        _p: PhantomData,
    }
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

pub fn progress_create<'a>(title: &str) -> Component<'a> {
    let component = unsafe {
        bitbox02_sys::progress_create(
            util::strings::str_to_cstr_vec(title).unwrap().as_ptr(), // copied in C
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

pub fn unlock_animation_create<'a, F>(on_done: F) -> Component<'a>
where
    // Callback must outlive component.
    F: FnMut() + 'a,
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
        _p: PhantomData,
    }
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
        on_drop: None,
        _p: PhantomData,
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
