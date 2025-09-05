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

//
// C interface
//

#[unsafe(no_mangle)]
pub extern "C" fn rust_rtt_init() {
    rtt_init()
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_rtt_flush() {
    rtt_flush();
}

/// # Safety
///
/// The pointer `ptr` must point to a null terminated string
#[unsafe(no_mangle)]
#[cfg_attr(not(all(feature = "rtt", target_os = "none")), allow(unused))]
pub unsafe extern "C" fn rust_log(ptr: *const core::ffi::c_char) {
    #[cfg(all(feature = "rtt", target_os = "none"))]
    {
        if ptr.is_null() {
            panic!("`ptr` must be a valid pointer");
        }
        let s = unsafe { core::ffi::CStr::from_ptr(ptr as _) };
        let s = unsafe { core::str::from_utf8_unchecked(s.to_bytes()) };
        rtt_target::rprintln!("{}", s);
    }
}

/// # Safety
///
/// The pointer `data` must point to a buffer of length `len`.
#[unsafe(no_mangle)]
#[allow(static_mut_refs)]
#[cfg_attr(not(all(feature = "rtt", target_os = "none")), allow(unused))]
pub unsafe extern "C" fn rust_rtt_ch1_write(data: *const u8, len: usize) {
    #[cfg(all(feature = "rtt", target_os = "none"))]
    {
        let buf = unsafe { core::slice::from_raw_parts(data, len) };
        let channel = unsafe { CH1_UP.as_mut().unwrap() };
        let mut written = 0;
        while written < len {
            written += channel.write(buf);
        }
    }
}

/// # Safety
///
/// The pointer `data` must point to a buffer of length `len`.
#[unsafe(no_mangle)]
#[allow(static_mut_refs)]
#[cfg_attr(not(all(feature = "rtt", target_os = "none")), allow(unused))]
pub unsafe extern "C" fn rust_rtt_ch0_read(data: *mut u8, len: usize) -> usize {
    #[cfg(all(feature = "rtt", target_os = "none"))]
    {
        let buf = unsafe { core::slice::from_raw_parts_mut(data, len) };
        let channel = unsafe { CH0_DOWN.as_mut().unwrap() };
        channel.read(buf)
    }
    #[cfg(not(all(feature = "rtt", target_os = "none")))]
    0
}
