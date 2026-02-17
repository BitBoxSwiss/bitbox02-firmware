#![no_std]
#![no_main]

extern crate alloc;

use alloc::boxed::Box;
use core::ffi::CStr;
use core::fmt::Write;
use core::panic::PanicInfo;
use cortex_m_rt::entry;

use bitbox_lvgl::{
    LV_PART_MAIN, LvAlign, LvDisplayRenderMode, lv_color_hex, lv_display_create,
    lv_display_set_buffers, lv_display_set_flush_cb, lv_init, lv_label_create, lv_label_set_text,
    lv_obj_align, lv_obj_set_style_bg_color, lv_obj_set_style_text_color, lv_screen_active,
    lv_st_ltdc_create_direct, lv_tick_set_cb,
};

mod uart;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cortex_m::interrupt::disable();

    let mut uart = uart::Uart::default();
    let _ = writeln!(&mut uart, "{info}");

    cortex_m::asm::bkpt();
    loop {}
}

extern "C" fn lv_log_cb(_level: bitbox_lvgl::ffi::lv_log_level_t, buf: *const core::ffi::c_char) {
    let mut uart = uart::Uart::default();

    if !buf.is_null() {
        let _ = uart.write_cstr_crlf(unsafe { CStr::from_ptr(buf) });
    }
}

fn hw_lvgl() {
    /* Get the currently active screen */
    let scr = lv_screen_active().expect("get active screen");

    lv_obj_set_style_bg_color(&scr, lv_color_hex(0x003a57), LV_PART_MAIN as u32);
    lv_obj_set_style_text_color(&scr, lv_color_hex(0xffffff), LV_PART_MAIN as u32);

    /* Create a label */
    let label = lv_label_create(&scr).expect("create label");

    /* Set the label text */
    lv_label_set_text(&label, "BitBox03\nHello, World!\nFrom LVGL").expect("label set text");

    /* Center it on the screen */
    lv_obj_align(&label, LvAlign::LV_ALIGN_CENTER, 0, 0);

    //let button = lv_button_create();
}

extern "C" fn my_flush_cb(
    display: *mut bitbox_lvgl::ffi::lv_display_t,
    area: *const bitbox_lvgl::ffi::lv_area_t,
    px_map: *mut u8,
) {
    debug_assert!(!display.is_null());
    debug_assert!(!area.is_null());
    debug_assert!(!px_map.is_null());
    //let area = unsafe { &*area };
    //info!("flush {:?}", area);
    //let fb_ptr = unsafe { bitbox_lvgl::ffi::lv_display_get_user_data(display) as *mut FrameBuffer };
    //debug_assert!(fb_ptr != core::ptr::null_mut());
    //let fb = unsafe { &mut *fb_ptr };
    //let pxs = px_map as *mut u32;
    //const STRIDE: i32 = 480;
    //let offset = area.y1 * STRIDE;
    //let len = (area.y2 - area.y1 + 1) * STRIDE;
    //if let DynamicImage::ImageRgba8(ref mut image_buf) = fb.buffer {
    //    for (i, pixel) in image_buf
    //        .pixels_mut()
    //        .skip(offset as usize)
    //        .take(len as usize)
    //        .enumerate()
    //    {
    //        *pixel = Rgba(unsafe { (*pxs.add(i)).to_le_bytes() });
    //    }
    //};

    /* IMPORTANT!!!
     * Inform LVGL that flushing is complete so buffer can be modified again. */
    unsafe { bitbox_lvgl::ffi::lv_display_flush_ready(display) };
}

use embedded_alloc::LlffHeap as Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();

fn setup_heap() {
    const HEAP_SIZE: usize = 128 * 1024;
    unsafe { HEAP.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) };
}

use core::ops::{Deref, DerefMut};

// RGBA8888 (32 bits per pixel)
const LVGL_BUFFER_SIZE: usize = 4 * 480 * 10;
#[repr(align(4))]
struct LvglBuffer([u8; LVGL_BUFFER_SIZE]);

impl LvglBuffer {
    fn new() -> LvglBuffer {
        LvglBuffer([0; LVGL_BUFFER_SIZE])
    }
}

impl Deref for LvglBuffer {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LvglBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[entry]
unsafe fn main() -> ! {
    setup_heap();
    unsafe { st_drivers_sys::platform_init() };
    lv_init();
    unsafe { bitbox_lvgl::ffi::lv_log_register_print_cb(Some(lv_log_cb)) };
    lv_tick_set_cb(Some(st_drivers_sys::HAL_GetTick));

    // Make a buffer and give it to lvgl.
    //let buf = Box::leak(Box::new(LvglBuffer::new()));
    //let disp = lv_display_create(480, 800).expect("create display");
    let my_ltdc_framebuffer_address = 0x2000_0000usize;
    let my_ltdc_layer_index = 0u32;
    let disp = unsafe {
        lv_st_ltdc_create_direct(
            my_ltdc_framebuffer_address,
            //Some(&mut buf),
            None,
            my_ltdc_layer_index,
        )
    };
    //unsafe {
    //    lv_display_set_buffers(
    //        &disp,
    //        buf,
    //        None,
    //        LvDisplayRenderMode::LV_DISPLAY_RENDER_MODE_PARTIAL,
    //    )
    //    .expect("display set buffers");
    //};
    //lv_display_set_flush_cb(&disp, Some(my_flush_cb));
    hw_lvgl();

    let mut uart = uart::Uart::default();
    let mut counter = 0u32;
    loop {
        if counter % 200 == 0 {
            let _ = writeln!(&mut uart, "hello, world");
        }
        bitbox_lvgl::lv_timer_handler();
        unsafe {
            st_drivers_sys::HAL_Delay(5);
        }
        counter += 1;
    }
}
