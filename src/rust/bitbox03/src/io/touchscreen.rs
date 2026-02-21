use alloc::collections::VecDeque;
use bitbox_lvgl::{
    LvIndevState, LvIndevType, LvPoint, lv_indev_create, lv_indev_set_read_cb, lv_indev_set_type,
    lv_indev_set_user_data,
};
use core::ptr::NonNull;
use tracing::info;

pub struct TouchScreenEvent {
    pub x: i32,
    pub y: i32,
    pub pressed: bool,
}

pub struct TouchScreen {
    events: &'static mut VecDeque<TouchScreenEvent>,
}

extern "C" fn indev_read_cb(
    indev: *mut bitbox_lvgl::ffi::lv_indev_t,
    data: *mut bitbox_lvgl::ffi::lv_indev_data_t,
) {
    let ud_ptr = unsafe { bitbox_lvgl::ffi::lv_indev_get_user_data(indev) };
    debug_assert!(ud_ptr != core::ptr::null_mut());
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
        let events: &'_ mut VecDeque<TouchScreenEvent> = Box::leak(Box::new(VecDeque::new()));
        let indev = lv_indev_create().expect("create input device");
        lv_indev_set_type(&indev, LvIndevType::LV_INDEV_TYPE_POINTER);
        lv_indev_set_read_cb(&indev, Some(indev_read_cb));

        unsafe { lv_indev_set_user_data(&indev, NonNull::new(events as *mut _ as *mut _)) };
        TouchScreen { events }
    }

    pub fn push(&mut self, event: TouchScreenEvent) {
        self.events.push_back(event)
    }
}
