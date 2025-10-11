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

pub struct Delay {
    state: DelayState,
    bitbox02_delay: bitbox02_sys::delay_t,
    duration: u32,
}

impl Delay {
    pub fn from_ms(ms: u32) -> Result<Delay, ()> {
        let delay = if ms == 0 {
            Delay {
                state: DelayState::Done,
                bitbox02_delay: bitbox02_sys::delay_t { id: usize::MAX },
                duration: ms,
            }
        } else {
            let mut delay = Delay {
                state: DelayState::Unstarted,
                bitbox02_delay: bitbox02_sys::delay_t { id: usize::MAX },
                duration: ms,
            };
            if !unsafe {
                bitbox02_sys::delay_init_ms(&mut delay.bitbox02_delay as *mut _, delay.duration)
            } {
                return Err(());
            }
            delay
        };
        Ok(delay)
    }
}

impl Future for Delay {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = &mut *self;
        match this.state {
            DelayState::Unstarted => {
                unsafe { bitbox02_sys::delay_start(&mut this.bitbox02_delay as *mut _) };
                this.state = DelayState::Running;
                Poll::Pending
            }
            DelayState::Running => {
                if unsafe { bitbox02_sys::delay_poll(&this.bitbox02_delay as *const _) } {
                    this.state = DelayState::Done;
                    return Poll::Ready(());
                }
                Poll::Pending
            }
            DelayState::Done => Poll::Ready(()),
        }
    }
}
