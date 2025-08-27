//! This module contains macros that work exactly like their equivalents without `debug_*`

/* From init.rs */

/// The same as [`rtt_init`](crate::rtt_init) macro but works only in debug
#[macro_export]
macro_rules! debug_rtt_init {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::rtt_init!($($arg)*); })
}

/// The same as [`rtt_init_default`](crate::rtt_init_default) macro but works only in debug
#[macro_export]
macro_rules! debug_rtt_init_default {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::rtt_init_default!($($arg)*); })
}

/* From print.rs */

/// The same as [`rtt_init_print`](crate::rtt_init_print) macro but works only in debug
#[macro_export]
macro_rules! debug_rtt_init_print {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::rtt_init_print!($($arg)*); })
}

/// The same as [`rprintln`](crate::rprintln) macro but works only in debug
#[macro_export]
macro_rules! debug_rprintln {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::rprintln!($($arg)*); })
}

/// The same as [`rprint`](crate::rprint) macro but works only in debug
#[macro_export]
macro_rules! debug_rprint {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::rprint!($($arg)*); })
}
