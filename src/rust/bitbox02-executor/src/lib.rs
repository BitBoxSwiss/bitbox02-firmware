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

#![no_std]
use async_task::{Builder, Runnable, Task};
use concurrent_queue::ConcurrentQueue;

pub struct Executor {
    queue: ConcurrentQueue<Runnable>,
}

impl Executor {
    pub const fn new() -> Executor {
        Executor {
            queue: ConcurrentQueue::unbounded(),
        }
    }

    /// Attempts to run a task if at least one is scheduled
    ///
    /// Running a scheduled task means simply polling its future once
    pub fn try_tick(&self) -> bool {
        match self.queue.pop() {
            Err(_) => false,
            Ok(runnable) => {
                runnable.run();
                true
            }
        }
    }

    /// Spawns a task onto the executor.
    pub fn spawn<T: 'static>(&'static self, future: impl Future<Output = T> + 'static) -> Task<T> {
        // `schedule` is the function eventually being called when `Waker.wake()` is called. The
        // function schedules the task by placing the tasks Runnable into the executors queue.
        let schedule = move |runnable| self.queue.push(runnable).unwrap();

        // SAFETY
        // 1. `future` doesn't need to be `Send` because the firmware is single threaded
        // 2. `schedule` doesn't need to be `Send` and `Sync` beause the firmware is single threaded
        let (runnable, task) = unsafe { Builder::new().spawn_unchecked(|()| future, schedule) };

        // Schedule the task once to get started
        runnable.schedule();
        task
    }
}

impl Default for Executor {
    fn default() -> Executor {
        Executor::new()
    }
}
