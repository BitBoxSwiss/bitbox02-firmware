// SPDX-License-Identifier: Apache-2.0

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
