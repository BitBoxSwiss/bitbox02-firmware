#![no_std]
#![no_main]

extern crate alloc;

use bitbox_executor::Executor;
use bitbox_mcu_stm32u5::pac::interrupt;
use bitbox_platform_stm32u5::timer::Tim6;
use bitbox_platform_stm32u5::usbx::{self, Endpoint, EndpointError, EndpointIn, EndpointOut};
use bitbox03::BOOT_ARGS;
use core::panic::PanicInfo;
use core::time::Duration;
use cortex_m_rt::entry;

use bitbox_lvgl::{self as lvgl, LvDisplay};

static EXECUTOR: Executor = Executor::new();
const SLEEP_DURATION_MS: u64 = 5;

#[interrupt]
unsafe fn GPU2D_IRQ() {
    unsafe {
        bitbox_board_stm32u5_dk::ffi::HAL_GPU2D_IRQHandler(
            &raw mut bitbox_board_stm32u5_dk::ffi::hgpu2d,
        )
    }
}

#[interrupt]
unsafe fn GPU2D_IRQSYS() {
    unsafe {
        bitbox_board_stm32u5_dk::ffi::HAL_GPU2D_ER_IRQHandler(
            &raw mut bitbox_board_stm32u5_dk::ffi::hgpu2d,
        )
    }
}

#[interrupt]
unsafe fn LCD_TFT() {
    unsafe {
        bitbox_board_stm32u5_dk::ffi::HAL_LTDC_IRQHandler(
            &raw mut bitbox_board_stm32u5_dk::ffi::hltdc,
        )
    }
}

#[interrupt]
unsafe fn OTG_HS() {
    unsafe {
        bitbox_board_stm32u5_dk::ffi::HAL_PCD_IRQHandler(
            &raw mut bitbox_board_stm32u5_dk::ffi::hpcd_USB_OTG_HS,
        )
    }
}

#[interrupt]
fn TIM6() {
    bitbox_platform_stm32u5::timer::tim6_interrupt_handler();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cortex_m::interrupt::disable();

    log::error!("{info}");

    cortex_m::asm::bkpt();
    loop {}
}

use embedded_alloc::LlffHeap as Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();

fn ui_init() -> Result<LvDisplay, lvgl::LvDisplayBufferError> {
    lvgl::system::init();
    lvgl::log::register_print_cb(|level, buf| {
        let message = buf.to_str().unwrap_or("<invalid utf8>");
        match level as u32 {
            lvgl::ffi::LV_LOG_LEVEL_ERROR => log::error!("LVGL: {message}"),
            lvgl::ffi::LV_LOG_LEVEL_WARN => log::warn!("LVGL: {message}"),
            lvgl::ffi::LV_LOG_LEVEL_INFO => log::info!("LVGL: {message}"),
            lvgl::ffi::LV_LOG_LEVEL_TRACE => log::trace!("LVGL: {message}"),
            lvgl::ffi::LV_LOG_LEVEL_USER => log::debug!("LVGL: {message}"),
            _ => log::debug!("LVGL[{level}]: {message}"),
        }
    });
    lvgl::tick::set_cb(Some(bitbox_board_stm32u5_dk::ffi::HAL_GetTick));

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

fn now_ms() -> u64 {
    unsafe { bitbox_board_stm32u5_dk::ffi::HAL_GetTick() as u64 }
}

async fn usb_hww_task() {
    let (mut usb_out, mut usb_in) = usbx::custom_hid();
    let mut hww = bitbox02_rust::hww::transport::hww_transport::<bitbox03::BitBox03>();
    let mut report = [0u8; 64];

    loop {
        usb_out.wait_enabled().await;
        hww = bitbox02_rust::hww::transport::hww_transport::<bitbox03::BitBox03>();

        loop {
            let len = match usb_out.read(&mut report).await {
                Ok(len) => len,
                Err(EndpointError::Disabled) => break,
                Err(EndpointError::BufferOverflow) => panic!("USBX read exceeded HID packet size"),
            };
            if len != report.len() {
                continue;
            }

            hww.handle_report(&report, now_ms());

            let mut disconnected = false;
            while let Some(response) = hww.pull_report() {
                match usb_in.write(&response).await {
                    Ok(()) => (),
                    Err(EndpointError::Disabled) => {
                        disconnected = true;
                        break;
                    }
                    Err(EndpointError::BufferOverflow) => {
                        panic!("USBX write exceeded HID packet size")
                    }
                }
            }
            if disconnected {
                break;
            }
        }
    }
}

fn main() -> ! {
    log::trace!("main");

    // Initalize UI
    //let display = ui_init().expect("create LTDC display");
    let mut bitbox = bitbox03::BitBox03::new();
    //bitbox.init(display);
    EXECUTOR.spawn(usb_hww_task()).detach();
    let mut sleep_timer = Tim6::new();

    let mut before = unsafe { bitbox_platform_stm32u5::ffi::HAL_GetTick() };
    loop {
        sleep_timer.start_timeout(Duration::from_millis(SLEEP_DURATION_MS));
        usbx::process();
        usbx::poll();
        while EXECUTOR.try_tick() {}
        //lvgl::timer::handler();
        sleep_timer.wait_event_or_timeout();
        let after = unsafe { bitbox_platform_stm32u5::ffi::HAL_GetTick() };
        before = after;
    }
}

#[entry]
unsafe fn entry() -> ! {
    BOOT_ARGS.clear();

    // Initialize vendor drivers
    unsafe { bitbox_board_stm32u5_dk::ffi::board_init() };
    bitbox03_boot_utils::rtt_logger_init!(true);
    log::info!("RTT initialized");

    // Initializing the heap must come super early
    embedded_alloc::init!(HEAP, 128 * 1024);

    main()
}
