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
        use core::cell::RefCell;
        use core::ffi::c_void;
        use core::task::{Poll, Waker};

        let mut bitbox02_delay = bitbox02_sys::delay_t { id: 0 };

        // Shared between the async context and the c callback
        struct SharedState {
            waker: Option<Waker>,
            result: Option<()>,
        }
        let shared_state = Box::new(RefCell::new(SharedState {
            waker: None,
            result: None,
        }));
        let shared_state_ptr = shared_state.as_ref() as *const RefCell<SharedState> as *mut c_void;
        unsafe extern "C" fn callback(user_data: *mut c_void) {
            let shared_state = unsafe { &*(user_data as *mut RefCell<SharedState>) };
            let mut shared_state = shared_state.borrow_mut();
            shared_state.result = Some(());
            if let Some(waker) = shared_state.waker.as_ref() {
                waker.wake_by_ref();
            }
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
                let mut shared_state = shared_state.borrow_mut();

                if let Some(result) = shared_state.result {
                    Poll::Ready(result)
                } else {
                    shared_state.waker = Some(cx.waker().clone());
                    Poll::Pending
                }
            }
        })
        .await
    }
}
