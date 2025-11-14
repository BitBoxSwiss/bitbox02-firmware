#![no_std]
use core::cell::RefCell;
use core::fmt;
use core::pin::Pin;
use core::sync::atomic::{AtomicBool, Ordering};
use core::task::{Context, Poll, Waker};

use async_task::Runnable;
use concurrent_queue::ConcurrentQueue;
use critical_section::Mutex;
use futures_lite::{future, prelude::*};
use pin_project_lite::pin_project;
use slab::Slab;

mod static_executors;

#[doc(no_inline)]
pub use async_task::{FallibleTask, Task};
pub use static_executors::*;

extern crate alloc;
use alloc::{sync::Arc, vec::Vec};

/// The state of a executor.
struct State {
    /// The global queue.
    queue: ConcurrentQueue<Runnable>,

    /// Local queues created by runners.
    local_queues: Mutex<RefCell<Vec<Arc<ConcurrentQueue<Runnable>>>>>,

    /// Set to `true` when a sleeping ticker is notified or no tickers are sleeping.
    notified: AtomicBool,

    /// A list of sleeping tickers.
    sleepers: Mutex<RefCell<Sleepers>>,

    /// Currently active tasks.
    active: Mutex<RefCell<Slab<Waker>>>,
}

impl State {
    /// Creates state for a new executor.
    const fn new() -> State {
        State {
            queue: ConcurrentQueue::unbounded(),
            local_queues: Mutex::new(RefCell::new(Vec::new())),
            notified: AtomicBool::new(true),
            sleepers: Mutex::new(RefCell::new(Sleepers {
                count: 0,
                wakers: Vec::new(),
                free_ids: Vec::new(),
            })),
            active: Mutex::new(RefCell::new(Slab::new())),
        }
    }

    // fn pin(&self) -> Pin<&Self> {
    //     Pin::new(self)
    // }

    // /// Returns a reference to currently active tasks.
    // fn active(self: Pin<&Self>) -> MutexGuard<'_, Slab<Waker>> {
    //     self.get_ref()
    //         .active
    //         .lock()
    //         .unwrap_or_else(PoisonError::into_inner)
    // }

    /// Notifies a sleeping ticker.
    #[inline]
    fn notify(&self) {
        if self
            .notified
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
        {
            critical_section::with(|cs| {
                let waker = self.sleepers.borrow_ref_mut(cs).notify();
                if let Some(w) = waker {
                    w.wake();
                }
            })
        }
    }

    pub(crate) fn try_tick(&self) -> bool {
        match self.queue.pop() {
            Err(_) => false,
            Ok(runnable) => {
                // Notify another ticker now to pick up where this ticker left off, just in case
                // running the task takes a long time.
                self.notify();

                // Run the task.
                runnable.run();
                true
            }
        }
    }

    pub(crate) async fn tick(&self) {
        let runnable = Ticker::new(self).runnable().await;
        runnable.run();
    }

    pub async fn run<T>(&self, future: impl Future<Output = T>) -> T {
        let mut runner = Runner::new(self);
        let mut rng = fastrand::Rng::with_seed(0x4d595df4d0f33173);

        // A future that runs tasks forever.
        let run_forever = async {
            loop {
                for _ in 0..200 {
                    let runnable = runner.runnable(&mut rng).await;
                    runnable.run();
                }
                future::yield_now().await;
            }
        };

        // Run `future` and `run_forever` concurrently until `future` completes.
        future.or(run_forever).await
    }
}

/// A list of sleeping tickers.
struct Sleepers {
    /// Number of sleeping tickers (both notified and unnotified).
    count: usize,

    /// IDs and wakers of sleeping unnotified tickers.
    ///
    /// A sleeping ticker is notified when its waker is missing from this list.
    wakers: Vec<(usize, Waker)>,

    /// Reclaimed IDs.
    free_ids: Vec<usize>,
}

impl Sleepers {
    /// Inserts a new sleeping ticker.
    fn insert(&mut self, waker: &Waker) -> usize {
        let id = match self.free_ids.pop() {
            Some(id) => id,
            None => self.count + 1,
        };
        self.count += 1;
        self.wakers.push((id, waker.clone()));
        id
    }

    /// Re-inserts a sleeping ticker's waker if it was notified.
    ///
    /// Returns `true` if the ticker was notified.
    fn update(&mut self, id: usize, waker: &Waker) -> bool {
        for item in &mut self.wakers {
            if item.0 == id {
                item.1.clone_from(waker);
                return false;
            }
        }

        self.wakers.push((id, waker.clone()));
        true
    }

    /// Removes a previously inserted sleeping ticker.
    ///
    /// Returns `true` if the ticker was notified.
    fn remove(&mut self, id: usize) -> bool {
        self.count -= 1;
        self.free_ids.push(id);

        for i in (0..self.wakers.len()).rev() {
            if self.wakers[i].0 == id {
                self.wakers.remove(i);
                return false;
            }
        }
        true
    }

