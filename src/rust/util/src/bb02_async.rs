// SPDX-License-Identifier: Apache-2.0

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

/// Waits for an option to contain a value and returns that value, leaving `None` in its place.
/// E.g. `assert_eq!(option(&Some(42)).await, 42)`.
pub fn option<O>(option: &RefCell<Option<O>>) -> AsyncOption<'_, O> {
    AsyncOption(option)
}

/// Polls a future until the result is available.
#[cfg(feature = "testing")]
pub fn block_on<O>(task: impl core::future::Future<Output = O>) -> O {
    let mut task: crate::bb02_async::Task<O> = alloc::boxed::Box::pin(task);
    loop {
        if let Poll::Ready(result) = spin(&mut task) {
            return result;
        }
    }
}
