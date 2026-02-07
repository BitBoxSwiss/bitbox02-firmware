// SPDX-License-Identifier: Apache-2.0

//! Stubs for the Bitbox02 simulator and also C unit-tests.

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
    params: &TrinaryInputStringParams<'_>,
    _can_cancel: bool,
    _preset: &str,
) -> Result<zeroize::Zeroizing<String>, ()> {
    crate::print_stdout(&format!(
        "ENTER SCREEN START\nTITLE: {}\nENTER SCREEN END\n",
        params.title
    ));

    Ok(zeroize::Zeroizing::new("".into()))
}

pub async fn confirm(params: &ConfirmParams<'_>) -> bool {
    crate::print_stdout(&format!(
        "CONFIRM SCREEN START\nTITLE: {}\nBODY: {}\nCONFIRM SCREEN END\n",
        params.title, params.body
    ));

    true
}

pub fn screen_process() {}

pub fn status_create(text: &str, _status_success: bool) -> Component {
    crate::print_stdout(&format!(
        "STATUS SCREEN START\nTITLE: {}\nSTATUS SCREEN END\n",
        text,
    ));
    Component { is_pushed: false }
}

pub async fn sdcard() -> bool {
    true
}

pub async fn menu_create(_params: MenuParams<'_>) -> Result<u8, ()> {
    panic!("not implemented");
}

pub async fn trinary_choice(
    _message: &str,
    _label_left: Option<&str>,
    _label_middle: Option<&str>,
    _label_right: Option<&str>,
) -> TrinaryChoice {
    panic!("not implemented")
}

pub async fn confirm_transaction_address_create(_amount: &str, _address: &str) -> bool {
    crate::print_stdout(&format!(
        "CONFIRM TRANSACTION ADDRESS SCREEN START\nAMOUNT: {}\nADDRESS: {}\nCONFIRM TRANSACTION ADDRESS SCREEN END\n",
        _amount, _address
    ));
    true
}

pub async fn confirm_transaction_fee_create(_amount: &str, _fee: &str, _longtouch: bool) -> bool {
    crate::print_stdout(&format!(
        "CONFIRM TRANSACTION FEE SCREEN START\nAMOUNT: {}\nFEE: {}\nCONFIRM TRANSACTION FEE SCREEN END\n",
        _amount, _fee
    ));
    true
}

pub fn trinary_input_string_set_input(_component: &mut Component, _word: &str) {
    panic!("not implemented")
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
