//! # Compile-time configurable deferred logging (for `printf()`-debugging *aka* tracing)
//!
//! This is an implementation of the `log::Log` trait, suitable for use
//! in both embedded and desktop environments.
//!
//! It has two main goals:
//! - logs are stored in a circular static memory buffer, so that logging is "zero-cost in the inner
//! loop" (apart from the formatting), with deferred actual I/O later via flushing.
//! - compile-time log level settings for applications with multiple library components;
//! inactive log levels of libraries are completely compiled out.
//!
//! Moreover, setting the kill switch feature flag `knock-it-off`, any and all traces of logging
//! are removed from the final binary.
//!
//! ## Usage and defaults
//!
//! > *"Global settings subtractive (default all), local settings additive (default none) but with kill-switch."*
//!
//! From `log`, we inherit:
//! - static global filters, default `LevelFilter::Trace` (i.e., everything), set via `delog` or
//! `log` feature flags (multiple settings result in the most restrictive filter)
//! - dynamic global filter, initialized in the "init"/"init_default" constructors of the
//! macro-generated structs implementing our `Delogger` trait. This can be changed by calls to
//! the global `set_max_level` function in `log`.
//!
//! Libraries that use the logging macros from `log` are governed by the more restrictive of these two settings.
//!
//! On the other hand, a library that uses the `delog::generate_macros!()` macro gains macros `info!`, `warn!`, etc.,
//! which, by default, do nothing, and are completely optimized out.
//!
//! If such a libray itself defines a feature `log-all` that is active, then logging calls with these macros are passed through
//! and goverend by the global filters. Such a library can also define a feature such as
//! `log-info`, which activates *exactly* the info-level logs; it is up to the library to define
//! logic such as "log-info implies log-warn". There is a kill-switch: if the library defines a
//! feature `log-none`, and some intermediate library activates one of the additive `log-*`
//! features, setting `log-none` completely turns off logging for this library.
//!
//! ## Background
//!
//! Compared to existing approaches such as `ufmt`, `cortex-m-funnel` and `defmt`,
//! we pursue different values and requirements, namely:
//!
//! - **compatibility with the standard `core::fmt` traits and the standard `log` library API**.
//!   This means that, while libraries may "upgrade" their logging capabilities by using `delog`
//!   as drop-in replacement for their logging calls (see below), any existing library that already
//!   uses `log` is compatible. This, for our use cases, is a huge win, as opposed to using up "weirdness
//!   budget" and requiring custom tooling for something that is often trivial, throw-away, simple logging.
//! - it follows that one can easily drop a `trace!("{:?}", &suspicious_object)` call at any time for
//!   any object that has a (possibly automatically derived) `Debug` trait implementation – without
//!   passing around structures and keeping on top of lifetimes.
//! - deferred logging: This is a typical "shared memory" logger, calls to `info!` etc.
//!   are not directly sent to their final output, but instead are stored in a circular buffer
//!   that is "drained" by calling `flush` on the logger at a convenient moment, for instance
//!   in an idle loop.
//! - immediate mode logging: Sometimes one wants to bypass the deferred flushing of logs,
//!   this is possible using either the little-known `target` argument to `info!` and friends
//!   with "!" as parameter, or using the additional `immediate_info!` and friends macros.
//! - ability to set log levels *per library, at compile-time*. This can be easily retro-fitted
//!   on existing `log`-based libraries, by adding the requisite features in `Cargo.toml` and
//!   replacing `log` with `delog` (see `gate-tests` for examples of this).
//! - the outcome is that one can leave useful logging calls in the library code, only to activate
//!   them in targeted ways at build time, exactly as needed.
//! - helper macros to easily output binary byte arrays and slices in hexadecimal representations,
//!   which wrap the data in newtypes with custom `fmt::UpperHex` etc. implementations.
//!
//! **Non-goals**:
//!
//! - ultimate speed or code size: Our intention are "normal" logs, not the use of logging/tracing to
//!   for stream binary data to the host. While admittedly the `core::fmt`-ing facilities are not as
//!   efficient as one might hope, in our use cases we have sufficient flash and RAM to use these (and
//!   some hope that, someday, eventually, maybe, the formatting machinery will be revisited and
//!   improved at the root level, namely the language itself.)
//!
//! That said, we believe there is opportunity to extend `delog` in the `defmt` direction by
//! using, e.g., the `fmt::Binary` trait, newtypes and sentinel values to embed raw binary
//! representations of data in time-critical situations without formatting, deferring
//! the extraction and actual formatting to some host-side mechanism.
//!
//! ## Features
//! The `flushers` and `semihosting` features mostly exist to share code within the examples,
//! including the `example` feature. Without them, dependencies are quite minimal, and compilation fast.
//!
//! The `fallible` and `immediate` features (default on) activate the `try_*!` and `*_now!` macros, respectively.
//!
//! ## Warning
//! The current circular buffer implementation (v0.1.0) is definitely unsound on desktop.
//! For embedded use, atomics are required (so no Cortex-M0/M1, and no plans to support non-atomic
//! platforms, which are likely to also be too resource-constrained to support the bloat inherent
//! in `core::fmt`). While we think the implemented circular buffer algorithm works for the "nested interrupt"
//! setup of NVICs, it has not been tested much.
//! The hope is that the worst case scenario is some slightly messed up log outputs.
//!
//! ## Outlook
//! We plan to iterate towards a v0.2.0 soon, making use of a separate "flusher" for the
//! "immediate" logging path. For instance, when logging via serial-over-USB, one might want immediate
//! logs to pend a separate RTIC interrupt handler that blocks until the logs are pushed and read
//! (allowing one to debug the boot process of a firmware), or one might want to just write to RTT
//! (or even semihosting xD) for these, during development.
//!

