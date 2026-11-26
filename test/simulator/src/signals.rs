// SPDX-License-Identifier: Apache-2.0

use std::io;
use std::sync::atomic::{AtomicBool, Ordering};

static QUITTING: AtomicBool = AtomicBool::new(false);
static HANGUP: AtomicBool = AtomicBool::new(false);

extern "C" fn handle_signal(signal: libc::c_int) {
    match signal {
        libc::SIGHUP => HANGUP.store(true, Ordering::Relaxed),
        libc::SIGINT | libc::SIGTERM => QUITTING.store(true, Ordering::Relaxed),
        _ => {}
    }
}

pub fn install() -> io::Result<()> {
    for signal in [libc::SIGINT, libc::SIGTERM, libc::SIGHUP] {
        let mut action: libc::sigaction = unsafe { std::mem::zeroed() };
        action.sa_sigaction = handle_signal as *const () as usize;
        action.sa_flags = libc::SA_RESTART;
        unsafe { libc::sigemptyset(&mut action.sa_mask) };
        if unsafe { libc::sigaction(signal, &action, std::ptr::null_mut()) } == -1 {
            return Err(io::Error::last_os_error());
        }
    }
    Ok(())
}

pub fn quitting() -> bool {
    QUITTING.load(Ordering::Relaxed)
}

pub fn take_hangup() -> bool {
    HANGUP.swap(false, Ordering::Relaxed)
}
