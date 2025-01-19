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

#[cfg(feature = "rtt")]
pub static mut CH1_UP: Option<rtt_target::UpChannel> = None;
#[cfg(feature = "rtt")]
pub static mut CH0_DOWN: Option<rtt_target::DownChannel> = None;

pub fn rtt_init() {
    #[cfg(feature = "rtt")]
    {
        let channels = rtt_target::rtt_init!(
        up: {
            0: {
                size:  1024,
                mode: rtt_target::ChannelMode::NoBlockSkip,
                name: "Terminal"
            }
            1: {
                size:  1024,
                mode: rtt_target::ChannelMode::NoBlockSkip,
                name: "ApiResponse"
            }
        }
        down: {
            0: {
                size: 1024,
                mode: rtt_target::ChannelMode::NoBlockSkip,
                name: "ApiRequest"
            }
        });
        rtt_target::set_print_channel(channels.up.0);
        // TODO: should not print if release build
        log!("RTT Initialized");

        unsafe {
            CH0_DOWN = Some(channels.down.0);
            CH1_UP = Some(channels.up.1);
        }
    }
}
