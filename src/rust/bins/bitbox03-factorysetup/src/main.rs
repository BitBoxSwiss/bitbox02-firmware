// SPDX-License-Identifier: Apache-2.0

#![no_std]
#![no_main]

mod attestation_root_pubkeys;
mod memory;

use bitbox_board_stm32u5_dk::ffi;
use bitbox_mcu_stm32u5 as _;
use bitbox_platform_stm32u5 as _;
use bitbox_securechip::optiga;
//use bitcoin::secp256k1::{Message, PublicKey, Secp256k1, ecdsa::Signature};
use core::panic::PanicInfo;
use cortex_m_rt::entry;
use embedded_alloc::LlffHeap as Heap;
use memory::{
    get_attestation_bootloader_hash, get_stored_attestation_device_pubkey,
    set_attestation_bootloader_hash, set_attestation_certificate, set_attestation_device_pubkey,
};
use rtt_target::ChannelMode;
use sha2::{Digest, Sha256};

use crate::attestation_root_pubkeys::ATTESTATION_ROOT_PUBKEYS;

const BOOT0_ADDR: u32 = 0x0800_2000;
const OP_SET_ATTESTATION_BOOTLOADER_HASH: u8 = b'a';
const OP_GEN_ATTESTATION_KEY: u8 = b'g';
const OP_SET_ATTESTATION_CERTIFICATE: u8 = b'c';
const OP_PROGRAM_BOOT0: u8 = b'p';
const OP_RESET_BOOT0: u8 = b'r';
const OP_QUIT: u8 = b'q';

const ATTESTATION_PUBKEY_LEN: usize = 64;
const ATTESTATION_CERTIFICATE_LEN: usize = 64;
const ROOT_PUBKEY_IDENTIFIER_LEN: usize = 32;
const ATTESTATION_SIGHASH_LEN: usize = 32;
const API_RESPONSE_HEADER_LEN: usize = 2;
const SET_ATTESTATION_CERTIFICATE_LEN: usize =
    1 + ATTESTATION_CERTIFICATE_LEN + ROOT_PUBKEY_IDENTIFIER_LEN;
const SET_ATTESTATION_CERTIFICATE_WITH_PUBKEY_LEN: usize =
    1 + ATTESTATION_PUBKEY_LEN + ATTESTATION_CERTIFICATE_LEN + ROOT_PUBKEY_IDENTIFIER_LEN;

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[derive(Copy, Clone)]
#[repr(u8)]
enum ErrorCode {
    Ok = 0,
    InvalidInput = 1,
    Failed = 2,
    UnknownCommand = 3,
}

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

fn ensure_hal_ok(status: ffi::HAL_StatusTypeDef) -> Result<(), ()> {
    if status as u32 == 0 { Ok(()) } else { Err(()) }
}

//fn current_nsbootadd0() -> u32 {
//    unsafe {
//        let mut option_bytes = ffi::FLASH_OBProgramInitTypeDef::default();
//        option_bytes.BootAddrConfig = ffi::OB_BOOTADDR_NS0;
//        ffi::HAL_FLASHEx_OBGetConfig(&mut option_bytes);
//        option_bytes.BootAddr
//    }
//}

fn program_boot0_nsbootadd0(addr: u32) -> Result<(), ()> {
    unsafe {
        ensure_hal_ok(ffi::HAL_FLASH_Unlock())?;
        ensure_hal_ok(ffi::HAL_FLASH_OB_Unlock())?;

        let mut option_bytes = ffi::FLASH_OBProgramInitTypeDef::default();
        option_bytes.OptionType = ffi::OPTIONBYTE_BOOTADDR;
        option_bytes.BootAddrConfig = ffi::OB_BOOTADDR_NS0;
        option_bytes.BootAddr = addr;

        ensure_hal_ok(ffi::HAL_FLASHEx_OBProgram(&mut option_bytes))?;

        // Launch will issue a system reset on success. This needs to be the last thing the factory
        // setup does. OB_Launch does not work if flash or option bytes are locked.
        ensure_hal_ok(ffi::HAL_FLASH_OB_Launch())
    }
}

