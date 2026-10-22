// SPDX-License-Identifier: Apache-2.0

#![no_std]
#![no_main]

#[cfg(feature = "board-stm32u5a9j-dk")]
use bitbox_board_stm32u5a9j_dk as board;
use bitbox_platform_stm32u5 as _;
// Install the firmware allocator before any executor tasks or transport buffers are created.
use bitbox03_firmware as _;
use core::panic::PanicInfo;
use cortex_m_rt::entry;
use static_cell::StaticCell;

static USB_OUT_BUFFER: StaticCell<[u8; 128]> = StaticCell::new();
static USB_BUFFERS: StaticCell<bitbox_usb::Buffers<'static>> = StaticCell::new();
static EXECUTOR: bitbox_executor::Executor = bitbox_executor::Executor::new();

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    log::error!("{info}");
    halt()
}

fn halt() -> ! {
    cortex_m::asm::bkpt();
    loop {
        cortex_m::asm::wfe();
    }
}

#[entry]
fn main() -> ! {
    board::init();
    bitbox_debug::rtt_logger_init!();
    let peripherals = bitbox_mcu_stm32u5::pac::Peripherals::take().unwrap();
    log::info!("Starting BitBox03 USB HID");
    let driver = board::usb::new(
        peripherals.OTG_HS,
        peripherals.GPIOA,
        peripherals.GPIOG,
        peripherals.ADC4,
        USB_OUT_BUFFER.init([0; 128]),
    );
    let (mut usb, hid) = bitbox_usb::new(
        driver,
        USB_BUFFERS.init(bitbox_usb::Buffers::default()),
        "BitBox03",
    );
    EXECUTOR.spawn(async move { usb.run().await }).detach();
    EXECUTOR
        .spawn(async move {
            let mut hid = hid;
            let mut transport =
                bitbox02_rust::hww::transport::hww_transport::<bitbox03::BitBox03>();
            let mut last_tick = bitbox_platform_stm32u5::millis();
            let mut elapsed_ms = 0u64;
            hid.run(
                &mut transport,
                move || {
                    let tick = bitbox_platform_stm32u5::millis();
                    elapsed_ms += u64::from(tick.wrapping_sub(last_tick));
                    last_tick = tick;
                    elapsed_ms
                },
                bitbox02_rust::async_usb::spin,
            )
            .await;
        })
        .detach();
    loop {
        EXECUTOR.try_tick();
    }
}
