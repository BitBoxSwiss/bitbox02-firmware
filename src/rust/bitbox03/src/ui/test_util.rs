// SPDX-License-Identifier: Apache-2.0

//! Shared test scaffolding for UI component tests: headless LVGL bring-up (a real 480×800
//! display and tick source, so input processing, layouting and animations run for real) and a
//! scripted touch pointer.

extern crate std;

use std::boxed::Box;
use std::collections::VecDeque;
use std::sync::{LazyLock, Mutex, MutexGuard, Once};
use std::time::{Duration, Instant};
use std::vec;

use core::ptr::NonNull;

use bitbox_lvgl::{
    self as lvgl, LvArea, LvDisplay, LvDisplayRenderMode, LvIndev, LvIndevState, LvIndevType,
    LvPoint, ObjExt, ffi,
};

const WIDTH: i32 = 480;
const HEIGHT: i32 = 800;

extern "C" fn now_ms() -> u32 {
    static START: LazyLock<Instant> = LazyLock::new(Instant::now);
    START.elapsed().as_millis() as u32
}

static LVGL_TEST_LOCK: Mutex<()> = Mutex::new(());
static INIT: Once = Once::new();

/// Serializes tests and lazily brings up LVGL with a headless 480×800 display and a tick
/// source. Every test touching LVGL must hold the returned guard for its whole body.
pub(crate) fn lock_and_init() -> MutexGuard<'static, ()> {
    // A failed test leaves the shared LVGL state usable, so ignore lock poisoning instead of
    // cascading one failure into every later test.
    let guard = LVGL_TEST_LOCK
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    INIT.call_once(|| {
        lvgl::system::init();
        lvgl::tick::set_cb(Some(now_ms));
        let draw_buf: &'static mut [u32] =
            Box::leak(vec![0u32; (WIDTH * HEIGHT) as usize].into_boxed_slice());
        let display = LvDisplay::new(WIDTH, HEIGHT).expect("create display");
        display
            .set_buffers(
                draw_buf,
                None,
                LvDisplayRenderMode::LV_DISPLAY_RENDER_MODE_PARTIAL,
            )
            .expect("set display buffers");
        // Dropping the handle does not delete the LVGL display; it lives for the whole
        // test process.
        display.set_flush_cb(|_display, _area, _px_map| {});
    });
    guard
}

/// Runs the LVGL timer loop (input reading, layout, animation, rendering) for `ms`.
pub(crate) fn pump_for(ms: u64) {
    let deadline = Instant::now() + Duration::from_millis(ms);
    while Instant::now() < deadline {
        lvgl::timer::handler();
        std::thread::sleep(Duration::from_millis(2));
    }
}

/// The absolute screen coordinates of `obj`.
pub(crate) fn coords(obj: &impl ObjExt) -> LvArea {
    let mut area = LvArea {
        x1: 0,
        y1: 0,
        x2: 0,
        y2: 0,
    };
    unsafe { ffi::lv_obj_get_coords(obj.as_ptr(), &mut area) };
    area
}

pub(crate) struct TouchSample {
    x: i32,
    y: i32,
    pressed: bool,
}

/// A scripted LVGL pointer device (same read model as `io::touchscreen::TouchScreen`: the
/// queue front is the current state; entries past the first are drained one per read). Unlike
/// the production type it deletes its input device on drop, so a finished test cannot keep
/// replaying its last sample into later tests.
pub(crate) struct ScriptedTouch {
    pub(crate) indev: LvIndev,
    queue: NonNull<VecDeque<TouchSample>>,
}

extern "C" fn scripted_read_cb(indev: *mut ffi::lv_indev_t, data: *mut ffi::lv_indev_data_t) {
    let queue = unsafe { ffi::lv_indev_get_user_data(indev) };
    debug_assert!(!queue.is_null());
    let queue = unsafe { &mut *(queue as *mut VecDeque<TouchSample>) };
    let data = unsafe { &mut *data };
    if let Some(next) = queue.front() {
        data.point = LvPoint {
            x: next.x,
            y: next.y,
        };
        data.state = if next.pressed {
            LvIndevState::LV_INDEV_STATE_PRESSED
        } else {
            LvIndevState::LV_INDEV_STATE_RELEASED
        };
    }
    if queue.len() > 1 {
        queue.pop_front();
        data.continue_reading = !queue.is_empty();
    }
}

impl ScriptedTouch {
    pub(crate) fn new() -> Self {
        let queue: &'static mut VecDeque<TouchSample> = Box::leak(Box::new(VecDeque::new()));
        let queue_ptr = NonNull::from(&mut *queue);
        let indev = LvIndev::new().expect("create input device");
        indev.set_type(LvIndevType::LV_INDEV_TYPE_POINTER);
        indev.set_read_cb(Some(scripted_read_cb));
        indev.set_user_data(Some(queue));
        Self {
            indev,
            queue: queue_ptr,
        }
    }

    pub(crate) fn push(&mut self, x: i32, y: i32, pressed: bool) {
        unsafe { self.queue.as_mut() }.push_back(TouchSample { x, y, pressed });
    }

    /// Queues a full tap (press, hold in place, release) at (`x`, `y`) and consumes it.
    pub(crate) fn tap(&mut self, x: i32, y: i32) {
        self.push(x, y, true);
        self.push(x, y, true);
        self.push(x, y, false);
        pump_for(120);
    }
}

impl Drop for ScriptedTouch {
    fn drop(&mut self) {
        unsafe {
            ffi::lv_indev_delete(self.indev.as_ptr());
            drop(Box::from_raw(self.queue.as_ptr()));
        }
    }
}
