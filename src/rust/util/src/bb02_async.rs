// SPDX-License-Identifier: Apache-2.0

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
