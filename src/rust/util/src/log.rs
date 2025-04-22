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
    {
        let channels = rtt_target::rtt_init! {
            up: {
                0: {
                    size: 1024,
                    mode: rtt_target::ChannelMode::NoBlockSkip,
                    name: "Terminal",
                    section: ".segger_rtt_buf",
                }
            }
            section_cb: ".segger_rtt"
        };

        rtt_target::set_print_channel(channels.up.0);

        log!("RTT Initialized");
    }
}

/// Wait until all messages have been read by host
pub fn rtt_flush() {
    #[cfg(feature = "rtt")]
    rtt_target::with_terminal_channel(|c| c.flush());
}