    /// Returns `true` if a sleeping ticker is notified or no tickers are sleeping.
    fn is_notified(&self) -> bool {
        self.count == 0 || self.count > self.wakers.len()
    }

    /// Returns notification waker for a sleeping ticker.
    ///
    /// If a ticker was notified already or there are no tickers, `None` will be returned.
    fn notify(&mut self) -> Option<Waker> {
        if self.wakers.len() == self.count {
            self.wakers.pop().map(|item| item.1)
        } else {
            None
        }
    }
}

/// Runs task one by one.
struct Ticker<'a> {
    /// The executor state.
    state: &'a State,

    /// Set to a non-zero sleeper ID when in sleeping state.
    ///
    /// States a ticker can be in:
    /// 1) Woken.
    ///    2a) Sleeping and unnotified.
    ///    2b) Sleeping and notified.
    sleeping: usize,
}

impl Ticker<'_> {
    /// Creates a ticker.
    fn new(state: &State) -> Ticker<'_> {
        Ticker { state, sleeping: 0 }
    }

    /// Moves the ticker into sleeping and unnotified state.
    ///
    /// Returns `false` if the ticker was already sleeping and unnotified.
    fn sleep(&mut self, waker: &Waker) -> bool {
        critical_section::with(|cs| {
            let mut sleepers = self.state.sleepers.borrow_ref_mut(cs);

            match self.sleeping {
                // Move to sleeping state.
                0 => {
                    self.sleeping = sleepers.insert(waker);
                }

                // Already sleeping, check if notified.
                id => {
                    if !sleepers.update(id, waker) {
                        return false;
                    }
                }
            }

            self.state
                .notified
                .store(sleepers.is_notified(), Ordering::Release);

            true
        })
    }

    /// Moves the ticker into woken state.
    fn wake(&mut self) {
        critical_section::with(|cs| {
            if self.sleeping != 0 {
                let mut sleepers = self.state.sleepers.borrow_ref_mut(cs);
                sleepers.remove(self.sleeping);

                self.state
                    .notified
                    .store(sleepers.is_notified(), Ordering::Release);
            }
            self.sleeping = 0;
        })
    }

    /// Waits for the next runnable task to run.
    async fn runnable(&mut self) -> Runnable {
        self.runnable_with(|| self.state.queue.pop().ok()).await
    }

    /// Waits for the next runnable task to run, given a function that searches for a task.
    async fn runnable_with(&mut self, mut search: impl FnMut() -> Option<Runnable>) -> Runnable {
        future::poll_fn(|cx| {
            loop {
                match search() {
                    None => {
                        // Move to sleeping and unnotified state.
                        if !self.sleep(cx.waker()) {
                            // If already sleeping and unnotified, return.
                            return Poll::Pending;
                        }
                    }
                    Some(r) => {
                        // Wake up.
                        self.wake();

                        // Notify another ticker now to pick up where this ticker left off, just in
                        // case running the task takes a long time.
                        self.state.notify();

                        return Poll::Ready(r);
                    }
                }
            }
        })
        .await
    }
}

impl Drop for Ticker<'_> {
    fn drop(&mut self) {
        critical_section::with(|cs| {
            // If this ticker is in sleeping state, it must be removed from the sleepers list.
            if self.sleeping != 0 {
                let mut sleepers = self.state.sleepers.borrow_ref_mut(cs);
                let notified = sleepers.remove(self.sleeping);

                self.state
                    .notified
                    .store(sleepers.is_notified(), Ordering::Release);

                // If this ticker was notified, then notify another ticker.
                if notified {
                    drop(sleepers);
                    self.state.notify();
                }
            }
        })
    }
}

/// A worker in a work-stealing executor.
///
/// This is just a ticker that also has an associated local queue for improved cache locality.
struct Runner<'a> {
    /// The executor state.
    state: &'a State,

    /// Inner ticker.
    ticker: Ticker<'a>,

    /// The local queue.
    local: Arc<ConcurrentQueue<Runnable>>,

    /// Bumped every time a runnable task is found.
    ticks: usize,
}

