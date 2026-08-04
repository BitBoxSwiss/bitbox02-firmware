use core::sync::atomic::Ordering;
use core::{cmp, ptr};

#[cfg(not(feature = "portable-atomic"))]
use core::sync::atomic::AtomicUsize;
#[cfg(feature = "portable-atomic")]
use portable_atomic::AtomicUsize;

/// Semi-abstract characterization of the deferred loggers that the `delog!` macro produces.
///
/// # Safety
/// This trait is markes "unsafe" to signal that users should never (need to) "write their own",
/// but always go through the `delog!` macro.
///
/// The user has access to the global logger via `delog::logger()`, but only as TryLog/Log
/// implementation, not with this direct access to implementation details.
pub unsafe trait Delogger: log::Log + crate::TryLog + State<&'static AtomicUsize> {
    /// the underlying buffer
    fn buffer(&self) -> &'static mut [u8];
    /// How many characters were claimed so far.
    fn claimed(&self) -> &'static AtomicUsize;
    /// Call the flusher.
    fn flush(&self, logs: &str);
    /// Actually render the arguments (via internal static buffer).
    fn render(&self, record: &log::Record) -> &'static [u8];

    /// Capacity of circular buffer.
    fn capacity(&self) -> usize {
        self.buffer().len()
    }
}

/// Trait for either state or statistics of loggers.
pub trait State<T> {
    /// How often was one of the logging macros called.
    fn attempts(&self) -> T;
    /// How often was one of the logging macros called without early exit (e.g., buffer not full)
    fn successes(&self) -> T;
    /// How often was the flusher called.
    fn flushes(&self) -> T;
    /// How many bytes were flushed so far.
    fn read(&self) -> T;
    /// How many bytes were logged so far.
    fn written(&self) -> T;
}

#[derive(Clone, Copy, Debug)]
/// Statistics on logger usage.
pub struct Statistics {
    /// How often was one of the logging macros called.
    pub attempts: usize,
    /// How often was one of the logging macros called without early exit (e.g., buffer not full)
    pub successes: usize,
    /// How often was the flusher called.
    pub flushes: usize,
    /// How many bytes were flushed so far.
    pub read: usize,
    /// How many bytes were logged so far.
    pub written: usize,
}

/// Fallible, panic-free version of the `log::Log` trait.
///
/// The intention is actually that implementors of this trait also
/// implement `log::Log` in a panic-free fashion, and simply drop logs
/// that can't be logged. Because, if the user can handle the error, they
/// would be using the fallible macros, and if not, they most likely do **not**
/// want to crash.
pub trait TryLog: log::Log {
    /// Fallible logging call (fails when buffer is full)
    fn try_log(&self, _: &log::Record) -> core::result::Result<(), ()>;
}

/// TryLog with some usage statistics on top.
pub trait TryLogWithStatistics: TryLog + State<usize> {
    /// Read out statistics on logger usage.
    fn statistics(&self) -> Statistics {
        Statistics {
            attempts: self.attempts(),
            successes: self.successes(),
            flushes: self.flushes(),
            read: self.read(),
            written: self.written(),
        }
    }

    // /// How often was one of the logging macros called.
    // fn attempts(&self) -> usize;
    // /// How often was one of the logging macros called without early exit (e.g., buffer not full)
    // fn successes(&self) -> usize;
    // /// How often was the flusher called.
    // fn flushes(&self) -> usize;
    // /// How many bytes were flushed so far.
    // fn read(&self) -> usize;
    // /// How many bytes were logged so far.
    // fn written(&self) -> usize;
}

