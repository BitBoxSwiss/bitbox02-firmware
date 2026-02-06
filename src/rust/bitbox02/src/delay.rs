// SPDX-License-Identifier: Apache-2.0

use core::time::Duration;

// Active in C simulator and Rust unit tests.
#[cfg(any(feature = "c-unit-testing", feature = "testing"))]
pub async fn delay_for(_duration: Duration) {
    // Do not delay in (non-graphical) simulator and Rust unit tests.
}

// Active in production firmware.
#[cfg(not(any(
    feature = "testing",
    feature = "c-unit-testing",
    feature = "simulator-graphical"
)))]
pub async fn delay_for(duration: Duration) {
    use alloc::sync::Arc;
    use core::cell::RefCell;
    use core::ffi::c_void;
    use core::task::{Poll, Waker};

    let mut bitbox02_delay = bitbox02_sys::delay_t { id: 0 };

    // Shared between the async context and the c callback
    struct SharedState {
        waker: Option<Waker>,
        result: Option<()>,
    }
    let shared_state = Arc::new(RefCell::new(SharedState {
        waker: None,
        result: None,
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
            &mut bitbox02_delay as *mut _,
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

// Active in the graphical simulators.
#[cfg(all(feature = "simulator-graphical", not(feature = "testing")))]
pub async fn delay_for(duration: Duration) {
    use alloc::sync::Arc;
    use core::task::{Poll, Waker};
    use std::sync::Mutex;

    // Shared between the async context and the c callback
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

    let mut handle: Option<std::thread::JoinHandle<()>> = Some(std::thread::spawn({
        let shared_state = Arc::clone(&shared_state);
        move || {
            std::thread::sleep(duration);
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
                // Store the waker so the callback can wake up this task
                shared_state.waker = Some(cx.waker().clone());
                Poll::Pending
            }
        }
    })
    .await
}
