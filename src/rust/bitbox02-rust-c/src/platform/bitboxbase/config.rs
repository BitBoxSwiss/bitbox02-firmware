use crate::c_char;
use bitbox02_rust::platform::bitboxbase::config::Config;
use bitbox02_rust::platform::bitboxbase::config::StatusLedMode;
use bitbox02_rust::util::FixedCString;
use core::fmt::Write;

// aaaah, global!
pub static mut CONFIG: Config = Config::new();

#[no_mangle]
pub extern "C" fn bitboxbase_config_led_mode_get() -> StatusLedMode {
    let config = unsafe { &CONFIG };
    config.status_led_mode.clone()
}

#[no_mangle]
pub extern "C" fn bitboxbase_config_ip_get(res: *mut c_char, res_len: usize) {
    // It is not safe to call any functions that also touch CONFIG at the same time
    let config = unsafe { &CONFIG };
    let buf = unsafe { core::slice::from_raw_parts_mut(res, res_len) };
    let mut fcstring = FixedCString::new(buf);

    if let Some(ip) = &config.ip {
        let _ = write!(fcstring, "{}", ip);
    } else {
        let _ = write!(fcstring, "unknown");
    }
}
