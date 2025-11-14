use crate::{State, debug_state};
use async_task::{Builder, Runnable, Task};
use core::{
    fmt,
    future::Future,
    panic::{RefUnwindSafe, UnwindSafe},
};

/// A static-lifetimed async Executor.
///
/// This is primarily intended to be used in [`static`] variables.
///
/// Spawning, running, and finishing tasks are optimized with the assumption that the executor will never be `Drop`'ed.
/// A static executor may require signficantly less overhead in both single-threaded and mulitthreaded use cases.
///
/// As this type does not implement `Drop`, losing the handle to the executor or failing
/// to consistently drive the executor with [`StaticExecutor::tick`] or
/// [`StaticExecutor::run`] will cause the all spawned tasks to permanently leak. Any
/// tasks at the time will not be cancelled.
///
/// [`static`]: https://doc.rust-lang.org/std/keyword.static.html
#[repr(transparent)]
pub struct StaticExecutor {
    state: State,
}

// SAFETY: Executor stores no thread local state that can be accessed via other thread.
unsafe impl Send for StaticExecutor {}
// SAFETY: Executor internally synchronizes all of it's operations internally.
unsafe impl Sync for StaticExecutor {}

impl UnwindSafe for StaticExecutor {}
impl RefUnwindSafe for StaticExecutor {}

impl fmt::Debug for StaticExecutor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        debug_state(&self.state, "StaticExecutor", f)
    }
}

impl StaticExecutor {
    /// Creates a new StaticExecutor.
    ///
    /// # Examples
    ///
    /// ```
    /// use async_executor::StaticExecutor;
    ///
    /// static EXECUTOR: StaticExecutor = StaticExecutor::new();
    /// ```
    pub const fn new() -> Self {
        Self {
            state: State::new(),
        }
    }

    /// Spawns a task onto the executor.
    ///
    /// Note: unlike [`Executor::spawn`], this function requires being called with a `'static`
    /// borrow on the executor.
    ///
    /// # Examples
    ///
    /// ```
    /// use async_executor::StaticExecutor;
    ///
    /// static EXECUTOR: StaticExecutor = StaticExecutor::new();
    ///
    /// let task = EXECUTOR.spawn(async {
    ///     println!("Hello world");
    /// });
    /// ```
    pub fn spawn<T: 'static>(&'static self, future: impl Future<Output = T> + 'static) -> Task<T> {
        let (runnable, task) =
            unsafe { Builder::new().spawn_unchecked(|()| future, self.schedule()) };
        runnable.schedule();
        task
    }

    /// Spawns a non-`'static` task onto the executor.
    ///
    /// ## Safety
    ///
    /// The caller must ensure that the returned task terminates
    /// or is cancelled before the end of 'a.
    pub unsafe fn spawn_scoped<'a, T: 'a>(
        &'static self,
        future: impl Future<Output = T> + 'a,
    ) -> Task<T> {
        // SAFETY:
        //
        // - Executor is single threaded
        // - `future` is not `'static`, but the caller guarantees that the
        //    task, and thus its `Runnable` must not live longer than `'a`.
        // - `self.schedule()` is `Send`, `Sync` and `'static`, as checked below.
        //    Therefore we do not need to worry about what is done with the
        //    `Waker`.
        let (runnable, task) =
            unsafe { Builder::new().spawn_unchecked(|()| future, self.schedule()) };
        runnable.schedule();
        task
    }

    /// Attempts to run a task if at least one is scheduled.
    ///
    /// Running a scheduled task means simply polling its future once.
    ///
    /// # Examples
    ///
    /// ```
    /// use async_executor::StaticExecutor;
    ///
    /// static EXECUTOR: StaticExecutor = StaticExecutor::new();
    ///
    /// assert!(!EXECUTOR.try_tick()); // no tasks to run
    ///
    /// let task = EXECUTOR.spawn(async {
    ///     println!("Hello world");
    /// });
    ///
    /// assert!(EXECUTOR.try_tick()); // a task was found
    /// ```
    pub fn try_tick(&self) -> bool {
        self.state.try_tick()
    }

    /// Runs a single task.
    ///
    /// Running a task means simply polling its future once.
    ///
    /// If no tasks are scheduled when this method is called, it will wait until one is scheduled.
    ///
    /// # Examples
    ///
    /// ```
    /// use async_executor::StaticExecutor;
    /// use futures_lite::future;
    ///
    /// static EXECUTOR: StaticExecutor = StaticExecutor::new();
    ///
    /// let task = EXECUTOR.spawn(async {
    ///     println!("Hello world");
    /// });
    ///
    /// future::block_on(EXECUTOR.tick()); // runs the task
    /// ```
    pub async fn tick(&self) {
        self.state.tick().await;
    }

    /// Runs the executor until the given future completes.
    ///
    /// # Examples
    ///
    /// ```
    /// use async_executor::StaticExecutor;
    /// use futures_lite::future;
    ///
    /// static EXECUTOR: StaticExecutor = StaticExecutor::new();
    ///
    /// let task = EXECUTOR.spawn(async { 1 + 2 });
    /// let res = future::block_on(EXECUTOR.run(async { task.await * 2 }));
    ///
    /// assert_eq!(res, 6);
    /// ```
    pub async fn run<T>(&self, future: impl Future<Output = T>) -> T {
        self.state.run(future).await
    }

    /// Returns a function that schedules a runnable task when it gets woken up.
    fn schedule(&'static self) -> impl Fn(Runnable) + Sync + 'static {
        let state: &'static State = &self.state;
        // TODO: If possible, push into the current local queue and notify the ticker.
        move |runnable| {
            let result = state.queue.push(runnable);
            debug_assert!(result.is_ok()); // Since we use unbounded queue, push will never fail.
            state.notify();
        }
    }
}

impl Default for StaticExecutor {
    fn default() -> Self {
        Self::new()
    }
}
