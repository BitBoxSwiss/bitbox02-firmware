// SPDX-License-Identifier: Apache-2.0

/// calls the C function in screen_saver.c to enable the screen saver
pub fn screen_saver_enable() {
    unsafe {
        bitbox02_sys::screen_saver_enable();
    }
}

// calls the C function in screen_saver.c to disable the screen saver
pub fn screen_saver_disable() {
    unsafe {
        bitbox02_sys::screen_saver_disable();
    }
}

pub struct ScreensaverInhibitor {
    __private: (), // prevents direct init
}

impl ScreensaverInhibitor {
    pub fn new() -> Self {
        screen_saver_disable();
        Self { __private: () }
    }
}

impl Drop for ScreensaverInhibitor {
    fn drop(&mut self) {
        screen_saver_enable();
    }
}
