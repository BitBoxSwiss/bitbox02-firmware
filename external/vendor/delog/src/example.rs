//! An example deferred logger with example flusher.
//!
//! It is included here for documentation purposes only.
//!
//! Do ensure that the `example` feature is not active in production!
//!
//! ```
//! pub struct StderrFlusher {}
//!
//! impl crate::Flusher for StderrFlusher {
//!     fn flush(&self, logs: &str) {
//!         print!("{}", logs);
//!     }
//! }
//!
//! delog!(Delogger, 256, StderrFlusher);
//! static FLUSHER: StderrFlusher = StderrFlusher {};
//! Delogger::init(log::LevelFilter::Info, &STDOUT_FLUSHER).ok();
//!
//! warn!("This is a warning");
//! info_now!("This is IMMEDIATE information");
//! info!("twenty-four bits '{}'", delog::hex_str!(&[0xa1u8, 0xfF, 0x03]));
//!
//! Delogger::flush();
//! ```

// use crate::flushers::StderrFlusher;
use crate::render::DefaultRenderer;

#[derive(Debug, Default)]
/// Flushes logs to stderr.
pub struct StderrFlusher {}

impl crate::Flusher for StderrFlusher {
    fn flush(&self, logs: &str) {
        eprint!("{}", logs);
    }
}

#[derive(Debug, Default)]
/// Flushes logs to stdout.
pub struct StdoutFlusher {}

impl crate::Flusher for StdoutFlusher {
    fn flush(&self, logs: &str) {
        print!("{}", logs);
    }
}

crate::delog!(Delogger, 4096, StderrFlusher, renderer: DefaultRenderer);

// #[macro_export]
// crate::generate_macros!();
