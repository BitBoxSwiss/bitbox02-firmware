// SPDX-License-Identifier: Apache-2.0

use core::pin::Pin;
use core::task::{Context, Poll};
use core::time::Duration;

#[cfg(not(any(
    feature = "testing",
    feature = "c-unit-testing",
    feature = "simulator-graphical"
)))]
struct DelayInner {
    bitbox02_delay: bitbox02_sys::delay_t,
}

#[cfg(any(
    feature = "testing",
    feature = "c-unit-testing",
    feature = "simulator-graphical"
))]
struct DelayInner {
    thread_handle: Option<std::thread::JoinHandle<()>>,
    done: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

pub struct Delay {
    inner: DelayInner,
}

impl Delay {
    #[cfg(not(any(
        feature = "testing",
        feature = "c-unit-testing",
        feature = "simulator-graphical"
    )))]
    pub fn from_ms(ms: u32) -> Delay {
        let mut delay = Delay {
            inner: DelayInner {
                bitbox02_delay: bitbox02_sys::delay_t { id: usize::MAX },
            },
        };
        unsafe { bitbox02_sys::delay_init_ms(&mut delay.inner.bitbox02_delay as *mut _, ms) }
        delay
    }
    #[cfg(any(
        feature = "testing",
        feature = "c-unit-testing",
        feature = "simulator-graphical"
    ))]
    pub fn from_ms(ms: u32) -> Delay {
        let (thread_handle, done) = if ms == 0 {
            (
                None,
                std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true)),
            )
        } else {
            let done = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
            let handle = Some(std::thread::spawn({
                let done = std::sync::Arc::clone(&done);
                move || {
                    std::thread::sleep(std::time::Duration::from_millis(ms as u64));
                    (*done).store(true, std::sync::atomic::Ordering::Relaxed);
                    // TODO: Waker.wake, once we have an async runtime
                }
            }));
            (handle, done)
        };
        Delay {
            inner: DelayInner {
                thread_handle,
                done,
            },
        }
    }
}

#[cfg(not(any(
    feature = "testing",
    feature = "c-unit-testing",
    feature = "simulator-graphical"
)))]
impl Future for Delay {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if unsafe { bitbox02_sys::delay_is_elapsed(&self.inner.bitbox02_delay as *const _) } {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

#[cfg(any(
    feature = "testing",
    feature = "c-unit-testing",
    feature = "simulator-graphical"
))]
impl Future for Delay {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.inner.done.load(std::sync::atomic::Ordering::Relaxed) {
            if let Some(th) = self.inner.thread_handle.take() {
                th.join().unwrap();
            }
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

#[cfg(not(any(
    feature = "testing",
    feature = "c-unit-testing",
    feature = "simulator-graphical"
)))]
impl Drop for Delay {
    fn drop(&mut self) {
        unsafe { bitbox02_sys::delay_cancel(&self.inner.bitbox02_delay as *const _) }
    }
}

pub async fn delay_for(#[allow(unused)] duration: Duration) {
    // Do not delay in (non-graphical) simulator and Rust unit tests.
    #[cfg(not(any(feature = "c-unit-testing", feature = "testing")))]
    Delay::from_ms(duration.as_millis() as u32).await
}