impl Runner<'_> {
    /// Creates a runner and registers it in the executor state.
    fn new(state: &State) -> Runner<'_> {
        let runner = Runner {
            state,
            ticker: Ticker::new(state),
            local: Arc::new(ConcurrentQueue::bounded(512)),
            ticks: 0,
        };
        critical_section::with(|cs| {
            state
                .local_queues
                .borrow_ref_mut(cs)
                .push(runner.local.clone());
        });
        runner
    }

    /// Waits for the next runnable task to run.
    async fn runnable(&mut self, _rng: &mut fastrand::Rng) -> Runnable {
        let runnable = self
            .ticker
            .runnable_with(|| {
                // Try the local queue.
                if let Ok(r) = self.local.pop() {
                    return Some(r);
                }

                // Try stealing from the global queue.
                if let Ok(r) = self.state.queue.pop() {
                    steal(&self.state.queue, &self.local);
                    return Some(r);
                }

                // TODO(nd): Does work stealing make sense in single core??
                // // Try stealing from other runners.
                // if let Ok(local_queues) = self.state.local_queues.try_read() {
                //     // Pick a random starting point in the iterator list and rotate the list.
                //     let n = local_queues.len();
                //     let start = rng.usize(..n);
                //     let iter = local_queues
                //         .iter()
                //         .chain(local_queues.iter())
                //         .skip(start)
                //         .take(n);

                //     // Remove this runner's local queue.
                //     let iter = iter.filter(|local| !Arc::ptr_eq(local, &self.local));

                //     // Try stealing from each local queue in the list.
                //     for local in iter {
                //         steal(local, &self.local);
                //         if let Ok(r) = self.local.pop() {
                //             return Some(r);
                //         }
                //     }
                // }

                None
            })
            .await;

        // Bump the tick counter.
        self.ticks = self.ticks.wrapping_add(1);

        if self.ticks.is_multiple_of(64) {
            // Steal tasks from the global queue to ensure fair task scheduling.
            steal(&self.state.queue, &self.local);
        }

        runnable
    }
}

impl Drop for Runner<'_> {
    fn drop(&mut self) {
        // Remove the local queue.
        critical_section::with(|cs| {
            self.state
                .local_queues
                .borrow_ref_mut(cs)
                .retain(|local| !Arc::ptr_eq(local, &self.local));
        });

        // Re-schedule remaining tasks in the local queue.
        while let Ok(r) = self.local.pop() {
            r.schedule();
        }
    }
}

/// Steals some items from one queue into another.
fn steal<T>(src: &ConcurrentQueue<T>, dest: &ConcurrentQueue<T>) {
    // Half of `src`'s length rounded up.
    let mut count = src.len().div_ceil(2);

    if count > 0 {
        // Don't steal more than fits into the queue.
        if let Some(cap) = dest.capacity() {
            count = count.min(cap - dest.len());
        }

        // Steal tasks.
        for _ in 0..count {
            if let Ok(t) = src.pop() {
                assert!(dest.push(t).is_ok());
            } else {
                break;
            }
        }
    }
}

/// Debug implementation for `Executor` and `LocalExecutor`.
fn debug_state(state: &State, name: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    /// Debug wrapper for the number of active tasks.
    struct ActiveTasks<'a>(&'a Mutex<RefCell<Slab<Waker>>>);

    impl fmt::Debug for ActiveTasks<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            critical_section::with(|cs| fmt::Debug::fmt(&self.0.borrow_ref(cs).len(), f))
        }
    }

    /// Debug wrapper for the local runners.
    struct LocalRunners<'a>(&'a Mutex<RefCell<Vec<Arc<ConcurrentQueue<Runnable>>>>>);

    impl fmt::Debug for LocalRunners<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            critical_section::with(|cs| fmt::Debug::fmt(&self.0.borrow_ref(cs).len(), f))
        }
    }

    /// Debug wrapper for the sleepers.
    struct SleepCount<'a>(&'a Mutex<RefCell<Sleepers>>);

    impl fmt::Debug for SleepCount<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            critical_section::with(|cs| fmt::Debug::fmt(&self.0.borrow_ref(cs).count, f))
        }
    }

    f.debug_struct(name)
        .field("active", &ActiveTasks(&state.active))
        .field("global_tasks", &state.queue.len())
        .field("local_runners", &LocalRunners(&state.local_queues))
        .field("sleepers", &SleepCount(&state.sleepers))
        .finish()
}

/// Runs a closure when dropped.
struct CallOnDrop<F: FnMut()>(F);

impl<F: FnMut()> Drop for CallOnDrop<F> {
    fn drop(&mut self) {
        (self.0)();
    }
}

pin_project! {
    /// A wrapper around a future, running a closure when dropped.
    struct AsyncCallOnDrop<Fut, Cleanup: FnMut()> {
        #[pin]
        future: Fut,
        cleanup: CallOnDrop<Cleanup>,
    }
}

// impl<Fut, Cleanup: FnMut()> AsyncCallOnDrop<Fut, Cleanup> {
//     fn new(future: Fut, cleanup: Cleanup) -> Self {
//         Self {
//             future,
//             cleanup: CallOnDrop(cleanup),
//         }
//     }
// }

impl<Fut: Future, Cleanup: FnMut()> Future for AsyncCallOnDrop<Fut, Cleanup> {
    type Output = Fut::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.project().future.poll(cx)
    }
}

// /// Polls a future until the result is available.
// pub fn block_on<O>(task: impl core::future::Future<Output = O>) -> O {
//     futures_lite::pin!(task);
//
//     //let runnable = Runnable{};
//     //let waker = runnable.waker()
//     ////let waker = Waker::from(bitbox02_reactor::Waker);
//     //let cx = &mut Context::from_waker(waker)
//     //loop {
//     //    match task.as_mut().poll(cx) {
//     //        Poll::Ready(output) => return output,
//     //        Poll::Pending => (), // TODO: Should "wait for events" and not busy loop
//     //    }
//     //}
// }