/// Generate a deferred logger with specified capacity and flushing mechanism.
///
/// Note that only the final "runner" generates, initializes and flushes such a deferred logger.
///
/// Libraries simply make calls to `log::log!`, or its drop-in replacement `delog::log!`,
/// and/or its extension `delog::log_now!`, and/or its alternatives `delog::try_log!` and  `delog::try_log_now`,
/// and/or the local logging variants `local_log!`.
#[cfg(not(any(
    feature = "max_level_off",
    all(not(debug_assertions), feature = "release_max_level_off")
)))]
#[macro_export]
macro_rules! delog {
    ($logger:ident, $capacity:expr, $render_capacity:expr, $flusher:ty) => {
        delog!(
            $logger,
            $capacity,
            $render_capacity,
            $flusher,
            renderer: $crate::render::DefaultRenderer
        );

        impl $logger {
            #[inline]
            pub fn init_default(
                level: $crate::log::LevelFilter,
                flusher: &'static $flusher,
            ) -> Result<(), ()> {
                $logger::init(level, flusher, $crate::render::default())
            }
        }
    };

    ($logger:ident, $capacity:expr, $flusher:ty) => {
        delog!(
            $logger,
            $capacity,
            $capacity,
            $flusher,
            renderer: $crate::render::DefaultRenderer
        );

        impl $logger {
            #[inline]
            pub fn init_default(
                level: $crate::log::LevelFilter,
                flusher: &'static $flusher,
            ) -> Result<(), ()> {
                $logger::init(level, flusher, $crate::render::default())
            }
        }
    };

    ($logger:ident, $capacity:expr, $flusher:ty, renderer: $renderer:ty) => {
        $crate::delog!($logger, $capacity, $capacity, $flusher, renderer: $renderer);
    };

    ($logger:ident, $capacity:expr, $render_capacity:expr, $flusher:ty, renderer: $renderer:ty) => {
        #[derive(Clone, Copy)]
        /// Generated deferred logging implementation.
        pub struct $logger {
            flusher: &'static $flusher,
            renderer: &'static $renderer,
            // immediate_flusher: &'static $flusher,
        }

        // log::Log implementations are required to be Send + Sync
        unsafe impl Send for $logger {}
        unsafe impl Sync for $logger {}

        impl $crate::log::Log for $logger {
            /// log level is set via log::set_max_level, not here, hence always true
            fn enabled(&self, _: &$crate::log::Metadata) -> bool {
                true
            }

            /// reads out logs from circular buffer, and flushes via injected flusher
            fn flush(&self) {
                let mut buf = [0u8; $capacity];

                let logs: &str = unsafe { $crate::dequeue(*self, &mut buf) };

                if logs.len() > 0 {
                    use $crate::Flusher;
                    self.flusher.flush(logs);
                }
            }

            fn log(&self, record: &$crate::log::Record) {
                // use $crate::Delogger;
                unsafe { $crate::enqueue(*self, record) }
            }
        }

        impl $crate::TryLog for $logger {
            fn try_log(&self, record: &$crate::log::Record) -> core::result::Result<(), ()> {
                unsafe { $crate::try_enqueue(*self, record) }
            }
        }

        impl $crate::State<usize> for $logger {
            fn attempts(&self) -> usize {
                <dyn $crate::Delogger>::attempts(self).load(core::sync::atomic::Ordering::SeqCst)
            }
            fn successes(&self) -> usize {
                <dyn $crate::Delogger>::successes(self).load(core::sync::atomic::Ordering::SeqCst)
            }

            fn flushes(&self) -> usize {
                <dyn $crate::Delogger>::flushes(self).load(core::sync::atomic::Ordering::SeqCst)
            }

            fn read(&self) -> usize {
                <dyn $crate::Delogger>::read(self).load(core::sync::atomic::Ordering::SeqCst)
            }
            fn written(&self) -> usize {
                <dyn $crate::Delogger>::written(self).load(core::sync::atomic::Ordering::SeqCst)
            }
        }

        impl $crate::TryLogWithStatistics for $logger {}

        #[allow(missing_docs)]
        impl $logger {
            #[inline]
            pub fn init(
                level: $crate::log::LevelFilter,
                flusher: &'static $flusher,
                renderer: &'static $renderer,
            ) -> Result<(), ()> {
                use core::sync::atomic::{AtomicBool, Ordering};

                static INITIALIZED: AtomicBool = AtomicBool::new(false);
                if INITIALIZED
                    .compare_exchange_weak(false, true, Ordering::AcqRel, Ordering::Acquire)
                    .is_ok()
                {
                    // let logger = Self { flusher, immediate_flusher: flusher };
                    let logger = Self { flusher, renderer };
                    Self::get().replace(logger);
                    $crate::logger().replace(Self::get().as_ref().unwrap());
                    $crate::log::set_logger(Self::get().as_ref().unwrap())
                        .map(|()| $crate::log::set_max_level(level))
                        .map_err(|_| ())
                } else {
                    Err(())
                }
            }

            fn get() -> &'static mut Option<$logger> {
                static mut LOGGER: Option<$logger> = None;
                unsafe { &mut LOGGER }
            }

            pub fn flush() {
                // gracefully degrade if we're not initialized yet
                if let Some(logger) = Self::get() {
                    $crate::log::Log::flush(logger)
                }
            }
        }

        impl $crate::State<&'static core::sync::atomic::AtomicUsize> for $logger {
            fn attempts(&self) -> &'static core::sync::atomic::AtomicUsize {
                use core::sync::atomic::AtomicUsize;
                static LOG_ATTEMPT_COUNT: AtomicUsize = AtomicUsize::new(0);
                &LOG_ATTEMPT_COUNT
            }

            fn successes(&self) -> &'static core::sync::atomic::AtomicUsize {
                use core::sync::atomic::AtomicUsize;
                static LOG_SUCCESS_COUNT: AtomicUsize = AtomicUsize::new(0);
                &LOG_SUCCESS_COUNT
            }

            fn flushes(&self) -> &'static core::sync::atomic::AtomicUsize {
                use core::sync::atomic::AtomicUsize;
                static LOG_FLUSH_COUNT: AtomicUsize = AtomicUsize::new(0);
                &LOG_FLUSH_COUNT
            }

            fn read(&self) -> &'static core::sync::atomic::AtomicUsize {
                use core::sync::atomic::AtomicUsize;
                static READ: AtomicUsize = AtomicUsize::new(0);
                &READ
            }

            fn written(&self) -> &'static core::sync::atomic::AtomicUsize {
                use core::sync::atomic::AtomicUsize;
                static WRITTEN: AtomicUsize = AtomicUsize::new(0);
                &WRITTEN
            }
        }

        unsafe impl $crate::Delogger for $logger {
            fn buffer(&self) -> &'static mut [u8] {
                static mut BUFFER: [u8; $capacity] = [0u8; $capacity];
                unsafe { &mut BUFFER }
            }

            fn flush(&self, logs: &str) {
                use $crate::Flusher;
                self.flusher.flush(logs)
            }

            fn claimed(&self) -> &'static core::sync::atomic::AtomicUsize {
                use core::sync::atomic::AtomicUsize;
                static CLAIMED: AtomicUsize = AtomicUsize::new(0);
                &CLAIMED
            }

            fn render(&self, record: &$crate::Record) -> &'static [u8] {
                static mut LOCAL_BUFFER: [u8; $render_capacity] = [0u8; $render_capacity];

                let local_buffer = unsafe { &mut LOCAL_BUFFER };
                use $crate::Renderer;
                self.renderer.render(local_buffer, record)
            }
        }
    };
}

