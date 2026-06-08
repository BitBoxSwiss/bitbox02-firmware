// SPDX-License-Identifier: Apache-2.0

pub use super::types::{
    AcceptRejectCb, ConfirmParams, ConfirmResponse, ContinueCancelCb, Font, MenuParams,
    MenuResponse, SdcardResponse, SelectWordCb, TrinaryChoice, TrinaryChoiceCb,
    TrinaryInputStringParams,
};

use core::ffi::{c_char, c_void};

extern crate alloc;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::cell::RefCell;
use core::task::{Poll, Waker};

// Keep enough bytes beyond the C label limit to prove that truncation is needed even when the
// Rust-side cut moves back to a UTF-8 boundary.
const LABEL_TRUNCATE_SIZE: usize = super::types::MAX_LABEL_SIZE + 4;

/// Keep this check at the common UI boundary so unsupported text cannot be silently omitted by
/// the renderer.
fn display_str_to_cstr_vec(text: &str) -> Vec<c_char> {
    assert!(
        util::display::is_safe_text(text, true)
            && text
                .chars()
                .all(|c| c == '\n' || Font::Default.has_glyph(c)),
        "BitBox02 UI text contains unsupported characters"
    );
    let mut result: Vec<c_char> = text.bytes().map(|byte| byte as c_char).collect();
    result.push(0);
    result
}

fn label_fits_width(text: &str, font: *const bitbox02_sys::UG_FONT) -> bool {
    let text = display_str_to_cstr_vec(text);
    unsafe { bitbox02_sys::label_fits_width(text.as_ptr(), font, bitbox02_sys::SCREEN_WIDTH as _) }
}

/// Returns true if the amount fits the transaction screen, using the smaller font if necessary.
pub fn transaction_amount_fits(amount: &str) -> bool {
    if label_fits_width(amount, unsafe { &bitbox02_sys::font_arial_11 }) {
        true
    } else {
        label_fits_width(amount, unsafe { &bitbox02_sys::font_arial_9 })
    }
}

/// Returns true if the fee fits the transaction screen's smaller fee font.
pub fn transaction_fee_fits(fee: &str) -> bool {
    label_fits_width(fee, unsafe { &bitbox02_sys::font_arial_9 })
}

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

struct UnlockAnimationSharedState {
    waker: Option<Waker>,
    result: Option<()>,
}

pub struct UnlockAnimation {
    component: Component,
    shared_state: Box<RefCell<UnlockAnimationSharedState>>,
}

impl UnlockAnimation {
    pub async fn play(self) {
        let _no_screensaver = crate::screen_saver::ScreensaverInhibitor::new();
        unsafe {
            bitbox02_sys::unlock_animation_play(self.component.component);
        }

        let UnlockAnimation {
            component: _component,
            shared_state,
        } = self;
        core::future::poll_fn(move |cx| {
            let mut shared_state = shared_state.borrow_mut();

            if let Some(result) = shared_state.result.take() {
                Poll::Ready(result)
            } else {
                shared_state.waker = Some(cx.waker().clone());
                Poll::Pending
            }
        })
        .await
    }
}

