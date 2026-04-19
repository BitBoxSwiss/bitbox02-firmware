// SPDX-License-Identifier: Apache-2.0

use core::time::Duration;

pub struct BitBox02Timer;

impl bitbox_hal::timer::Timer for BitBox02Timer {
    // Active in C simulator.
    #[cfg(feature = "c-unit-testing")]
    async fn delay_for(_duration: Duration) {
        // Do not delay in (non-graphical) simulator and Rust unit tests.
    }

    // Active in production firmware.
    #[cfg(not(feature = "c-unit-testing"))]
    async fn delay_for(duration: Duration) {
        use alloc::boxed::Box;
        use core::ffi::c_void;
        use core::sync::atomic::{AtomicBool, Ordering};
        use core::task::Poll;
        use futures_core::task::__internal::AtomicWaker;

        let mut bitbox02_delay = bitbox02_sys::delay_t { id: 0 };

        // Shared between the async context and the c callback
        struct SharedState {
            waker: AtomicWaker,
            done: AtomicBool,
        }
        let shared_state = Box::new(SharedState {
            waker: AtomicWaker::new(),
            done: AtomicBool::new(false),
        });

        let shared_state_ptr = shared_state.as_ref() as *const SharedState as *mut c_void;
        unsafe extern "C" fn callback(user_data: *mut c_void) {
            let shared_state = unsafe { &*(user_data as *mut SharedState) };
            shared_state.done.store(true, Ordering::Release);
            shared_state.waker.wake();
        }
        unsafe {
            bitbox02_sys::delay_init_ms(
                &mut bitbox02_delay as *mut _,
                duration.as_millis() as u32,
                Some(callback),
                shared_state_ptr,
            )
        }
        struct DelayGuard<'a>(&'a bitbox02_sys::delay_t);
        impl Drop for DelayGuard<'_> {
            fn drop(&mut self) {
                unsafe {
                    bitbox02_sys::delay_cancel(self.0 as *const _);
                }
            }
        }
        let _delay_guard = DelayGuard(&bitbox02_delay);
        core::future::poll_fn({
            let shared_state = &shared_state;
            move |cx| {
                if shared_state.done.load(Ordering::Acquire) {
                    return Poll::Ready(());
                }

                // Register first, then re-check the completion flag so a callback that fires
                // between the first load and the registration cannot be missed.
                shared_state.waker.register(cx.waker());
                if shared_state.done.load(Ordering::Acquire) {
                    Poll::Ready(())
                } else {
                    Poll::Pending
                }
            }
        })
        .await
    }
}
