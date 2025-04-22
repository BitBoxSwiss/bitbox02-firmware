use crate::rprintln;
use once_cell::sync::OnceCell;

struct Logger {
    level_filter: log::LevelFilter,
}

impl log::Log for Logger {
    /// Returns if logger is enabled.
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.level_filter
    }

    /// Log the record.
    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            rprintln!(
                "{:<5} [{}] {}",
                record.level(),
                record.target(),
                record.args()
            );
        }
    }

    /// Flush buffered records.
    fn flush(&self) {
        // Nothing to do here
    }
}

static LOGGER: OnceCell<Logger> = OnceCell::new();

/// Init the logger with maximum level (Trace).
///
/// Note: Normally there is no need to call this manually, use `rtt_init_log!` instead.
pub fn init_logger() {
    init_logger_with_level(log::LevelFilter::Trace);
}

/// Init the logger with a specific level.
///
/// Note: Normally there is no need to call this manually, use `rtt_init_log!` instead.
pub fn init_logger_with_level(level_filter: log::LevelFilter) {
    // Logger was already initialized.
    if LOGGER.get().is_some() {
        return;
    }
    let logger = LOGGER.get_or_init(|| Logger { level_filter });

    // Use racy init if the feature is enabled or the target doesn't support atomic pointers.
    #[cfg(any(not(target_has_atomic = "ptr"), feature = "log_racy_init"))]
    unsafe {
        init_racy(logger);
    }

    // Use the default init otherwise.
    #[cfg(all(target_has_atomic = "ptr", not(feature = "log_racy_init")))]
    init_default(logger);
}

#[cfg(all(target_has_atomic = "ptr", not(feature = "log_racy_init")))]
fn init_default(logger: &'static Logger) {
    log::set_logger(logger).ok();
    log::set_max_level(logger.level_filter);
}

// # Safety
//
// This function will call the unsafe functions [log::set_logger_racy] and
// [log::set_max_level_racy] if either the feature `log_racy_init` is enabled or the target doesn't
// support atomic pointers. The [once_cell::OnceCell] should ensure that this is only called
// once.
#[cfg(any(not(target_has_atomic = "ptr"), feature = "log_racy_init"))]
unsafe fn init_racy(logger: &'static Logger) {
    log::set_logger_racy(logger).ok();
    log::set_max_level_racy(logger.level_filter);
}

/// Initializes RTT with a single up channel, sets it as the print channel for the printing macros
/// and sets up a log backend with the given log level.
///
/// The optional arguments specify the level filter (default: `log::LevelFilter::Trace`),
/// the blocking mode (default: `NoBlockSkip`) and size of the buffer in bytes (default: 1024).
///
/// See [`rtt_init`] for more details.
///
/// [`rtt_init`]: crate::rtt_init
#[macro_export]
macro_rules! rtt_init_log {
    ($level:path, $mode:path, $size:expr) => {{
        $crate::rtt_init_print!($mode, $size);
        $crate::init_logger_with_level($level);
    }};

    ($level:path, $mode:path) => {
        $crate::rtt_init_log!($level, $mode, 1024);
    };

    ($level:path) => {{
        use $crate::ChannelMode::NoBlockSkip;
        $crate::rtt_init_log!($level, NoBlockSkip, 1024);
    }};

    () => {{
        use log::LevelFilter::Trace;
        use $crate::ChannelMode::NoBlockSkip;
        $crate::rtt_init_log!(Trace, NoBlockSkip, 1024);
    }};
}
