// Copyright 2025 Shift Crypto AG
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

/// Yields to the executor.
pub fn yield_now() -> impl core::future::Future<Output = ()> {
    let mut yielded = false;
    core::future::poll_fn(move |_cx| {
        if yielded {
            core::task::Poll::Ready(())
        } else {
            yielded = true;
            core::task::Poll::Pending
        }
    })
}

/// Executes both futures in tandem, returning the results of both when both are done.
pub fn join<O1, O2>(
    fut1: impl core::future::Future<Output = O1>,
    fut2: impl core::future::Future<Output = O2>,
) -> impl core::future::Future<Output = (O1, O2)> {
    let mut fut1 = Box::pin(fut1);
    let mut fut2 = Box::pin(fut2);

    let mut result1 = None;
    let mut result2 = None;

    core::future::poll_fn(move |cx| {
        if result1.is_none() {
            if let Poll::Ready(res) = fut1.as_mut().poll(cx) {
                result1 = Some(res);
            }
        }

        if result2.is_none() {
            if let Poll::Ready(res) = fut2.as_mut().poll(cx) {
                result2 = Some(res);
            }
        }

        if result1.is_some() && result2.is_some() {
            Poll::Ready((result1.take().unwrap(), result2.take().unwrap()))
        } else {
            Poll::Pending
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yield_now() {
        async fn f1() -> i32 {
            yield_now().await;
            yield_now().await;
            yield_now().await;
            42
        }

        let mut task: Task<i32> = Box::pin(f1());
        assert_eq!(spin(&mut task), Poll::Pending);
        assert_eq!(spin(&mut task), Poll::Pending);
        assert_eq!(spin(&mut task), Poll::Pending);
        assert_eq!(spin(&mut task), Poll::Ready(42));
    }

    #[test]
    fn test_join() {
        async fn f1() -> i32 {
            -42
        }

        async fn f2() -> Box<u32> {
            yield_now().await;
            yield_now().await;
            yield_now().await;
            Box::new(42)
        }

        assert_eq!(block_on(join(f1(), f2())), (-42, Box::new(42)));
        assert_eq!(block_on(join(f2(), f1())), (Box::new(42), -42));
    }
}
