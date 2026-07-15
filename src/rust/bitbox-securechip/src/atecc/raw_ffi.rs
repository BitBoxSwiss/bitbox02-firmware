// SPDX-License-Identifier: Apache-2.0

use super::{BLOCK_SIZE, NONCE_NUMIN_SIZE, SIGNATURE_SIZE, Slot};
use zeroize::Zeroizing;

pub(super) fn status() -> i32 {
    unsafe { bitbox_securechip_sys::atecc_ops_get_status() }
}

pub(super) fn poll_delay_ms() -> u32 {
    unsafe { bitbox_securechip_sys::atecc_ops_get_poll_delay_ms() }
}

pub(super) fn poll() {
    unsafe {
        bitbox_securechip_sys::atecc_ops_poll();
    }
}

pub(super) fn clear_io_temp_key() {
    unsafe {
        bitbox_securechip_sys::atecc_io_clear_tempkey();
    }
}

pub(super) fn start_nonce_rand(num_in: &[u8; NONCE_NUMIN_SIZE]) -> i32 {
    unsafe { bitbox_securechip_sys::atecc_cmd_start_nonce_rand(num_in.as_ptr()) }
}

pub(super) fn start_checkmac(response: &Zeroizing<[u8; 32]>) -> i32 {
    unsafe { bitbox_securechip_sys::atecc_cmd_start_checkmac(response.as_ptr()) }
}

pub(super) fn start_random() -> i32 {
    unsafe { bitbox_securechip_sys::atecc_cmd_start_random() }
}

pub(super) fn start_counter_read() -> i32 {
    unsafe { bitbox_securechip_sys::atecc_cmd_start_counter_read() }
}

pub(super) fn start_info_revision() -> i32 {
    unsafe { bitbox_securechip_sys::atecc_cmd_start_info_revision() }
}

pub(super) fn start_kdf(slot: Slot, msg: &[u8; 32]) -> i32 {
    unsafe { bitbox_securechip_sys::atecc_cmd_start_kdf(slot, msg.as_ptr(), msg.len()) }
}

pub(super) fn start_derivekey_rollkey() -> i32 {
    unsafe { bitbox_securechip_sys::atecc_cmd_start_derivekey_rollkey() }
}

pub(super) fn start_nonce_load_msgdigest(msg: &[u8; 32]) -> i32 {
    unsafe { bitbox_securechip_sys::atecc_cmd_start_nonce_load_msgdigest(msg.as_ptr()) }
}

pub(super) fn start_sign_attestation() -> i32 {
    unsafe { bitbox_securechip_sys::atecc_cmd_start_sign_attestation() }
}

pub(super) fn start_gendig_encryption_key() -> i32 {
    unsafe { bitbox_securechip_sys::atecc_cmd_start_gendig_encryption_key() }
}

#[cfg(any(feature = "app-u2f", feature = "factory-setup"))]
pub(super) fn start_read_block(slot: Slot, block: u8) -> i32 {
    unsafe { bitbox_securechip_sys::atecc_cmd_start_read_block(slot as u16, block) }
}

pub(super) fn start_write_encrypted_block(
    slot: Slot,
    block: u8,
    value: &Zeroizing<[u8; BLOCK_SIZE]>,
    mac: &Zeroizing<[u8; BLOCK_SIZE]>,
) -> i32 {
    unsafe {
        bitbox_securechip_sys::atecc_cmd_start_write_encrypted_block(
            slot as u16,
            block,
            value.as_ptr(),
            mac.as_ptr(),
        )
    }
}

pub(super) fn read_random_response(out: &mut Zeroizing<[u8; 32]>) -> i32 {
    unsafe { bitbox_securechip_sys::atecc_cmd_read_random_response(out.as_mut_ptr()) }
}

pub(super) fn read_counter_response(out: &mut u32) -> i32 {
    unsafe { bitbox_securechip_sys::atecc_cmd_read_counter_response(out) }
}

pub(super) fn read_info_response(out: &mut [u8; 4]) -> i32 {
    unsafe { bitbox_securechip_sys::atecc_cmd_read_info_response(out.as_mut_ptr()) }
}

pub(super) fn read_kdf_response(
    data: &mut Zeroizing<[u8; 32]>,
    nonce: &mut Zeroizing<[u8; 32]>,
) -> i32 {
    unsafe {
        bitbox_securechip_sys::atecc_cmd_read_kdf_response(data.as_mut_ptr(), nonce.as_mut_ptr())
    }
}

pub(super) fn read_sign_response(out: &mut [u8; SIGNATURE_SIZE]) -> i32 {
    unsafe { bitbox_securechip_sys::atecc_cmd_read_sign_response(out.as_mut_ptr()) }
}

#[cfg(any(feature = "app-u2f", feature = "factory-setup"))]
pub(super) fn read_block_response(out: &mut Zeroizing<[u8; BLOCK_SIZE]>) -> i32 {
    unsafe { bitbox_securechip_sys::atecc_cmd_read_block_response(out.as_mut_ptr()) }
}

pub(super) fn auth_compute_response(
    num_in: &[u8; NONCE_NUMIN_SIZE],
    rand_out: &Zeroizing<[u8; 32]>,
    auth_key: &[u8; 32],
    response: &mut Zeroizing<[u8; 32]>,
) -> i32 {
    unsafe {
        bitbox_securechip_sys::atecc_auth_compute_response(
            num_in.as_ptr(),
            rand_out.as_ptr(),
            auth_key.as_ptr(),
            response.as_mut_ptr(),
        )
    }
}

pub(super) fn kdf_decrypt(
    io_protection_key: &[u8; 32],
    nonce_out: &Zeroizing<[u8; 32]>,
    data: &mut Zeroizing<[u8; 32]>,
) -> i32 {
    unsafe {
        bitbox_securechip_sys::atecc_kdf_decrypt(
            io_protection_key.as_ptr(),
            nonce_out.as_ptr(),
            data.as_mut_ptr(),
            BLOCK_SIZE,
        )
    }
}

pub(super) fn io_prepare_tempkey(
    num_in: &[u8; NONCE_NUMIN_SIZE],
    rand_out: &Zeroizing<[u8; 32]>,
) -> i32 {
    unsafe { bitbox_securechip_sys::atecc_io_prepare_tempkey(num_in.as_ptr(), rand_out.as_ptr()) }
}

pub(super) fn io_apply_gendig(encryption_key: &[u8; 32]) -> i32 {
    unsafe { bitbox_securechip_sys::atecc_io_apply_gendig(encryption_key.as_ptr()) }
}

pub(super) fn io_prepare_encrypted_write(
    slot: Slot,
    block: u8,
    input: &Zeroizing<[u8; BLOCK_SIZE]>,
    encrypted: &mut Zeroizing<[u8; BLOCK_SIZE]>,
    mac: &mut Zeroizing<[u8; BLOCK_SIZE]>,
) -> i32 {
    unsafe {
        bitbox_securechip_sys::atecc_io_prepare_encrypted_write(
            slot as u16,
            block,
            input.as_ptr(),
            encrypted.as_mut_ptr(),
            mac.as_mut_ptr(),
        )
    }
}

#[cfg(any(feature = "app-u2f", feature = "factory-setup"))]
pub(super) fn io_decrypt_block(data: &mut Zeroizing<[u8; BLOCK_SIZE]>) -> i32 {
    unsafe { bitbox_securechip_sys::atecc_io_decrypt_block(data.as_mut_ptr(), BLOCK_SIZE) }
}
