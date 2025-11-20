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

//use core::pin::Pin;
//use core::task::{Context, Poll};
use core::time::Duration;

// #[cfg(not(any(
//     feature = "testing",
//     feature = "c-unit-testing",
//     feature = "simulator-graphical"
// )))]
// struct DelayInner {
//     bitbox02_delay: bitbox02_sys::delay_t,
// }
//
// #[cfg(any(
//     feature = "testing",
//     feature = "c-unit-testing",
//     feature = "simulator-graphical"
// ))]
// struct DelayInner {
//     thread_handle: Option<std::thread::JoinHandle<()>>,
//     done: std::sync::Arc<std::sync::atomic::AtomicBool>,
// }
//
// pub struct Delay {
//     inner: DelayInner,
// }
//
// impl Delay {
//     #[cfg(not(any(
//         feature = "testing",
//         feature = "c-unit-testing",
//         feature = "simulator-graphical"
//     )))]
//     pub fn from_ms(ms: u32) -> Delay {
//         let mut delay = Delay {
//             inner: DelayInner {
//                 bitbox02_delay: bitbox02_sys::delay_t { id: usize::MAX },
//             },
//         };
//         delay
//     }
//     #[cfg(any(
//         feature = "testing",
//         feature = "c-unit-testing",
//         feature = "simulator-graphical"
//     ))]
//     pub fn from_ms(ms: u32) -> Delay {
//         let (thread_handle, done) = if ms == 0 {
//             (
//                 None,
//                 std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true)),
//             )
//         } else {
//             let done = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
//             let handle = Some(std::thread::spawn({
//                 let done = std::sync::Arc::clone(&done);
//                 move || {
//                     std::thread::sleep(std::time::Duration::from_millis(ms as u64));
//                     (*done).store(true, std::sync::atomic::Ordering::Relaxed);
//                     // TODO: Waker.wake, once we have an async runtime
//                 }
//             }));
//             (handle, done)
//         };
//         Delay {
//             inner: DelayInner {
//                 thread_handle,
//                 done,
//             },
//         }
//     }
// }
//
// #[cfg(not(any(
//     feature = "testing",
//     feature = "c-unit-testing",
//     feature = "simulator-graphical"
// )))]
// impl Future for Delay {
//     type Output = ();
//
//     fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
//         if unsafe { bitbox02_sys::delay_is_elapsed(&self.inner.bitbox02_delay as *const _) } {
//             Poll::Ready(())
//         } else {
//             Poll::Pending
//         }
//     }
// }
//
// #[cfg(any(
//     feature = "testing",
//     feature = "c-unit-testing",
//     feature = "simulator-graphical"
// ))]
// impl Future for Delay {
//     type Output = ();
//     fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
//         if self.inner.done.load(std::sync::atomic::Ordering::Relaxed) {
//             if let Some(th) = self.inner.thread_handle.take() {
//                 th.join().unwrap();
//             }
//             Poll::Ready(())
//         } else {
//             Poll::Pending
//         }
//     }
// }

// TODO: How to implement drop?
// #[cfg(not(any(
//     feature = "testing",
//     feature = "c-unit-testing",
//     feature = "simulator-graphical"
// )))]
// impl Drop for Delay {
//     fn drop(&mut self) {
//         unsafe { bitbox02_sys::delay_cancel(&self.inner.bitbox02_delay as *const _) }
//     }
// }

use alloc::sync::Arc;
use core::cell::RefCell;
use core::ffi::c_void;
use core::task::{Poll, Waker};

pub async fn delay_for(duration: Duration) {
    // Shared between the async context and the c callback
    struct SharedState {
        waker: Option<Waker>,
        result: Option<()>,
        bitbox02_delay: bitbox02_sys::delay_t,
    }
    let shared_state = Arc::new(RefCell::new(SharedState {
        waker: None,
        result: None,
        bitbox02_delay: bitbox02_sys::delay_t { id: 0 },
    }));
    unsafe extern "C" fn callback(user_data: *mut c_void) {
        let shared_state: Arc<RefCell<SharedState>> = unsafe { Arc::from_raw(user_data as *mut _) };
        let mut shared_state = shared_state.borrow_mut();
        shared_state.result = Some(());
        if let Some(waker) = shared_state.waker.as_ref() {
            waker.wake_by_ref();
        }
    }
    unsafe {
        bitbox02_sys::delay_init_ms(
            &mut shared_state.borrow_mut().bitbox02_delay as *mut _,
            duration.as_millis() as u32,
            Some(callback),
            Arc::into_raw(Arc::clone(&shared_state)) as *mut _,
        )
    }

    core::future::poll_fn({
        let shared_state = Arc::clone(&shared_state);
        move |cx| {
            let mut shared_state = shared_state.borrow_mut();

            if let Some(result) = shared_state.result {
                Poll::Ready(result)
            } else {
                // Store the waker so the callback can wake up this task
                shared_state.waker = Some(cx.waker().clone());
                Poll::Pending
            }
        }
    })
    .await
}
