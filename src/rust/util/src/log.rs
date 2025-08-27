// Re-export rtt_target so that it is available to the macro user
#[cfg(all(feature = "rtt", target_os = "none"))]
pub use ::rtt_target;

/// Macro to log over RTT if `rtt` feature is set, otherwise noop
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => { #[cfg(feature="rtt")] {$crate::log::rtt_target::rprintln!($($arg)*) }};
}

// Make log macro usable in crate
pub use log;

#[cfg(all(feature = "rtt", target_os = "none"))]
pub static mut CH1_UP: Option<rtt_target::UpChannel> = None;
#[cfg(all(feature = "rtt", target_os = "none"))]
pub static mut CH0_DOWN: Option<rtt_target::DownChannel> = None;

pub fn rtt_init() {
    #[cfg(all(feature = "rtt", target_os = "none"))]
    {
        let channels = rtt_target::rtt_init! {
            up: {
                0: {
                    size: 1024,
                    mode: rtt_target::ChannelMode::NoBlockSkip,
                    name: "Terminal",
                    section: ".segger_rtt_buf",
                }
                1: {
                    size:  1024,
                    mode: rtt_target::ChannelMode::NoBlockSkip,
                    name: "ApiResponse",
                    section: ".segger_rtt_buf",
                }
            }
            down: {
                0: {
                    size: 1024,
                    mode: rtt_target::ChannelMode::NoBlockSkip,
                    name: "ApiRequest",
                    section: ".segger_rtt_buf",
                }
            }
            section_cb: ".segger_rtt"
        };

        rtt_target::set_print_channel(channels.up.0);

        unsafe {
            CH0_DOWN = Some(channels.down.0);
            CH1_UP = Some(channels.up.1);
        }
    }
}

/// Wait until all messages have been read by host
pub fn rtt_flush() {
    #[cfg(all(feature = "rtt", target_os = "none"))]
    rtt_target::with_terminal_channel(|c| c.flush());
}