/// Generate a deferred logger that will completely optimize out.
///
/// Note that the cfg-gate needs to be around the entire macro, as the library
/// calling it will not be the crate that has the `max_level_off` feature.
#[cfg(any(
    feature = "max_level_off",
    all(not(debug_assertions), feature = "release_max_level_off")
))]
#[macro_export]
macro_rules! delog {
    ($logger:ident, $capacity:expr, $flusher:ty) => {
        delog!(
            $logger,
            $capacity,
            $flusher,
            renderer: $crate::render::DefaultRenderer
        );

        impl $logger {
            #[inline]
            pub fn init_default(
                level: $crate::log::LevelFilter,
                flusher: &'static $flusher,
            ) -> Result<(), ()> {
                Ok(())
            }
        }
    };

    ($logger:ident, $capacity:expr, $flusher:ty, renderer: $renderer:ty) => {
        #[derive(Clone, Copy)]
        /// Generated deferred logging implementation.
        pub struct $logger {}

        // log::Log implementations are required to be Send + Sync
        unsafe impl Send for $logger {}
        unsafe impl Sync for $logger {}

        impl $crate::log::Log for $logger {
            /// log level is set via log::set_max_level, not here, hence always true
            fn enabled(&self, _: &$crate::log::Metadata) -> bool {
                true
            }

            /// reads out logs from circular buffer, and flushes via injected flusher
            fn flush(&self) {}

            fn log(&self, _record: &$crate::log::Record) {}
        }

        impl $crate::TryLog for $logger {
            fn try_log(&self, record: &$crate::log::Record) -> core::result::Result<(), ()> {
                Ok(())
            }
        }

        impl $crate::State<usize> for $logger {
            fn attempts(&self) -> usize {
                0
            }
            fn successes(&self) -> usize {
                0
            }
            fn flushes(&self) -> usize {
                0
            }
            fn read(&self) -> usize {
                0
            }
            fn written(&self) -> usize {
                0
            }
        }

        impl $crate::TryLogWithStatistics for $logger {}

        #[allow(missing_docs)]
        impl $logger {
            #[inline]
            pub fn init(
                level: $crate::log::LevelFilter,
                flusher: &'static $flusher,
                renderer: &'static $renderer,
            ) -> Result<(), ()> {
                Ok(())
            }

            fn get() -> &'static mut Option<$logger> {
                static mut LOGGER: Option<$logger> = None;
                unsafe { &mut LOGGER }
            }

            pub fn flush() {}
        }

        impl $crate::State<&'static core::sync::atomic::AtomicUsize> for $logger {
            fn attempts(&self) -> &'static core::sync::atomic::AtomicUsize {
                use core::sync::atomic::AtomicUsize;
                static LOG_ATTEMPT_COUNT: AtomicUsize = AtomicUsize::new(0);
                &LOG_ATTEMPT_COUNT
            }

            fn successes(&self) -> &'static core::sync::atomic::AtomicUsize {
                use core::sync::atomic::AtomicUsize;
                static LOG_SUCCESS_COUNT: AtomicUsize = AtomicUsize::new(0);
                &LOG_SUCCESS_COUNT
            }

            fn flushes(&self) -> &'static core::sync::atomic::AtomicUsize {
                use core::sync::atomic::AtomicUsize;
                static LOG_FLUSH_COUNT: AtomicUsize = AtomicUsize::new(0);
                &LOG_FLUSH_COUNT
            }

            fn read(&self) -> &'static core::sync::atomic::AtomicUsize {
                use core::sync::atomic::AtomicUsize;
                static READ: AtomicUsize = AtomicUsize::new(0);
                &READ
            }

            fn written(&self) -> &'static core::sync::atomic::AtomicUsize {
                use core::sync::atomic::AtomicUsize;
                static WRITTEN: AtomicUsize = AtomicUsize::new(0);
                &WRITTEN
            }
        }

        unsafe impl $crate::Delogger for $logger {
            fn buffer(&self) -> &'static mut [u8] {
                unsafe { &mut [] }
            }

            fn flush(&self, logs: &str) {}

            fn claimed(&self) -> &'static core::sync::atomic::AtomicUsize {
                use core::sync::atomic::AtomicUsize;
                static CLAIMED: AtomicUsize = AtomicUsize::new(0);
                &CLAIMED
            }

            fn render(&self, record: &$crate::Record) -> &'static [u8] {
                &[]
            }
        }
    };
}

