// SPDX-License-Identifier: Apache-2.0

use alloc::boxed::Box;
use core::pin::Pin;

/// Task is the top-level future which can be polled by an executor.
/// Note that other futures awaited inside do not have to be pinned.
/// The 'a lifetime allows to poll a boxed/pinned future that is not
/// 'static, or a future with non-'static input param references.
pub type Task<'a, O> = Pin<Box<dyn core::future::Future<Output = O> + 'a>>;

/// Busy wait until `task` resolves. Ignores Waker.
#[cfg(feature = "testing")]
pub fn block_on<O>(task: impl core::future::Future<Output = O>) -> O {
    let mut task: crate::bb02_async::Task<O> = alloc::boxed::Box::pin(task);
    loop {
        if let Poll::Ready(result) = spin(&mut task) {
            return result;
        }
    }
}
