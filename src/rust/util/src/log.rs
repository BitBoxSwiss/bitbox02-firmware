// Re-export rtt_target so that it is available to the macro user
#[cfg(feature = "rtt")]
pub use ::rtt_target;

/// Macro to log over RTT if `rtt` feature is set, otherwise noop
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => { #[cfg(feature="rtt")] {$crate::log::rtt_target::rprintln!($($arg)*) }};
}

// Make log macro usable in crate
pub use log;

pub fn rtt_init() {
    #[cfg(feature = "rtt")]
    rtt_target::rtt_init_print!();
    log!("RTT Initialized");
}
