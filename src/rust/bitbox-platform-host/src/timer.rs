// SPDX-License-Identifier: Apache-2.0

use alloc::sync::Arc;
use core::task::{Poll, Waker};
use core::time::Duration;
use std::sync::Mutex;
use std::thread;

pub struct HostTimer;

impl bitbox_hal::timer::Timer for HostTimer {
    async fn delay_for(duration: Duration) {
        struct SharedState {
            waker: Option<Waker>,
            result: Option<()>,
        }

        if duration == Duration::ZERO {
            return;
        }

        let shared_state = Arc::new(Mutex::new(SharedState {
            waker: None,
            result: None,
        }));

        let mut handle = Some(thread::spawn({
            let shared_state = Arc::clone(&shared_state);
            move || {
                thread::sleep(duration);
                let mut shared_state = shared_state.lock().unwrap();
                shared_state.result = Some(());
                if let Some(waker) = shared_state.waker.as_ref() {
                    waker.wake_by_ref()
                }
            }
        }));

        core::future::poll_fn({
            let shared_state = Arc::clone(&shared_state);
            move |cx| {
                let mut shared_state = shared_state.lock().unwrap();

                if let Some(result) = shared_state.result {
                    if let Some(handle) = handle.take() {
                        handle.join().unwrap();
                    }
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