fn attestation_sighash(
    attestation_device_pubkey: &[u8; ATTESTATION_PUBKEY_LEN],
) -> Result<[u8; 32], ()> {
    let mut hasher = Sha256::new();
    hasher.update(get_attestation_bootloader_hash()?);
    hasher.update(attestation_device_pubkey);
    Ok(hasher.finalize().into())
}

//fn verify_attestation_certificate(
//    attestation_device_pubkey: &[u8; ATTESTATION_PUBKEY_LEN],
//    certificate: &[u8; ATTESTATION_CERTIFICATE_LEN],
//) -> bool {
//    let Ok(signature) = Signature::from_compact(certificate) else {
//        return false;
//    };
//    let Ok(sighash) = attestation_sighash(attestation_device_pubkey) else {
//        return false;
//    };
//    let Ok(message) = Message::from_digest_slice(&sighash) else {
//        return false;
//    };
//    let secp = Secp256k1::new();
//
//    for root_pubkey in ATTESTATION_ROOT_PUBKEYS {
//        let Ok(root_pubkey) = PublicKey::from_slice(&root_pubkey) else {
//            continue;
//        };
//        if secp
//            .verify_ecdsa(&message, &signature, &root_pubkey)
//            .is_ok()
//        {
//            return true;
//        }
//    }
//    false
//}

fn prepare_attestation_device_pubkey() -> Result<[u8; ATTESTATION_PUBKEY_LEN], ()> {
    if let Some(pubkey) = get_stored_attestation_device_pubkey()? {
        return Ok(pubkey);
    }

    let mut pubkey = [0u8; ATTESTATION_PUBKEY_LEN];
    //optiga::gen_attestation_key(&mut pubkey)?;
    set_attestation_device_pubkey(&pubkey)?;
    Ok(pubkey)
}

fn handle_api_request(
    input: &[u8],
    attestation_device_pubkey: &[u8; ATTESTATION_PUBKEY_LEN],
    response: &mut [u8; API_RESPONSE_HEADER_LEN + ATTESTATION_PUBKEY_LEN],
) -> usize {
    let opcode = input.first().copied().unwrap_or_default();
    response[0] = opcode;
    let error_code = match opcode {
        OP_SET_ATTESTATION_BOOTLOADER_HASH => {
            if input.len() != 1 + ATTESTATION_SIGHASH_LEN {
                ErrorCode::InvalidInput
            } else {
                let mut bootloader_hash = [0u8; ATTESTATION_SIGHASH_LEN];
                bootloader_hash.copy_from_slice(&input[1..]);
                match set_attestation_bootloader_hash(&bootloader_hash) {
                    Ok(()) => {
                        response[API_RESPONSE_HEADER_LEN
                            ..API_RESPONSE_HEADER_LEN + ATTESTATION_PUBKEY_LEN]
                            .copy_from_slice(attestation_device_pubkey);
                        response[1] = ErrorCode::Ok as u8;
                        return API_RESPONSE_HEADER_LEN + ATTESTATION_PUBKEY_LEN;
                    }
                    Err(()) => ErrorCode::Failed,
                }
            }
        }
        OP_GEN_ATTESTATION_KEY => {
            if input.len() != 1 {
                ErrorCode::InvalidInput
            } else {
                response[API_RESPONSE_HEADER_LEN..API_RESPONSE_HEADER_LEN + ATTESTATION_PUBKEY_LEN]
                    .copy_from_slice(attestation_device_pubkey);
                response[1] = ErrorCode::Ok as u8;
                return API_RESPONSE_HEADER_LEN + ATTESTATION_PUBKEY_LEN;
            }
        }
        OP_SET_ATTESTATION_CERTIFICATE => {
            if input.len() != SET_ATTESTATION_CERTIFICATE_LEN
                && input.len() != SET_ATTESTATION_CERTIFICATE_WITH_PUBKEY_LEN
            {
                ErrorCode::InvalidInput
            } else {
                let mut certificate = [0u8; ATTESTATION_CERTIFICATE_LEN];
                let mut root_pubkey_identifier = [0u8; ROOT_PUBKEY_IDENTIFIER_LEN];
                let payload = if input.len() == SET_ATTESTATION_CERTIFICATE_WITH_PUBKEY_LEN {
                    let mut supplied_pubkey = [0u8; ATTESTATION_PUBKEY_LEN];
                    supplied_pubkey.copy_from_slice(&input[1..1 + ATTESTATION_PUBKEY_LEN]);
                    if supplied_pubkey != *attestation_device_pubkey {
                        None
                    } else {
                        Some(&input[1 + ATTESTATION_PUBKEY_LEN..])
                    }
                } else {
                    Some(&input[1..])
                };

                let Some(payload) = payload else {
                    return {
                        response[1] = ErrorCode::InvalidInput as u8;
                        API_RESPONSE_HEADER_LEN
                    };
                };
                certificate.copy_from_slice(&payload[..ATTESTATION_CERTIFICATE_LEN]);
                root_pubkey_identifier.copy_from_slice(&payload[ATTESTATION_CERTIFICATE_LEN..]);
                ErrorCode::InvalidInput

                //if !verify_attestation_certificate(attestation_device_pubkey, &certificate) {
                //    ErrorCode::InvalidInput
                //} else {
                //    match set_attestation_certificate(
                //        attestation_device_pubkey,
                //        &certificate,
                //        &root_pubkey_identifier,
                //    ) {
                //        Ok(()) => ErrorCode::Ok,
                //        Err(()) => ErrorCode::Failed,
                //    }
                //}
            }
        }
        _ => ErrorCode::UnknownCommand,
    };
    response[1] = error_code as u8;
    API_RESPONSE_HEADER_LEN
}

