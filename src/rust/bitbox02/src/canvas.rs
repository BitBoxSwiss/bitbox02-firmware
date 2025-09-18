pub fn init() {
    unsafe {
        bitbox02_sys::canvas_init();
    }
}

pub fn commit() {
    unsafe {
        bitbox02_sys::canvas_commit();
    }
}
