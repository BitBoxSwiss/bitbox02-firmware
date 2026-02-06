// SPDX-License-Identifier: Apache-2.0

//! Stubs for the Bitbox02 simulator and also C unit-tests.

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

impl<'a> Component<'a> {
    pub fn screen_stack_push(&mut self) {
        if self.is_pushed {
            panic!("component pushed twice");
        }
        self.is_pushed = true;
    }
}

impl<'a> Drop for Component<'a> {
    fn drop(&mut self) {
        if !self.is_pushed {
            panic!("component not pushed");
        }
    }
}

pub fn trinary_input_string_create<'a, F>(
    params: &TrinaryInputStringParams,
    mut confirm_callback: F,
    _cancel_callback: Option<ContinueCancelCb<'a>>,
) -> Component<'a>
where
    F: FnMut(zeroize::Zeroizing<String>) + 'a,
{
    crate::print_stdout(&format!(
        "ENTER SCREEN START\nTITLE: {}\nENTER SCREEN END\n",
        params.title
    ));

    confirm_callback(zeroize::Zeroizing::new("".into()));
    Component {
        is_pushed: false,
        _p: PhantomData,
    }
}

pub async fn confirm(params: &ConfirmParams<'_>) -> bool {
    crate::print_stdout(&format!(
        "CONFIRM SCREEN START\nTITLE: {}\nBODY: {}\nCONFIRM SCREEN END\n",
        params.title, params.body
    ));

    true
}

pub fn screen_process() {}

pub fn status_create<'a>(text: &str, _status_success: bool) -> Component<'a> {
    crate::print_stdout(&format!(
        "STATUS SCREEN START\nTITLE: {}\nSTATUS SCREEN END\n",
        text,
    ));
    Component {
        is_pushed: false,
        _p: PhantomData,
    }
}

pub fn sdcard_create<'a, F>(mut callback: F) -> Component<'a>
where
    F: FnMut(bool) + 'a,
{
    callback(true);
    Component {
        is_pushed: false,
        _p: PhantomData,
    }
}

pub fn menu_create(_params: MenuParams<'_>) -> Component<'_> {
    panic!("not implemented");
}

pub fn trinary_choice_create<'a>(
    _message: &'a str,
    _label_left: Option<&'a str>,
    _label_middle: Option<&'a str>,
    _label_right: Option<&'a str>,
    _chosen_callback: TrinaryChoiceCb,
) -> Component<'a> {
    panic!("not implemented")
}

pub fn confirm_transaction_address_create<'a, 'b>(
    _amount: &'a str,
    _address: &'a str,
    mut callback: AcceptRejectCb<'b>,
) -> Component<'b> {
    crate::print_stdout(&format!(
        "CONFIRM TRANSACTION ADDRESS SCREEN START\nAMOUNT: {}\nADDRESS: {}\nCONFIRM TRANSACTION ADDRESS SCREEN END\n",
        _amount, _address
    ));
    callback(true);
    Component {
        is_pushed: false,
        _p: PhantomData,
    }
}

pub fn confirm_transaction_fee_create<'a, 'b>(
    _amount: &'a str,
    _fee: &'a str,
    _longtouch: bool,
    mut callback: AcceptRejectCb<'b>,
) -> Component<'b> {
    crate::print_stdout(&format!(
        "CONFIRM TRANSACTION FEE SCREEN START\nAMOUNT: {}\nFEE: {}\nCONFIRM TRANSACTION FEE SCREEN END\n",
        _amount, _fee
    ));
    callback(true);
    Component {
        is_pushed: false,
        _p: PhantomData,
    }
}

pub fn trinary_input_string_set_input(_component: &mut Component, _word: &str) {
    panic!("not implemented")
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
