// SPDX-License-Identifier: Apache-2.0

//! Stubs for testing.
//!
//! NOTE: This is on the way out - we use the Workflows trait and TestingWorkflow to unit test
//! workflows now.

pub use super::types::{
    AcceptRejectCb, ConfirmParams, ConfirmResponse, ContinueCancelCb, Font, MenuParams,
    MenuResponse, SdcardResponse, SelectWordCb, TrinaryChoice, TrinaryChoiceCb,
    TrinaryInputStringParams,
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

pub async fn confirm(_params: &ConfirmParams<'_>) -> ConfirmResponse {
    panic!("not used");
}

pub fn screen_process() {}

pub fn status_create(_text: &str, _status_success: bool) -> Component {
    panic!("not used");
}

pub async fn sdcard() -> SdcardResponse {
    panic!("not used");
}

pub async fn menu(_params: MenuParams<'_>) -> MenuResponse {
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

pub async fn confirm_transaction_address(_amount: &str, _address: &str) -> ConfirmResponse {
    panic!("not used");
}

pub async fn confirm_transaction_fee(
    _amount: &str,
    _fee: &str,
    _longtouch: bool,
) -> ConfirmResponse {
    panic!("not used");
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
