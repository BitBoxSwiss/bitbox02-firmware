// SPDX-License-Identifier: Apache-2.0

#![no_std]
#![no_main]

#[cfg(feature = "board-stm32u5a9j-dk")]
use bitbox_board_stm32u5a9j_dk as board;
use bitbox_mcu_stm32u5 as _;
use bitbox_platform_stm32u5 as _;
use bitbox_platform_stm32u5::flash::{self, BootAddressConfig};
#[cfg(feature = "board-stm32u5a9j-dk")]
use board::ffi;
use core::panic::PanicInfo;
use cortex_m_rt::entry;

const BOOT0_ADDR: u32 = 0x0800_2000;
const API_REQUEST_LEN: usize = 512;
const ECHO_PREFIX: &[u8] = b"echo: ";

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    halt()
}

fn halt() -> ! {
    cortex_m::asm::bkpt();
    loop {
        cortex_m::asm::wfe();
    }
}

fn current_nsbootadd0() -> u32 {
    flash::boot_address(BootAddressConfig::NonSecure0)
}

fn program_boot0_nsbootadd0(addr: u32) -> flash::Result<()> {
    let mut unlocked_flash = flash::UnlockedFlash::unlock()?;
    let mut option_bytes = unlocked_flash.unlock_option_bytes()?;
    option_bytes.program_boot_address(BootAddressConfig::NonSecure0, addr)?;

    // Launch will issue a system reset on success. This needs to be the last thing the factory
    // setup does. OB_Launch does not work if flash or option bytes are locked.
    log::info!("launching option bytes; device will reset");
    option_bytes.launch()
}

#[entry]
fn main() -> ! {
    board::init();
    let mut channels = bitbox_debug::rtt_channels_init();
    let nsbootadd0 = current_nsbootadd0();
    if nsbootadd0 != BOOT0_ADDR {
        log::info!(
            "writing nsbootadd0 from {:#010x} to {:#010x}",
            nsbootadd0,
            BOOT0_ADDR
        );
        if program_boot0_nsbootadd0(BOOT0_ADDR).is_err() {
            halt();
        }
    }

    log::info!("nsbootadd0 OK, {:#010x}", nsbootadd0);
    let mut read_buf = [0u8; API_REQUEST_LEN];
    let mut response_buf = [0u8; ECHO_PREFIX.len() + API_REQUEST_LEN];
    loop {
        let read = channels.api_request.read(&mut read_buf);
        if read > 0 {
            response_buf[..ECHO_PREFIX.len()].copy_from_slice(ECHO_PREFIX);
            response_buf[ECHO_PREFIX.len()..ECHO_PREFIX.len() + read]
                .copy_from_slice(&read_buf[..read]);

            let response_len = ECHO_PREFIX.len() + read;
            let written = channels.api_response.write(&response_buf[..response_len]);
            if written != response_len {
                log::warn!("short API response write: {written}");
            }
        }
        unsafe {
            ffi::HAL_Delay(100);
        }
    }
}