#![deny(missing_docs)]
#![cfg_attr(not(any(feature = "std", test)), no_std)]

use core::fmt;

pub use log;
pub use log::{Level, LevelFilter, Record};

#[cfg(feature = "example")]
pub mod example;

pub mod hex;

mod logger;
mod macros;
pub mod render;

pub use logger::{
    dequeue, enqueue, try_enqueue, Delogger, State, Statistics, TryLog, TryLogWithStatistics,
};

/// A way to pass on logs, user supplied.
///
/// In embedded, this is intended to pend an interrupt
/// to send the logs off via (USB) serial, semihosting, or similar.
///
/// On PC, typical implemenation will just println! or eprintln!
pub trait Flusher: core::fmt::Debug + Send {
    /// Implementor must handle passed log `&str` in some hopefully useful way.
    fn flush(&self, logs: &str);
}

/// A way to format logs, user supplied.
pub trait Renderer: Send + Sync {
    /// Implementor must render record into `buf`, returning the slice containing the rendered
    /// record.
    fn render<'a>(&self, buf: &'a mut [u8], record: &log::Record) -> &'a [u8];
}

static mut LOGGER: Option<&'static dyn logger::TryLogWithStatistics> = None;

/// Returns a reference to the logger (as `TryLogWithStatistics` implementation)
pub fn logger() -> &'static mut Option<&'static dyn logger::TryLogWithStatistics> {
    // TODO: implement safe alternative
    #[allow(static_mut_refs)]
    unsafe { &mut LOGGER }
}

// WARNING: this is not part of the crate's public API and is subject to change at any time.
// Taken from `log` crate, mutatis mutandis.
// The methods are here and not in `macro` to avoid making the latter public.
#[doc(hidden)]
pub fn __private_api_try_log(
    args: fmt::Arguments,
    level: log::Level,
    &(target, module_path, file, line): &(&str, &'static str, &'static str, u32),
) -> core::result::Result<(), ()> {
    crate::logger().ok_or(())?.try_log(
        &log::Record::builder()
            .args(args)
            .level(level)
            .target(target)
            .module_path_static(Some(module_path))
            .file_static(Some(file))
            .line(Some(line))
            .build(),
    )
}

// WARNING: this is not part of the crate's public API and is subject to change at any time.
#[doc(hidden)]
pub fn __private_api_try_log_lit(
    message: &str,
    level: log::Level,
    &(target, module_path, file, line): &(&str, &'static str, &'static str, u32),
) -> core::result::Result<(), ()> {
    crate::logger().ok_or(())?.try_log(
        &log::Record::builder()
            .args(format_args!("{}", message))
            .level(level)
            .target(target)
            .module_path_static(Some(module_path))
            .file_static(Some(file))
            .line(Some(line))
            .build(),
    )
}
