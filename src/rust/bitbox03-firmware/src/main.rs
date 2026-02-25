#![no_std]
#![no_main]

extern crate alloc;

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

use embedded_alloc::LlffHeap as Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();

fn ui_init() -> Result<Option<bitbox_lvgl::LvDisplay>, bitbox_lvgl::LvDisplayBufferError> {
    lv_init();
    bitbox_lvgl::lv_log_register_print_cb(|_level, buf| {
        let mut uart = uart::Uart::default();
        let _ = uart.write_cstr_crlf(buf);
    });
    lv_tick_set_cb(Some(st_drivers_sys::HAL_GetTick));

    let ltdc_fbuf_addr_1 = 0xA000_0000usize;
    // TODO: Get display size and bytes per pixel from somewhere...
    let fbuf1 = unsafe { core::slice::from_raw_parts_mut(ltdc_fbuf_addr_1 as _, 480 * 800 * 4) };
    // Position the framebuffers 2 MB apart
    let ltdc_fbuf_addr_2 = 0xA000_0000usize + 2 * 1024 * 1024;
    let fbuf2 = unsafe { core::slice::from_raw_parts_mut(ltdc_fbuf_addr_2 as _, 480 * 800 * 4) };
    let ltdc_layer = 0;
    unsafe { lv_st_ltdc_create_direct(fbuf1, Some(fbuf2), ltdc_layer) }
}

#[entry]
unsafe fn main() -> ! {
    // Initialize vendor drivers
    unsafe { st_drivers_sys::platform_init() };

    // Initializing the heap must come super early
    embedded_alloc::init!(HEAP, 128 * 1024);

    // Initalize UI
    let _disp = ui_init();
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
