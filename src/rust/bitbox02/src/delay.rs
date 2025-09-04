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

use core::pin::Pin;
use core::task::{Context, Poll};

enum DelayState {
    Unstarted,
    Running,
    Done,
}

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
    state: DelayState,
    inner: DelayInner,
    duration: u32,
}

impl Delay {
    #[cfg(not(any(
        feature = "testing",
        feature = "c-unit-testing",
        feature = "simulator-graphical"
    )))]
    pub fn from_ms(ms: u32) -> Result<Delay, ()> {
        let delay = if ms == 0 {
            Delay {
                state: DelayState::Done,
                inner: DelayInner {
                    bitbox02_delay: bitbox02_sys::delay_t { id: usize::MAX },
                },
                duration: ms,
            }
        } else {
            let mut delay = Delay {
                state: DelayState::Unstarted,
                inner: DelayInner {
                    bitbox02_delay: bitbox02_sys::delay_t { id: usize::MAX },
                },
                duration: ms,
            };
            if !unsafe {
                bitbox02_sys::delay_init_ms(
                    &mut delay.inner.bitbox02_delay as *mut _,
                    delay.duration,
                )
            } {
                return Err(());
            }
            delay
        };
        Ok(delay)
    }
    #[cfg(any(
        feature = "testing",
        feature = "c-unit-testing",
        feature = "simulator-graphical"
    ))]
    pub fn from_ms(ms: u32) -> Result<Delay, ()> {
        let delay = if ms == 0 {
            Delay {
                state: DelayState::Done,
                inner: DelayInner {
                    thread_handle: None,
                    done: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
                },
                duration: ms,
            }
        } else {
            Delay {
                state: DelayState::Unstarted,
                inner: DelayInner {
                    thread_handle: None,
                    done: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true)),
                },
                duration: ms,
            }
        };
        Ok(delay)
    }
}

#[cfg(not(any(
    feature = "testing",
    feature = "c-unit-testing",
    feature = "simulator-graphical"
)))]
impl Future for Delay {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = &mut *self;
        match this.state {
            DelayState::Unstarted => {
                unsafe { bitbox02_sys::delay_start(&mut this.inner.bitbox02_delay as *mut _) };
                this.state = DelayState::Running;
                Poll::Pending
            }
            DelayState::Running => {
                if unsafe { bitbox02_sys::delay_poll(&this.inner.bitbox02_delay as *const _) } {
                    this.state = DelayState::Done;
                    return Poll::Ready(());
                }
                Poll::Pending
            }
            DelayState::Done => Poll::Ready(()),
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
        let this = &mut *self;
        match this.state {
            DelayState::Unstarted => {
                let ms = this.duration as u64;
                let done = std::sync::Arc::clone(&this.inner.done);
                this.inner.thread_handle = Some(std::thread::spawn(move || {
                    std::thread::sleep(std::time::Duration::from_millis(ms));
                    (*done).store(true, std::sync::atomic::Ordering::Relaxed);
                }));
                this.state = DelayState::Running;
                Poll::Pending
            }
            DelayState::Running => {
                if this.inner.done.load(std::sync::atomic::Ordering::Relaxed) {
                    this.state = DelayState::Done;
                    if let Some(th) = this.inner.thread_handle.take() {
                        th.join().unwrap();
                    }
                    Poll::Ready(())
                } else {
                    Poll::Pending
                }
            }
            DelayState::Done => Poll::Ready(()),
        }
    }
}
