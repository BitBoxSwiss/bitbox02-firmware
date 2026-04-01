// SPDX-License-Identifier: Apache-2.0

extern crate std;

use std::sync::{Mutex, MutexGuard, Once};

static LVGL_TEST_LOCK: Mutex<()> = Mutex::new(());
static INIT: Once = Once::new();

pub(crate) fn lock_and_init() -> MutexGuard<'static, ()> {
    let lock = LVGL_TEST_LOCK.lock().unwrap();
    INIT.call_once(crate::system::init);
    lock
}
