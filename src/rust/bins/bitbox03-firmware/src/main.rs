#![no_std]
#![no_main]

extern crate alloc;

use bitbox_hal::{Hal, Ui};
use bitbox_stm32u5::uart::Uart;
use core::fmt::Write;
use core::panic::PanicInfo;
use cortex_m_rt::entry;
use stm32u5::stm32u5a9::interrupt;

use bitbox_lvgl::{self as lvgl, LvDisplay};

#[interrupt]
unsafe fn GPU2D_IRQ() {
    unsafe { bitbox_stm32u5::ffi::GPU2D_IRQHandler() }
}

#[interrupt]
unsafe fn GPU2D_IRQSYS() {
    unsafe { bitbox_stm32u5::ffi::GPU2D_ER_IRQHandler() }
}

#[interrupt]
unsafe fn LCD_TFT() {
    unsafe { bitbox_stm32u5::ffi::LTDC_IRQHandler() }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cortex_m::interrupt::disable();

    let mut uart = Uart::default();
    let _ = writeln!(&mut uart, "{info}");

    cortex_m::asm::bkpt();
    loop {}
}

use embedded_alloc::LlffHeap as Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();

fn ui_init() -> Result<LvDisplay, lvgl::LvDisplayBufferError> {
    lvgl::system::init();
    lvgl::log::register_print_cb(|_level, buf| {
        let mut uart = Uart::default();
        let _ = uart.write_cstr_crlf(buf);
    });
    lvgl::tick::set_cb(Some(bitbox_stm32u5::ffi::HAL_GetTick));

    let ltdc_fbuf_addr_1 = 0xA000_0000usize;
    const LTDC_FBUF_SIZE: usize = 480 * 800 * 4;
    // TODO: Get display size and bytes per pixel from somewhere...
    let fbuf1: &'static mut [u8; LTDC_FBUF_SIZE] =
        unsafe { &mut *(ltdc_fbuf_addr_1 as *mut [u8; LTDC_FBUF_SIZE]) };
    // Position the framebuffers 2 MB apart
    let ltdc_fbuf_addr_2 = 0xA000_0000usize + 2 * 1024 * 1024;
    let fbuf2: &'static mut [u8; LTDC_FBUF_SIZE] =
        unsafe { &mut *(ltdc_fbuf_addr_2 as *mut [u8; LTDC_FBUF_SIZE]) };
    let ltdc_layer = 0;
    LvDisplay::st_ltdc_create_direct(fbuf1, Some(fbuf2), ltdc_layer)
}

#[entry]
unsafe fn main() -> ! {
    // Initialize vendor drivers
    unsafe { bitbox_stm32u5::ffi::platform_init() };

    // Initializing the heap must come super early
    embedded_alloc::init!(HEAP, 128 * 1024);

    // Initalize UI
    let display = ui_init().expect("create LTDC display");
    let mut bitbox = bitbox03::BitBox03::new();
    bitbox.init(display);

    let mut uart = Uart::default();
    let mut counter = 0u32;
    loop {
        if counter % 1000 == 1000 - 1 {
            let _ = writeln!(&mut uart, "hello, world");
            bitbox.ui().switch_to_logo()
        }
        lvgl::timer::handler();
        unsafe {
            bitbox_stm32u5::ffi::HAL_Delay(5);
        }
        counter += 1;
    }
}
