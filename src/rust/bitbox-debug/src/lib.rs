// SPDX-License-Identifier: Apache-2.0

#![no_std]

#[doc(hidden)]
#[cfg(all(feature = "rtt", target_os = "none"))]
pub use log;
#[doc(hidden)]
#[cfg(all(feature = "rtt", target_os = "none"))]
pub use rtt_target;

#[cfg(all(feature = "rtt", target_os = "none"))]
pub struct RttChannels {
    pub api_response: rtt_target::UpChannel,
    pub api_request: rtt_target::DownChannel,
}

#[cfg(all(feature = "rtt", target_os = "none"))]
pub fn rtt_channels_init() -> RttChannels {
    let channels = rtt_target::rtt_init! {
        up: {
            0: {
                size: 1024,
                mode: rtt_target::ChannelMode::NoBlockSkip,
                name: "Terminal",
                section: ".segger_rtt_buf",
            }
            1: {
                size: 1024,
                mode: rtt_target::ChannelMode::NoBlockSkip,
                name: "API Response",
                section: ".segger_rtt_buf",
            }
        }
        down: {
            // OpenOCD maps channel ids to TCP ports, so keep channel 0 defined even though the
            // API uses channel 1.
            0: {
                size: 16,
                mode: rtt_target::ChannelMode::NoBlockSkip,
                name: "Terminal",
                section: ".segger_rtt_buf",
            }
            1: {
                size: 1024,
                mode: rtt_target::ChannelMode::NoBlockSkip,
                name: "API Request",
                section: ".segger_rtt_buf",
            }
        }
        section_cb: ".segger_rtt"
        reuse_if_initialized: true
    };
    rtt_target::set_print_channel(channels.up.0);
    rtt_target::init_logger_with_level(log::LevelFilter::Trace);
    RttChannels {
        api_response: channels.up.1,
        api_request: channels.down.1,
    }
}

#[macro_export]
macro_rules! rtt_logger_init {
    () => {{
        #[cfg(all(feature = "rtt", target_os = "none"))]
        {
            let _ = $crate::rtt_channels_init();
        }
    }};
}
