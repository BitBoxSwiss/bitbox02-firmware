// SPDX-License-Identifier: Apache-2.0

#![no_std]

#[doc(hidden)]
#[cfg(all(feature = "rtt", target_os = "none"))]
pub use log;
#[doc(hidden)]
#[cfg(all(feature = "rtt", target_os = "none"))]
pub use rtt_target;

#[doc(hidden)]
#[cfg(all(feature = "rtt", target_os = "none"))]
pub fn rtt_logger_init() {
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
        reuse_if_initialized: true
    };
    rtt_target::set_print_channel(channels.up.0);
    rtt_target::init_logger_with_level(log::LevelFilter::Trace);
}

#[macro_export]
macro_rules! rtt_logger_init {
    () => {{
        #[cfg(all(feature = "rtt", target_os = "none"))]
        {
            $crate::rtt_logger_init();
        }
    }};
}