/// The core "write to circular buffer" method. Marked unsafe to discourage use!
///
/// # Safety
/// Unfortunately exposed for all to see, as the `delog!` macro needs access to it to
/// implement the logger at call site. Hence marked as unsafe.
pub unsafe fn enqueue(delogger: impl Delogger, record: &log::Record) {
    crate::logger::try_enqueue(delogger, record).ok();
}

/// The fallible "write to circular buffer" method. Marked unsafe to discourage use!
///
/// # Safety
/// Unfortunately exposed for all to see, as the `delog!` macro needs access to it to
/// implement the logger at call site. Hence marked as unsafe.
///
/// This implementation needs some HEAVY testing. It is unsound on PC, where the OS
/// can schedule threads in any manner, but assumed to be sound in ARM Cortex-M NVIC
/// situations, where interrupts are "nested", in the sense that one may be interrupted,
/// then the interrupter can, ..., then the interrupter hands back control, ..., and finally
/// the original caller of this function regains control.
///
/// In this situation, we keep track of three counters `(read, written, claimed)`, with
/// invariants `read <= written <= claimed`. Each writer pessimistically gauges sufficient
/// capacity for its log by checking `claimed + size <= read + capacity`, accounting for the
/// wraparound. If so, the writer **atomically advances the claim counter**, and starts copying
/// its data in this newly claimed space. At the end, it is the duty of the "first" caller
/// to advance the `written` counter to the correct state.
#[allow(unused_unsafe, unused_variables)]
pub unsafe fn try_enqueue(
    delogger: impl Delogger,
    record: &log::Record,
) -> core::result::Result<(), ()> {
    #[cfg(any(
        feature = "max_level_off",
        all(not(debug_assertions), feature = "release_max_level_off")
    ))]
    {
        return Ok(());
    }
    #[cfg(not(any(
        feature = "max_level_off",
        all(not(debug_assertions), feature = "release_max_level_off")
    )))]
    {
        if record.level() > crate::log::max_level() {
            return Ok(());
        }

        // keep track of how man logs were attempted
        delogger.attempts().fetch_add(1, Ordering::SeqCst);

        if record.target() == "!" {
            // todo: possibly use separate immediate_flusher
            let input = delogger.render(record);
            let input = unsafe { core::str::from_utf8_unchecked(input) };
            Delogger::flush(&delogger, input);
            delogger.successes().fetch_add(1, Ordering::SeqCst);
            return Ok(());
        }

        let capacity = delogger.capacity();
        let log = delogger.render(record);
        let size = log.len();

        let previously_claimed = loop {
            let read = delogger.read().load(Ordering::SeqCst);
            let claimed = delogger.claimed().load(Ordering::SeqCst);

            // figure out the corner cases for "wrap-around" at usize capacity
            if claimed + size > read + capacity {
                // not enough space, currently
                return Err(());
            }

            // try to stake out our claim
            let previous = delogger.claimed().compare_exchange(
                claimed,
                claimed + size,
                Ordering::SeqCst,
                Ordering::SeqCst,
            );

            // we were not interrupted, the region is now ours
            if previous == Ok(claimed) {
                break claimed;
            }
        };

        // find out if we're the "first" and will need to update `written` at the end:
        let written = delogger.written().load(Ordering::SeqCst);
        let first: bool = written == previously_claimed;

        // now copy our data - we can be interrupted here at anytime
        let destination = previously_claimed % capacity;
        let buffer = delogger.buffer();
        if destination + size < capacity {
            // can do a single copy
            unsafe {
                ptr::copy_nonoverlapping(log.as_ptr(), buffer.as_mut_ptr().add(destination), size)
            };
        } else {
            // need to split
            let split = capacity - destination;
            unsafe {
                ptr::copy_nonoverlapping(log.as_ptr(), buffer.as_mut_ptr().add(destination), split);
                ptr::copy_nonoverlapping(
                    log.as_ptr().add(split),
                    buffer.as_mut_ptr(),
                    size - split,
                );
            }
        }

        if first {
            // update `written` to current `claimed` (which may be beyond our own claim)
            loop {
                let claimed = delogger.claimed().load(Ordering::SeqCst);
                delogger.written().store(claimed, Ordering::SeqCst);
                if claimed == delogger.claimed().load(Ordering::SeqCst) {
                    break;
                }
            }
        }

        delogger.successes().fetch_add(1, Ordering::SeqCst);
        Ok(())
    }
}

