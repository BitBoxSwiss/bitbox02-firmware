// SPDX-License-Identifier: Apache-2.0

#![no_std]

extern crate alloc;

use alloc::collections::VecDeque;
use async_task::{Builder, Runnable, Task};
use core::cell::RefCell;
use core::sync::atomic::{AtomicUsize, Ordering};
use critical_section::Mutex;

// There are currently three root-task sources: startup, U2F unlock, and U2F confirm. They are
// normally serialized, so only a few slots are needed today. Reserve 16 slots for future workflows
// and more concurrency. On the 32-bit firmware target each slot holds one 4-byte Runnable pointer,
// so the queue buffer uses 64 bytes of heap plus one 8-byte allocator header.
//
// async-task coalesces repeated wakeups, so each active task owns at most one queued Runnable.
// queue.len() only counts scheduled tasks; sleeping and currently running tasks are not in it.
// Bounding all active tasks therefore also bounds the largest possible queue.
const MAX_TASKS: usize = 16;

// The counter is not what prevents allocation: schedule()'s no-growth check does that. This guard
// ensures the preallocated queue cannot be exhausted and reports excess tasks from spawn() rather
// than from a waker that may be running in interrupt context.
struct ActiveTaskGuard(&'static AtomicUsize);

impl ActiveTaskGuard {
    fn new(active_tasks: &'static AtomicUsize) -> Self {
        if active_tasks
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |active_tasks| {
                (active_tasks < MAX_TASKS).then_some(active_tasks + 1)
            })
            .is_err()
        {
            panic!("maximum number of executor tasks exceeded");
        }
        Self(active_tasks)
    }
}

impl Drop for ActiveTaskGuard {
    fn drop(&mut self) {
        let previous = self.0.fetch_sub(1, Ordering::Relaxed);
        debug_assert!(previous > 0);
    }
}

pub struct Executor {
    queue: Mutex<RefCell<VecDeque<Runnable>>>,
    active_tasks: AtomicUsize,
}

impl Executor {
    pub const fn new() -> Executor {
        Executor {
            queue: Mutex::new(RefCell::new(VecDeque::new())),
            active_tasks: AtomicUsize::new(0),
        }
    }

    fn reserve_queue(&self) {
        // VecDeque::with_capacity() is not const, so allocate on the first spawn instead. spawn()
        // already allocates and is never called from interrupt context.
        critical_section::with(|cs| {
            let mut queue = self.queue.borrow(cs).borrow_mut();
            if queue.capacity() < MAX_TASKS {
                let additional = MAX_TASKS - queue.len();
                queue.reserve_exact(additional);
            }
        });
    }

    fn schedule(&self, runnable: Runnable) {
        // Wakers can call this from interrupt context while try_tick() accesses the queue from the
        // main loop, hence the critical section.
        let result = critical_section::with(|cs| {
            let mut queue = self.queue.borrow(cs).borrow_mut();
            // This is the allocator-safety check. push_back() cannot grow the VecDeque when there
            // is unused preallocated capacity. Treat either invariant violation as fatal instead
            // of allocating from interrupt context.
            if queue.len() >= MAX_TASKS || queue.capacity() < MAX_TASKS {
                Err(runnable)
            } else {
                queue.push_back(runnable);
                Ok(())
            }
        });
        if let Err(runnable) = result {
            // A scheduled task owns this reference. Leaking it on this fatal invariant violation
            // avoids running task destruction in interrupt context before the panic handler takes
            // over.
            core::mem::forget(runnable);
            panic!("executor queue full");
        }
    }

    /// Attempts to run a task if at least one is scheduled
    ///
    /// Running a scheduled task means simply polling its future once
    pub fn try_tick(&self) -> bool {
        let runnable = critical_section::with(|cs| self.queue.borrow(cs).borrow_mut().pop_front());
        match runnable {
            None => false,
            Some(runnable) => {
                runnable.run();
                true
            }
        }
    }

    /// Spawns a task onto the executor.
    ///
    /// At most 16 tasks may be active at once.
    ///
    /// This may allocate and must not be called from interrupt context.
    pub fn spawn<T: 'static>(&'static self, future: impl Future<Output = T> + 'static) -> Task<T> {
        self.reserve_queue();
        let active_task_guard = ActiveTaskGuard::new(&self.active_tasks);

        // `schedule` is the function eventually being called when `Waker.wake()` is called. The
        // function schedules the task by placing the tasks Runnable into the executors queue.
        let schedule = move |runnable| self.schedule(runnable);

        // SAFETY
        // 1. `future` doesn't need to be `Send` because the firmware is single threaded
        // 2. `schedule` doesn't need to be `Send` and `Sync` beause the firmware is single threaded
        let (runnable, task) = unsafe {
            Builder::new().spawn_unchecked(
                move |()| async move {
                    // Keep the task counted until its future completes or is cancelled.
                    let _active_task_guard = active_task_guard;
                    future.await
                },
                schedule,
            )
        };

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

#[cfg(test)]
mod tests {
    extern crate std;

    use super::*;
    use core::future::{pending, poll_fn};
    use core::task::Poll;
    use std::boxed::Box;
    use std::panic::{AssertUnwindSafe, catch_unwind};
    use std::vec::Vec;

    fn executor() -> &'static Executor {
        Box::leak(Box::new(Executor::new()))
    }

    #[test]
    fn test_try_tick_reuses_queue_slots() {
        const NUM_WAKES: usize = 100;

        let executor = executor();
        let poll_count: &'static AtomicUsize = Box::leak(Box::new(AtomicUsize::new(0)));
        executor
            .spawn(poll_fn(move |cx| {
                let count = poll_count.load(Ordering::Relaxed);
                if count == NUM_WAKES {
                    Poll::Ready(())
                } else {
                    poll_count.store(count + 1, Ordering::Relaxed);
                    cx.waker().wake_by_ref();
                    Poll::Pending
                }
            }))
            .detach();

        for _ in 0..=NUM_WAKES {
            assert!(executor.try_tick());
        }
        assert!(!executor.try_tick());
        assert_eq!(poll_count.load(Ordering::Relaxed), NUM_WAKES);
    }

    #[test]
    fn test_spawn_task_limit() {
        let executor = executor();
        let tasks = (0..MAX_TASKS)
            .map(|_| executor.spawn(pending::<()>()))
            .collect::<Vec<_>>();

        let result = catch_unwind(AssertUnwindSafe(|| executor.spawn(pending::<()>())));
        assert!(result.is_err());

        drop(tasks);
        for _ in 0..MAX_TASKS {
            assert!(executor.try_tick());
        }
        assert!(!executor.try_tick());
        assert_eq!(executor.active_tasks.load(Ordering::Relaxed), 0);
    }
}
