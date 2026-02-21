#![no_std]
#![no_main]

extern crate alloc;

use core::ffi::CStr;
use core::fmt::Write;
use core::panic::PanicInfo;
use cortex_m_rt::entry;
use stm32u5::stm32u5a9::interrupt;

use bitbox_lvgl::{lv_init, lv_st_ltdc_create_direct, lv_tick_set_cb};

mod uart;

unsafe extern "C" {
    fn LTDC_IRQHandler();
    fn GPU2D_IRQHandler();
    fn GPU2D_ER_IRQHandler();
}

#[interrupt]
unsafe fn GPU2D_IRQ() {
    unsafe { GPU2D_IRQHandler() }
}

#[interrupt]
unsafe fn GPU2D_IRQSYS() {
    unsafe { GPU2D_ER_IRQHandler() }
}

#[interrupt]
unsafe fn LCD_TFT() {
    unsafe { LTDC_IRQHandler() }
}

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

use embedded_alloc::LlffHeap as Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();

fn setup_heap() {
    const HEAP_SIZE: usize = 128 * 1024;
    unsafe { HEAP.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) };
}

use core::ops::{Deref, DerefMut};

// TODO, we probably want a secondary buffer...
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
    let _disp = unsafe {
        lv_st_ltdc_create_direct(
            my_ltdc_framebuffer_address,
            //Some(&mut buf),
            None,
            my_ltdc_layer_index,
        )
    };
    bitbox03::io::screen::splash();

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
