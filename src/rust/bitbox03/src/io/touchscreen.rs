use alloc::boxed::Box;
use alloc::collections::VecDeque;
use bitbox_lvgl as lvgl;
use core::ptr::NonNull;
use lvgl::{LvIndevState, LvIndevType, LvPoint};
use tracing::info;

pub struct TouchScreenEvent {
    pub x: i32,
    pub y: i32,
    pub pressed: bool,
}

pub struct TouchScreen {
    events: NonNull<VecDeque<TouchScreenEvent>>,
}

extern "C" fn indev_read_cb(
    indev: *mut lvgl::ffi::lv_indev_t,
    data: *mut lvgl::ffi::lv_indev_data_t,
) {
    let ud_ptr = unsafe { lvgl::ffi::lv_indev_get_user_data(indev) };
    debug_assert!(!ud_ptr.is_null());
    let ud = unsafe { &mut *(ud_ptr as *mut VecDeque<TouchScreenEvent>) };
    if let Some(next) = ud.pop_front() {
        info!("popped event");
        let data = unsafe { &mut *data };
        data.point = LvPoint {
            x: next.x,
            y: next.y,
        };
        data.state = if next.pressed {
            LvIndevState::LV_INDEV_STATE_PRESSED
        } else {
            LvIndevState::LV_INDEV_STATE_RELEASED
        };
        data.continue_reading = !ud.is_empty()
    }
}
impl TouchScreen {
    pub fn new() -> TouchScreen {
        let events: &'static mut VecDeque<TouchScreenEvent> = Box::leak(Box::new(VecDeque::new()));
        let events_ptr = NonNull::from(&mut *events);
        let indev = lvgl::LvIndev::new().expect("create input device");
        indev.set_type(LvIndevType::LV_INDEV_TYPE_POINTER);
        indev.set_read_cb(Some(indev_read_cb));

        indev.set_user_data(Some(events));
        TouchScreen { events: events_ptr }
    }

    pub fn push(&mut self, event: TouchScreenEvent) {
        unsafe { self.events.as_mut().push_back(event) }
    }
}
