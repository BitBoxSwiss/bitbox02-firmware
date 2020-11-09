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

//! Stubs for testing.

pub use super::types::{
    ConfirmParams, ContinueCancelCb, Font, MenuParams, SelectWordCb, TrinaryInputStringParams,
};

use crate::safeinputstring::SafeInputString;

use core::marker::PhantomData;

extern crate alloc;

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

pub fn trinary_input_string_create_password<'a, F>(
    _title: &str,
    _special_chars: bool,
    _confirm_callback: F,
    _cancel_callback: Option<ContinueCancelCb<'a>>,
) -> Component<'a>
where
    F: FnMut(SafeInputString) + 'a,
{
    panic!("not implemented")
}

pub fn confirm_create<'a, F>(params: &ConfirmParams, mut result_callback: F) -> Component<'a>
where
    F: FnMut(bool) + 'a,
{
    let data = crate::testing::DATA.0.borrow();
    assert_eq!(data.ui_confirm_create_body.as_ref().unwrap(), params.body);
    result_callback(data.ui_confirm_create_result.unwrap());
    Component {
        is_pushed: false,
        _p: PhantomData,
    }
}

pub fn screen_process() {}

pub fn status_create<'a, F>(_text: &str, _status_success: bool, mut callback: F) -> Component<'a>
where
    F: FnMut() + 'a,
{
    callback();
    Component {
        is_pushed: false,
        _p: PhantomData,
    }
}

pub fn sdcard_create<'a, F>(insert: bool, mut continue_callback: F) -> Component<'a>
where
    F: FnMut() + 'a,
{
    let data = crate::testing::DATA.0.borrow();
    assert_eq!(data.ui_sdcard_create_arg.unwrap(), insert);
    continue_callback();
    Component {
        is_pushed: false,
        _p: PhantomData,
    }
}

pub fn menu_create(_params: MenuParams<'_>) -> Component<'_> {
    panic!("not implemented");
}

pub fn with_lock_animation<F: Fn()>(_f: F) {
    panic!("not implemented")
}

pub fn screen_stack_pop_all() {
    panic!("not implemented")
}
