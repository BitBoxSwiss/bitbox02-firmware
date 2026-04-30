// SPDX-License-Identifier: Apache-2.0

#![no_std]
#![no_main]

extern crate alloc;

use bitbox_board_stm32u5_dk::ffi;
use bitbox_boot_utils::{
    BOOT_ARGS, BootCommand, DFU_METADATA_ADDR, FIRMWARE_ADDR, FIRMWARE_MAX_LEN, FLASH_PAGE_SIZE,
    IMAGE_HEADER_MAGIC_FIRMWARE, IMAGE_SIGNATURE_COUNT, P256_PUBLIC_KEY_LEN, bootload, halt,
    vector_table_from_signed_image,
};
use bitbox_executor::Executor;
use bitbox_mcu_stm32u5 as _;
use bitbox_mcu_stm32u5::pac::interrupt;
use bitbox_platform_stm32u5::flash;
use bitbox_platform_stm32u5::uart::Uart;
use bitbox_platform_stm32u5::usbx::{self, Endpoint, EndpointError, EndpointIn, EndpointOut};
use bitbox_u2fhid::REPORT_SIZE;
use bitbox03_boot1::transport::bootloader_transport_arm;
use core::fmt::Write;
use core::panic::PanicInfo;
use cortex_m_rt::entry;

use embedded_alloc::LlffHeap as Heap;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    cortex_m::asm::bkpt();
    loop {
        cortex_m::asm::wfe();
    }
}

const SIGNING_PUBKEYS: [[u8; P256_PUBLIC_KEY_LEN]; IMAGE_SIGNATURE_COUNT] =
    [[0; P256_PUBLIC_KEY_LEN]; IMAGE_SIGNATURE_COUNT];

static EXECUTOR: Executor = Executor::new();

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[interrupt]
unsafe fn OTG_HS() {
    unsafe {
        bitbox_board_stm32u5_dk::ffi::HAL_PCD_IRQHandler(
            &raw mut bitbox_board_stm32u5_dk::ffi::hpcd_USB_OTG_HS,
        )
    }
}

fn clear_dfu_metadata() -> Result<(), ()> {
    let page = [0xff; FLASH_PAGE_SIZE];
    flash::write_page(DFU_METADATA_ADDR, &page).map_err(|_| ())?;

    let mut erased = [0u8; 16];
    flash::read(DFU_METADATA_ADDR, &mut erased);
    if erased.iter().all(|byte| *byte == 0xff) {
        Ok(())
    } else {
        Err(())
    }
}

fn now_ms() -> u64 {
    unsafe { bitbox_board_stm32u5_dk::ffi::HAL_GetTick() as u64 }
}

async fn usb_bootloader_task() {
    let (mut usb_out, mut usb_in) = usbx::custom_hid();
    let mut report = [0u8; REPORT_SIZE];

    loop {
        usb_out.wait_enabled().await;
        let mut transport = bootloader_transport_arm();

        loop {
            let len = match usb_out.read(&mut report).await {
                Ok(len) => len,
                Err(EndpointError::Disabled) => break,
                Err(EndpointError::BufferOverflow) => {
                    panic!("USBX read exceeded bootloader HID packet size")
                }
            };
            if len != report.len() {
                continue;
            }

            transport.handle_report(&report, now_ms());

            let mut disconnected = false;
            while let Some(response) = transport.pull_report() {
                match usb_in.write(&response).await {
                    Ok(()) => (),
                    Err(EndpointError::Disabled) => {
                        disconnected = true;
                        break;
                    }
                    Err(EndpointError::BufferOverflow) => {
                        panic!("USBX write exceeded bootloader HID packet size")
                    }
                }
            }
            if disconnected {
                break;
            }
        }
    }
}

fn usb_wait() -> ! {
    EXECUTOR.spawn(usb_bootloader_task()).detach();

    loop {
        usbx::process();
        usbx::poll();
        while EXECUTOR.try_tick() {}
        unsafe {
            bitbox_board_stm32u5_dk::ffi::HAL_Delay(5);
        }
    }
}

#[entry]
fn main() -> ! {
    unsafe {
        ffi::board_init();
    }
    unsafe {
        embedded_alloc::init!(HEAP, 128 * 1024);
    }
    let mut uart = Uart::default();
    let _ = writeln!(&mut uart, "[b1] init");

    if clear_dfu_metadata().is_err() {
        halt();
    }

    match BOOT_ARGS.command() {
        Some(BootCommand::BootloaderWait) => {
            BOOT_ARGS.clear();
            let _ = writeln!(&mut uart, "[b1] waiting on usb");
            usb_wait();
        }
        Some(_) => halt(),
        None => {}
    }

    let Ok(vector_table) = vector_table_from_signed_image(
        FIRMWARE_ADDR,
        FIRMWARE_MAX_LEN,
        IMAGE_HEADER_MAGIC_FIRMWARE,
        &SIGNING_PUBKEYS,
    ) else {
        halt();
    };

    bootload(vector_table)
}
