use crate::c_char;
use bitbox02_rust::platform::bitboxbase::state::{
    BitBoxBaseBackgroundDescription, BitBoxBaseBackgroundState, State, DESCRIPTIONS,
};
use bitbox02_rust::util::FixedCString;
use core::fmt::Write;

pub static mut STATE: State = State {
    state: BitBoxBaseBackgroundState::BBBWaiting,
    description_code: BitBoxBaseBackgroundDescription::Empty,
};

#[no_mangle]
pub extern "C" fn bitboxbase_state_set_not_alive() {
    let state = unsafe { &mut STATE };
    if state.state != BitBoxBaseBackgroundState::BBBNotAlive {
        (*state).state = BitBoxBaseBackgroundState::BBBNotAlive;
        bitbox02::bitboxbase_screensaver_reset();
    }
}

#[no_mangle]
pub extern "C" fn bitboxbase_state_get() -> BitBoxBaseBackgroundState {
    let state = unsafe { &STATE };
    state.state.clone()
}

#[no_mangle]
pub extern "C" fn bitboxbase_state_get_description(buf: *mut c_char, buf_len: usize) {
    assert!(!buf.is_null());
    let state = unsafe { &STATE };
    let buf = unsafe { core::slice::from_raw_parts_mut(buf, buf_len) };
    let mut buf = FixedCString::new(buf);
    let _ = write!(
        buf,
        "{}",
        DESCRIPTIONS
            .get(state.description_code as usize)
            .unwrap_or(&"<Unknown>")
    );
}