pub async fn trinary_input_string(
    params: &TrinaryInputStringParams<'_>,
    can_cancel: bool,
    preset: &str,
) -> Result<zeroize::Zeroizing<String>, ()> {
    // Shared between the async context and the c callback
    struct SharedState {
        waker: Option<Waker>,
        result: Option<Result<zeroize::Zeroizing<String>, ()>>,
    }
    let shared_state = Box::new(RefCell::new(SharedState {
        waker: None,
        result: None,
    }));
    let shared_state_ptr = shared_state.as_ref() as *const RefCell<SharedState> as *mut c_void;

    unsafe extern "C" fn cancel_cb(user_data: *mut c_void) {
        let shared_state = unsafe { &*(user_data as *mut RefCell<SharedState>) };
        let mut shared_state = shared_state.borrow_mut();
        if shared_state.result.is_none() {
            shared_state.result = Some(Err(()));
            if let Some(waker) = shared_state.waker.as_ref() {
                waker.wake_by_ref();
            }
        }
    }

    unsafe extern "C" fn confirm_cb(password: *const c_char, user_data: *mut c_void) {
        let shared_state = unsafe { &*(user_data as *mut RefCell<SharedState>) };
        let mut shared_state = shared_state.borrow_mut();
        let pw: zeroize::Zeroizing<String> = zeroize::Zeroizing::new(
            unsafe { util::strings::str_from_null_terminated_ptr(password) }
                .unwrap()
                .into(),
        );
        if shared_state.result.is_none() {
            shared_state.result = Some(Ok(pw));
            if let Some(waker) = shared_state.waker.as_ref() {
                waker.wake_by_ref();
            }
        }
    }

    let (actual_cancel_cb, cancel_shared_state) = if can_cancel {
        (
            Some(cancel_cb as unsafe extern "C" fn(*mut c_void)),
            shared_state_ptr,
        )
    } else {
        (None, core::ptr::null_mut())
    };

    let title = display_str_to_cstr_vec(util::strings::truncate_str(
        params.title,
        LABEL_TRUNCATE_SIZE,
    ));
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
            shared_state_ptr, // passed to confirm_cb as `user_data`.
            actual_cancel_cb,
            cancel_shared_state, // passed to cancel_cb as `user_data`.
        )
    };
    if !preset.is_empty() {
        unsafe {
            bitbox02_sys::trinary_input_string_set_input(
                component,
                display_str_to_cstr_vec(preset).as_ptr(),
            )
        }
    }

    let mut component = Component {
        component,
        is_pushed: false,
    };
    component.screen_stack_push();

    core::future::poll_fn({
        let shared_state = &shared_state;
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

/// Returns `ConfirmResponse::Approved` if the user accepts,
/// `ConfirmResponse::Cancelled` if the user rejects.
pub async fn confirm(params: &ConfirmParams<'_>) -> ConfirmResponse {
    let _no_screensaver = crate::screen_saver::ScreensaverInhibitor::new();

    // Shared between the async context and the c callback
    struct SharedState {
        waker: Option<Waker>,
        result: Option<ConfirmResponse>,
    }
    let shared_state = Box::new(RefCell::new(SharedState {
        waker: None,
        result: None,
    }));
    let shared_state_ptr = shared_state.as_ref() as *const RefCell<SharedState> as *mut c_void;

    unsafe extern "C" fn callback(result: bool, user_data: *mut c_void) {
        let shared_state = unsafe { &*(user_data as *mut RefCell<SharedState>) };
        let mut shared_state = shared_state.borrow_mut();
        if shared_state.result.is_none() {
            shared_state.result = Some(if result {
                ConfirmResponse::Approved
            } else {
                ConfirmResponse::Cancelled
            });
            if let Some(waker) = shared_state.waker.as_ref() {
                waker.wake_by_ref();
            }
        }
    }

    let title = display_str_to_cstr_vec(util::strings::truncate_str(
        params.title,
        LABEL_TRUNCATE_SIZE,
    ));
    let body = display_str_to_cstr_vec(util::strings::truncate_str(
        params.body,
        LABEL_TRUNCATE_SIZE,
    ));
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
            shared_state_ptr, // passed to callback as `user_data`.
        )
    };

    let mut component = Component {
        component,
        is_pushed: false,
    };
    component.screen_stack_push();

    core::future::poll_fn({
        let shared_state = &shared_state;
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

pub fn screen_process() {
    unsafe {
        bitbox02_sys::screen_process(false);
    }
}

pub fn status_create(text: &str, status_success: bool) -> Component {
    let component = unsafe {
        bitbox02_sys::status_create(
            display_str_to_cstr_vec(text).as_ptr(), // copied in C
            status_success,
        )
    };
    Component {
        component,
        is_pushed: false,
    }
}

pub async fn sdcard() -> SdcardResponse {
    let _no_screensaver = crate::screen_saver::ScreensaverInhibitor::new();

    // Shared between the async context and the c callback
    struct SharedState {
        waker: Option<Waker>,
        result: Option<SdcardResponse>,
    }
    let shared_state = Box::new(RefCell::new(SharedState {
        waker: None,
        result: None,
    }));
    let shared_state_ptr = shared_state.as_ref() as *const RefCell<SharedState> as *mut c_void;

    unsafe extern "C" fn callback(result: bool, user_data: *mut c_void) {
        let shared_state = unsafe { &*(user_data as *mut RefCell<SharedState>) };
        let mut shared_state = shared_state.borrow_mut();
        let response = if result {
            SdcardResponse::Inserted
        } else {
            SdcardResponse::Cancelled
        };
        if shared_state.result.is_none() {
            shared_state.result = Some(response);
            if let Some(waker) = shared_state.waker.as_ref() {
                waker.wake_by_ref();
            }
        }
    }

    let component = unsafe {
        bitbox02_sys::sdcard_create(
            Some(callback),
            shared_state_ptr, // passed to callback as `user_data`.
        )
    };

    let mut component = Component {
        component,
        is_pushed: false,
    };
    component.screen_stack_push();

    core::future::poll_fn({
        let shared_state = &shared_state;
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

pub async fn menu(params: MenuParams<'_>) -> MenuResponse {
    let _no_screensaver = crate::screen_saver::ScreensaverInhibitor::new();
    let cancel_confirm_title = params.cancel_confirm_title;

    // Shared between the async context and the c callback
    struct SharedState {
        waker: Option<Waker>,
        result: Option<MenuResponse>,
    }
    let shared_state = Box::new(RefCell::new(SharedState {
        waker: None,
        result: None,
    }));
    let shared_state_ptr = shared_state.as_ref() as *const RefCell<SharedState> as *mut c_void;

    unsafe extern "C" fn select_word_cb(word_idx: u8, user_data: *mut c_void) {
        let shared_state = unsafe { &*(user_data as *mut RefCell<SharedState>) };
        let mut shared_state = shared_state.borrow_mut();
        if shared_state.result.is_none() {
            shared_state.result = Some(MenuResponse::SelectWord(word_idx));
            if let Some(waker) = shared_state.waker.as_ref() {
                waker.wake_by_ref();
            }
        }
    }

    unsafe extern "C" fn continue_on_last_cb(user_data: *mut c_void) {
        let shared_state = unsafe { &*(user_data as *mut RefCell<SharedState>) };
        let mut shared_state = shared_state.borrow_mut();
        if shared_state.result.is_none() {
            shared_state.result = Some(MenuResponse::ContinueOnLast);
            if let Some(waker) = shared_state.waker.as_ref() {
                waker.wake_by_ref();
            }
        }
    }

    unsafe extern "C" fn cancel_cb(user_data: *mut c_void) {
        let shared_state = unsafe { &*(user_data as *mut RefCell<SharedState>) };
        let mut shared_state = shared_state.borrow_mut();
        if shared_state.result.is_none() {
            shared_state.result = Some(MenuResponse::Cancel);
            if let Some(waker) = shared_state.waker.as_ref() {
                waker.wake_by_ref();
            }
        }
    }

    // We want to turn &[&str] into a C char**.
    //
    // Step 1: create the C strings. This var has to be alive until after menu() finishes,
    // otherwise the pointers we send to menu_create() will be invalid.
    let words: Vec<Vec<core::ffi::c_char>> = params
        .words
        .iter()
        .map(|word| display_str_to_cstr_vec(word))
        .collect();
    // Step two: collect pointers. This var also has to be valid until menu() finishes, or
    // the pointer will be invalid.
    let c_words: Vec<*const core::ffi::c_char> =
        words.iter().map(|word| word.as_ptr() as _).collect();

    let (select_word_cb, select_word_user_data) = match params.select_word {
        false => (None, core::ptr::null_mut()),
        true => (
            Some(select_word_cb as _),
            shared_state_ptr, // passed to select_word_cb as `user_data`.
        ),
    };

    let (continue_on_last_cb, continue_on_last_user_data) = match params.continue_on_last {
        false => (None, core::ptr::null_mut()),
        true => (
            Some(continue_on_last_cb as _),
            shared_state_ptr, // passed to continue_on_last_cb as `user_data`.
        ),
    };
    let title = params.title.map(display_str_to_cstr_vec);
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
            Some(cancel_cb as _),
            shared_state_ptr, // passed to cancel_cb as `user_data`.
            core::ptr::null_mut(),
        )
    };
    let mut component = Component {
        component,
        is_pushed: false,
    };
    component.screen_stack_push();

    loop {
        let result = core::future::poll_fn({
            let shared_state = &shared_state;
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
        .await;

        match result {
            MenuResponse::SelectWord(_) | MenuResponse::ContinueOnLast => return result,
            MenuResponse::Cancel => match cancel_confirm_title {
                None => return MenuResponse::Cancel,
                Some(title) => {
                    // `ConfirmResponse::Cancelled` means _do not cancel_,
                    // stay in the same menu component.
                    match confirm(&ConfirmParams {
                        title,
                        body: "Do you really\nwant to cancel?",
                        ..Default::default()
                    })
                    .await
                    {
                        ConfirmResponse::Approved => return MenuResponse::Cancel,
                        ConfirmResponse::Cancelled => continue,
                    }
                }
            },
        }
    }
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
    let shared_state = Box::new(RefCell::new(SharedState {
        waker: None,
        result: None,
    }));
    let shared_state_ptr = shared_state.as_ref() as *const RefCell<SharedState> as *mut c_void;

    unsafe extern "C" fn callback(choice: TrinaryChoice, user_data: *mut c_void) {
        let shared_state = unsafe { &*(user_data as *mut RefCell<SharedState>) };
        let mut shared_state = shared_state.borrow_mut();
        if shared_state.result.is_none() {
            shared_state.result = Some(choice);
            if let Some(waker) = shared_state.waker.as_ref() {
                waker.wake_by_ref();
            }
        }
    }

    let label_left = label_left.map(display_str_to_cstr_vec);
    let label_middle = label_middle.map(display_str_to_cstr_vec);
    let label_right = label_right.map(display_str_to_cstr_vec);

    let component = unsafe {
        bitbox02_sys::trinary_choice_create(
            display_str_to_cstr_vec(message).as_ptr(), // copied in C
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
            shared_state_ptr,      // passed to callback as `user_data`.
            core::ptr::null_mut(), // parent component, there is no parent.
        )
    };

    let mut component = Component {
        component,
        is_pushed: false,
    };
    component.screen_stack_push();

    core::future::poll_fn({
        let shared_state = &shared_state;
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

pub async fn confirm_transaction_address(amount: &str, address: &str) -> ConfirmResponse {
    let _no_screensaver = crate::screen_saver::ScreensaverInhibitor::new();

    // Shared between the async context and the c callback
    struct SharedState {
        waker: Option<Waker>,
        result: Option<ConfirmResponse>,
    }
    let shared_state = Box::new(RefCell::new(SharedState {
        waker: None,
        result: None,
    }));
    let shared_state_ptr = shared_state.as_ref() as *const RefCell<SharedState> as *mut c_void;

    unsafe extern "C" fn callback(result: bool, user_data: *mut c_void) {
        let shared_state = unsafe { &*(user_data as *mut RefCell<SharedState>) };
        let mut shared_state = shared_state.borrow_mut();
        if shared_state.result.is_none() {
            shared_state.result = Some(if result {
                ConfirmResponse::Approved
            } else {
                ConfirmResponse::Cancelled
            });
            if let Some(waker) = shared_state.waker.as_ref() {
                waker.wake_by_ref();
            }
        }
    }

    let component = unsafe {
        bitbox02_sys::confirm_transaction_address_create(
            display_str_to_cstr_vec(amount).as_ptr(),  // copied in C
            display_str_to_cstr_vec(address).as_ptr(), // copied in C
            Some(callback),
            shared_state_ptr, // passed to callback as `user_data`.
        )
    };

    let mut component = Component {
        component,
        is_pushed: false,
    };
    component.screen_stack_push();

    core::future::poll_fn({
        let shared_state = &shared_state;
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

pub async fn confirm_swap(title: &str, from: &str, to: &str) -> ConfirmResponse {
    let _no_screensaver = crate::screen_saver::ScreensaverInhibitor::new();

    // Shared between the async context and the c callback
    struct SharedState {
        waker: Option<Waker>,
        result: Option<ConfirmResponse>,
    }
    let shared_state = Box::new(RefCell::new(SharedState {
        waker: None,
        result: None,
    }));
    let shared_state_ptr = shared_state.as_ref() as *const RefCell<SharedState> as *mut c_void;

    unsafe extern "C" fn callback(result: bool, user_data: *mut c_void) {
        let shared_state = unsafe { &*(user_data as *mut RefCell<SharedState>) };
        let mut shared_state = shared_state.borrow_mut();
        if shared_state.result.is_none() {
            shared_state.result = Some(if result {
                ConfirmResponse::Approved
            } else {
                ConfirmResponse::Cancelled
            });
            if let Some(waker) = shared_state.waker.as_ref() {
                waker.wake_by_ref();
            }
        }
    }

    let component = unsafe {
        bitbox02_sys::confirm_swap_create(
            display_str_to_cstr_vec(title).as_ptr(), // copied in C
            display_str_to_cstr_vec(from).as_ptr(),  // copied in C
            display_str_to_cstr_vec(to).as_ptr(),    // copied in C
            Some(callback),
            shared_state_ptr, // passed to callback as `user_data`.
        )
    };

    let mut component = Component {
        component,
        is_pushed: false,
    };
    component.screen_stack_push();

    core::future::poll_fn({
        let shared_state = &shared_state;
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

pub async fn confirm_transaction_fee(amount: &str, fee: &str, longtouch: bool) -> ConfirmResponse {
    let _no_screensaver = crate::screen_saver::ScreensaverInhibitor::new();

    // Shared between the async context and the c callback
    struct SharedState {
        waker: Option<Waker>,
        result: Option<ConfirmResponse>,
    }
    let shared_state = Box::new(RefCell::new(SharedState {
        waker: None,
        result: None,
    }));
    let shared_state_ptr = shared_state.as_ref() as *const RefCell<SharedState> as *mut c_void;

    unsafe extern "C" fn callback(result: bool, user_data: *mut c_void) {
        let shared_state = unsafe { &*(user_data as *mut RefCell<SharedState>) };
        let mut shared_state = shared_state.borrow_mut();
        if shared_state.result.is_none() {
            shared_state.result = Some(if result {
                ConfirmResponse::Approved
            } else {
                ConfirmResponse::Cancelled
            });
            if let Some(waker) = shared_state.waker.as_ref() {
                waker.wake_by_ref();
            }
        }
    }

    let component = unsafe {
        bitbox02_sys::confirm_transaction_fee_create(
            display_str_to_cstr_vec(amount).as_ptr(), // copied in C
            display_str_to_cstr_vec(fee).as_ptr(),    // copied in C
            longtouch,
            Some(callback),
            shared_state_ptr, // passed to callback as `user_data`.
        )
    };

    let mut component = Component {
        component,
        is_pushed: false,
    };
    component.screen_stack_push();

    core::future::poll_fn({
        let shared_state = &shared_state;
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

pub fn screen_stack_pop_all() {
    unsafe {
        bitbox02_sys::ui_screen_stack_pop_all();
    }
}

pub fn progress_create(title: &str) -> Component {
    let component = unsafe {
        bitbox02_sys::progress_create(
            display_str_to_cstr_vec(title).as_ptr(), // copied in C
        )
    };

    Component {
        component,
        is_pushed: false,
    }
}

pub fn progress_set_fraction(component: &mut Component, numerator: u32, denominator: u32) {
    unsafe { bitbox02_sys::progress_set_fraction(component.component, numerator, denominator) }
}

pub fn empty_create() -> Component {
    Component {
        component: unsafe { bitbox02_sys::empty_create() },
        is_pushed: false,
    }
}

pub fn unlock_animation_create() -> UnlockAnimation {
    let shared_state = Box::new(RefCell::new(UnlockAnimationSharedState {
        waker: None,
        result: None,
    }));
    let shared_state_ptr =
        shared_state.as_ref() as *const RefCell<UnlockAnimationSharedState> as *mut c_void;

    unsafe extern "C" fn callback(user_data: *mut c_void) {
        let shared_state = unsafe { &*(user_data as *mut RefCell<UnlockAnimationSharedState>) };
        let mut shared_state = shared_state.borrow_mut();
        if shared_state.result.is_none() {
            shared_state.result = Some(());
            if let Some(waker) = shared_state.waker.as_ref() {
                waker.wake_by_ref();
            }
        }
    }

    let component = unsafe {
        bitbox02_sys::unlock_animation_create(
            Some(callback),
            shared_state_ptr, // passed to callback as `user_data`.
        )
    };

    let mut component = Component {
        component,
        is_pushed: false,
    };
    component.screen_stack_push();

    UnlockAnimation {
        component,
        shared_state,
    }
}

pub async fn choose_orientation() -> bool {
    // Shared between the async context and the c callback
    struct SharedState {
        waker: Option<Waker>,
        result: Option<bool>,
    }
    let shared_state = Box::new(RefCell::new(SharedState {
        waker: None,
        result: None,
    }));
    let shared_state_ptr = shared_state.as_ref() as *const RefCell<SharedState> as *mut c_void;

    unsafe extern "C" fn callback(upside_down: bool, user_data: *mut c_void) {
        let shared_state = unsafe { &*(user_data as *mut RefCell<SharedState>) };
        let mut shared_state = shared_state.borrow_mut();
        shared_state.result = Some(upside_down);
        if let Some(waker) = shared_state.waker.as_ref() {
            waker.wake_by_ref();
        }
    }

    let component = unsafe {
        bitbox02_sys::orientation_arrows_create(
            Some(callback),
            shared_state_ptr, // passed to callback as `user_data`.
        )
    };

    let mut component = Component {
        component,
        is_pushed: false,
    };
    component.screen_stack_push();

    core::future::poll_fn({
        let shared_state = &shared_state;
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