#[entry]
fn main() -> ! {
    unsafe {
        ffi::board_init_essentials();
    }
    unsafe {
        embedded_alloc::init!(HEAP, 8 * 1024);
    }
    let attestation_device_pubkey = match prepare_attestation_device_pubkey() {
        Ok(pubkey) => pubkey,
        Err(()) => halt(),
    };
    // OpenOCD maps channels id to tcp port so an empty channel 0 down is defined
    let mut channels = rtt_target::rtt_init! {
        up: {
            0: {
                size: 1024,
                mode: ChannelMode::NoBlockSkip,
                name: "Terminal",
                section: ".segger_rtt_buf",
            }
            1: {
                size: 1024,
                mode: ChannelMode::NoBlockSkip,
                name: "API Response",
                section: ".segger_rtt_buf",
            }
        }
        down: {
            0: {
                size: 16,
                mode: ChannelMode::NoBlockSkip,
                name: "Terminal",
                section: ".segger_rtt_buf",
            }
            1: {
                size: 1024,
                mode: ChannelMode::NoBlockSkip,
                name: "API Request",
                section: ".segger_rtt_buf",
            }
        }
        section_cb: ".segger_rtt"
    };
    rtt_target::set_print_channel(channels.up.0);
    //let current_nsbootadd0 = current_nsbootadd0();
    //rtt_target::rprintln!("NSBOOTADD0=0x{:08x}", current_nsbootadd0);

    rtt_target::rprintln!("OK");
    let mut read_buf = [0u8; 512];
    let mut response_buf = [0u8; API_RESPONSE_HEADER_LEN + ATTESTATION_PUBKEY_LEN];
    loop {
        let read = channels.down.1.read(&mut read_buf);
        if read > 0 {
            match read_buf[0] {
                OP_QUIT => break,
                OP_RESET_BOOT0 => {
                    // reset
                    if program_boot0_nsbootadd0(0x08000000).is_err() {
                        halt();
                    }
                }
                OP_PROGRAM_BOOT0 => {
                    // program
                    if program_boot0_nsbootadd0(BOOT0_ADDR).is_err() {
                        halt();
                    }
                }
                _ => {
                    let response_len = handle_api_request(
                        &read_buf[..read],
                        &attestation_device_pubkey,
                        &mut response_buf,
                    );
                    let written = channels.up.1.write(&response_buf[..response_len]);
                    if written != response_len {
                        rtt_target::rprintln!("short API response write: {written}");
                    }
                }
            }
        }
        // Poll with 100ms
        unsafe {
            bitbox_board_stm32u5_dk::ffi::HAL_Delay(100);
        }
    }

    cortex_m::peripheral::SCB::sys_reset()
}
