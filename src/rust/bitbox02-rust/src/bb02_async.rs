// Copyright 2020 Shift Cryptosecurity AG
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

use alloc::boxed::Box;
use core::cell::RefCell;
use core::pin::Pin;
use core::task::{Context, Poll};

/// Task is the top-level future which can be polled by an executor.
/// Note that other futures awaited inside do not have to be pinned.
/// The 'a lifetime allows to spin a boxed/pinned future that is not
/// 'static, or a future with non-'static input param references.
pub type Task<'a, O> = Pin<Box<dyn core::future::Future<Output = O> + 'a>>;

/// A primitive poll invocation for a task, with no waking functionality.
pub fn spin<O>(task: &mut Task<O>) -> Poll<O> {
    // TODO: statically allocate the context.
    let waker = crate::waker_fn::waker_fn(|| {});
    let context = &mut Context::from_waker(&waker);
    task.as_mut().poll(context)
}

/// Implements the Option future, see `option()`.
pub struct AsyncOption<'a, O>(&'a RefCell<Option<O>>);

impl<O> core::future::Future for AsyncOption<'_, O> {
    type Output = O;
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.0.borrow_mut().take() {
            None => Poll::Pending,
            Some(output) => Poll::Ready(output),
        }
    }
}

/// Disables the screensaver while waiting for an option to contain a value. Afterwards, it returns that value
pub async fn option_no_screensaver<O>(opt: &RefCell<Option<O>>) -> O {
    bitbox02::screen_saver::screen_saver_disable();
    let result = option(opt).await;
    bitbox02::screen_saver::screen_saver_enable();
    result
}

/// Waits for an option to contain a value and returns that value, leaving `None` in its place.
/// E.g. `assert_eq!(option(&Some(42)).await, 42)`.
pub fn option<O>(option: &RefCell<Option<O>>) -> AsyncOption<O> {
    AsyncOption(option)
}

/// Polls a future until the result is available.
pub fn block_on<O>(task: impl core::future::Future<Output = O>) -> O {
    let mut task: crate::bb02_async::Task<O> = alloc::boxed::Box::pin(task);
    loop {
        bitbox02::ui::screen_process();
        if let Poll::Ready(result) = spin(&mut task) {
            return result;
        }
    }
}