/// The core "read from circular buffer" method. Marked unsafe to discourage use!
///
/// # Safety
/// Unfortunately exposed for all to see, as the `delog!` macro needs access to it to
/// implement the logger at call site. Hence marked as unsafe.
#[allow(unused_unsafe)]
pub unsafe fn dequeue(delogger: impl Delogger, buf: &mut [u8]) -> &str {
    delogger.flushes().fetch_add(1, Ordering::SeqCst);
    // we control the inputs, so we know this is a valid string
    unsafe { core::str::from_utf8_unchecked(drain_as_bytes(delogger, buf)) }
}

/// Copy out the contents of the `Logger` ring buffer into the given buffer,
/// updating `read` to make space for new log data
fn drain_as_bytes(delogger: impl Delogger, buf: &mut [u8]) -> &[u8] {
    unsafe {
        let read = delogger.read().load(Ordering::SeqCst);
        let written = delogger.written().load(Ordering::SeqCst);
        let p = delogger.buffer().as_ptr();

        // early exit to hint the compiler that `n` is not `0`
        let capacity = delogger.buffer().len();
        if capacity == 0 {
            return &[];
        }

        if written > read {
            // number of bytes to copy
            let available = cmp::min(buf.len(), written.wrapping_sub(read));

            let r = read % capacity;

            // NOTE `ptr::copy_nonoverlapping` instead of `copy_from_slice` to avoid panics
            if r + available > capacity {
                // two memcpy-s
                let mid = capacity - r;
                // buf[..mid].copy_from_slice(&buffer[r..]);
                ptr::copy_nonoverlapping(p.add(r), buf.as_mut_ptr(), mid);
                // buf[mid..mid + c].copy_from_slice(&buffer[..available - mid]);
                ptr::copy_nonoverlapping(p, buf.as_mut_ptr().add(mid), available - mid);
            } else {
                // single memcpy
                // buf[..c].copy_from_slice(&buffer[r..r + c]);
                ptr::copy_nonoverlapping(p.add(r), buf.as_mut_ptr(), available);
            }

            delogger
                .read()
                .store(read.wrapping_add(available), Ordering::SeqCst);

            // &buf[..c]
            buf.get_unchecked(..available)
        } else {
            &[]
        }
    }
}
