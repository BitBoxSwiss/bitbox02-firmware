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

use alloc::sync::Arc;
use core::task::{Poll, Waker};
use core::time::Duration;

#[cfg(not(any(
    feature = "testing",
    feature = "c-unit-testing",
    feature = "simulator-graphical"
)))]
pub async fn delay_for(duration: Duration) {
    use core::cell::RefCell;
    use core::ffi::c_void;
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

#[cfg(any(
    feature = "testing",
    feature = "c-unit-testing",
    feature = "simulator-graphical"
))]
pub async fn delay_for(duration: Duration) {
    use std::sync::Mutex;
    // Shared between the async context and the c callback
    struct SharedState {
        waker: Option<Waker>,
        result: Option<()>,
        handle: Option<std::thread::JoinHandle<()>>,
    }

    if duration == Duration::ZERO {
        return;
    }

    let shared_state = Arc::new(Mutex::new(SharedState {
        waker: None,
        result: None,
        handle: None,
    }));

    let handle = std::thread::spawn({
        let shared_state = Arc::clone(&shared_state);
        move || {
            std::thread::sleep(duration);
            let mut shared_state = shared_state.lock().unwrap();
            shared_state.result = Some(());
            if let Some(waker) = shared_state.waker.as_ref() {
                waker.wake_by_ref()
            }
        }
    });

    shared_state.lock().unwrap().handle = Some(handle);

    core::future::poll_fn({
        let shared_state = Arc::clone(&shared_state);
        move |cx| {
            let mut shared_state = shared_state.lock().unwrap();

            if let Some(result) = shared_state.result {
                if let Some(handle) = shared_state.handle.take() {
                    handle.join().unwrap();
                }
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
