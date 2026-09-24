// SPDX-License-Identifier: Apache-2.0

#![no_std]
#![no_main]

#[cfg(feature = "board-stm32u5a9j-dk")]
use bitbox_board_stm32u5a9j_dk as board;
use bitbox_platform_stm32u5 as _;
use core::panic::PanicInfo;
use cortex_m_rt::entry;
use static_cell::StaticCell;

static USB_OUT_BUFFER: StaticCell<[u8; 128]> = StaticCell::new();
static USB_BUFFERS: StaticCell<bitbox_usb::Buffers<'static>> = StaticCell::new();

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
        if cfg!(feature = "usb-echo") {
            "BitBox03 USB echo"
        } else {
            "BitBox03"
        },
        env!("CARGO_PKG_VERSION"),
    );
    // The firmware is still a stub. Once wallet startup is available, run USB
    // and the HWW transport on bitbox-executor. This bring-up path needs no heap.
    #[cfg(feature = "usb-echo")]
    embassy_futures::block_on(embassy_futures::join::join(usb.run(), async move {
        let mut hid = hid;
        loop {
            hid.ready().await;
            match hid.read().await {
                Ok(report) => {
                    // A disconnect cancels the response; never resend it in a new session.
                    let _ = hid.write(&report).await;
                }
                Err(bitbox_usb::ReadError::Disabled) => {}
                Err(bitbox_usb::ReadError::InvalidReport) => log::warn!("invalid USB HID report"),
            }
        }
    }));
    #[cfg(not(feature = "usb-echo"))]
    {
        let _hid = hid;
        embassy_futures::block_on(usb.run())
    }
    #[cfg(feature = "usb-echo")]
    halt()
}
