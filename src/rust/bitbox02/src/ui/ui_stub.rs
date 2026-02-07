// SPDX-License-Identifier: Apache-2.0

//! Stubs for testing.
//!
//! NOTE: This is on the way out - we use the Workflows trait and TestingWorkflow to unit test
//! workflows now.

pub use super::types::{
    AcceptRejectCb, ConfirmParams, ContinueCancelCb, Font, MenuParams, SelectWordCb, TrinaryChoice,
    TrinaryChoiceCb, TrinaryInputStringParams,
};

extern crate alloc;

use alloc::string::String;

pub struct Component {
    is_pushed: bool,
}

impl Component {
    pub fn screen_stack_push(&mut self) {
        if self.is_pushed {
            panic!("component pushed twice");
        }
        self.is_pushed = true;
    }
}

impl Drop for Component {
    fn drop(&mut self) {
        if !self.is_pushed {
            panic!("component not pushed");
        }
    }
}

pub async fn trinary_input_string(
    _params: &TrinaryInputStringParams<'_>,
    _can_cancel: bool,
    _preset: &str,
) -> Result<zeroize::Zeroizing<String>, ()> {
    panic!("not used");
}

pub async fn confirm(_params: &ConfirmParams<'_>) -> bool {
    panic!("not used");
}

pub fn screen_process() {}

pub fn status_create(_text: &str, _status_success: bool) -> Component {
    panic!("not used");
}

pub async fn sdcard() -> bool {
    panic!("not used");
}

pub async fn menu_create(_params: MenuParams<'_>) -> Result<u8, ()> {
    panic!("not used");
}

pub async fn trinary_choice(
    _message: &str,
    _label_left: Option<&str>,
    _label_middle: Option<&str>,
    _label_right: Option<&str>,
) -> TrinaryChoice {
    panic!("not used")
}

pub async fn confirm_transaction_address_create(_amount: &str, _address: &str) -> bool {
    panic!("not used");
}

pub async fn confirm_transaction_fee_create(_amount: &str, _fee: &str, _longtouch: bool) -> bool {
    panic!("not used");
}

pub fn trinary_input_string_set_input(_component: &mut Component, _word: &str) {
    panic!("not used")
}

pub fn screen_stack_pop_all() {}

pub fn progress_create(_title: &str) -> Component {
    Component { is_pushed: false }
}

pub fn progress_set(_component: &mut Component, _progress: f32) {}

pub fn empty_create() -> Component {
    Component { is_pushed: false }
}

pub async fn unlock_animation() {}

pub async fn choose_orientation() -> bool {
    false
}
