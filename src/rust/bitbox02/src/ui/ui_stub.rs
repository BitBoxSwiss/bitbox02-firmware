// SPDX-License-Identifier: Apache-2.0

//! Stubs for testing.
//!
//! NOTE: This is on the way out - we use the Workflows trait and TestingWorkflow to unit test
//! workflows now.

pub use super::types::{
    AcceptRejectCb, ConfirmParams, ContinueCancelCb, Font, MenuParams, SelectWordCb, TrinaryChoice,
    TrinaryChoiceCb, TrinaryInputStringParams,
};

use core::marker::PhantomData;

extern crate alloc;

use alloc::string::String;

pub struct Component<'a> {
    is_pushed: bool,
    _p: PhantomData<&'a ()>,
}

impl Component<'_> {
    pub fn screen_stack_push(&mut self) {
        if self.is_pushed {
            panic!("component pushed twice");
        }
        self.is_pushed = true;
    }
}

impl Drop for Component<'_> {
    fn drop(&mut self) {
        if !self.is_pushed {
            panic!("component not pushed");
        }
    }
}

pub fn trinary_input_string_create<'a, F>(
    _params: &TrinaryInputStringParams,
    _confirm_callback: F,
    _cancel_callback: Option<ContinueCancelCb<'a>>,
) -> Component<'a>
where
    F: FnMut(zeroize::Zeroizing<String>) + 'a,
{
    panic!("not used");
}

pub async fn confirm(_params: &ConfirmParams<'_>) -> bool {
    panic!("not used");
}

pub fn screen_process() {}

pub fn status_create<'a>(_text: &str, _status_success: bool) -> Component<'a> {
    panic!("not used");
}

pub fn sdcard_create<'a, F>(_callback: F) -> Component<'a>
where
    F: FnMut(bool) + 'a,
{
    panic!("not used");
}

pub fn menu_create(_params: MenuParams<'_>) -> Component<'_> {
    panic!("not used");
}

pub fn trinary_choice_create<'a>(
    _message: &'a str,
    _label_left: Option<&'a str>,
    _label_middle: Option<&'a str>,
    _label_right: Option<&'a str>,
    _chosen_callback: TrinaryChoiceCb,
) -> Component<'a> {
    panic!("not used")
}

pub fn confirm_transaction_address_create<'a, 'b>(
    _amount: &'a str,
    _address: &'a str,
    _callback: AcceptRejectCb<'b>,
) -> Component<'b> {
    panic!("not used");
}

pub fn confirm_transaction_fee_create<'a, 'b>(
    _amount: &'a str,
    _fee: &'a str,
    _longtouch: bool,
    _callback: AcceptRejectCb<'b>,
) -> Component<'b> {
    panic!("not used");
}

pub fn trinary_input_string_set_input(_component: &mut Component, _word: &str) {
    panic!("not used")
}

pub fn screen_stack_pop_all() {}

pub fn progress_create<'a>(_title: &str) -> Component<'a> {
    Component {
        is_pushed: false,
        _p: PhantomData,
    }
}

pub fn progress_set(_component: &mut Component, _progress: f32) {}

pub fn empty_create<'a>() -> Component<'a> {
    Component {
        is_pushed: false,
        _p: PhantomData,
    }
}

pub fn unlock_animation_create<'a, F>(mut on_done: F) -> Component<'a>
where
    F: FnMut() + 'a,
{
    on_done();
    Component {
        is_pushed: false,
        _p: PhantomData,
    }
}

pub async fn choose_orientation() -> bool {
    false
}
