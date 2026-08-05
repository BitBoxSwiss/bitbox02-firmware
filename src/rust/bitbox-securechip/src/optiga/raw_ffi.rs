// SPDX-License-Identifier: Apache-2.0

pub(super) fn status() -> bitbox_securechip_sys::optiga_lib_status_t {
    unsafe { bitbox_securechip_sys::optiga_ops_get_status() }
}

pub(super) fn set_status_busy() {
    unsafe {
        bitbox_securechip_sys::optiga_ops_set_status_busy();
    }
}

pub(super) fn util_instance() -> *mut bitbox_securechip_sys::optiga_util_t {
    unsafe { bitbox_securechip_sys::optiga_util_instance() }
}

pub(super) fn crypt_instance() -> *mut bitbox_securechip_sys::optiga_crypt_t {
    unsafe { bitbox_securechip_sys::optiga_crypt_instance() }
}

pub(super) unsafe fn util_read_data(
    util: *mut bitbox_securechip_sys::optiga_util_t,
    oid: u16,
    offset: u16,
    out: *mut u8,
    out_len: *mut u16,
) -> bitbox_securechip_sys::optiga_lib_status_t {
    unsafe { bitbox_securechip_sys::optiga_util_read_data(util, oid, offset, out, out_len) }
}

pub(super) unsafe fn crypt_hmac(
    crypt: *mut bitbox_securechip_sys::optiga_crypt_t,
    hmac_type: bitbox_securechip_sys::optiga_hmac_type_t,
    secret: u16,
    input: *const u8,
    input_len: u32,
    mac: *mut u8,
    mac_len: *mut u32,
) -> bitbox_securechip_sys::optiga_lib_status_t {
    unsafe {
        bitbox_securechip_sys::optiga_crypt_hmac(
            crypt, hmac_type, secret, input, input_len, mac, mac_len,
        )
    }
}

pub(super) unsafe fn util_write_data(
    util: *mut bitbox_securechip_sys::optiga_util_t,
    oid: u16,
    write_type: u8,
    offset: u16,
    input: *const u8,
    input_len: u16,
) -> bitbox_securechip_sys::optiga_lib_status_t {
    unsafe {
        bitbox_securechip_sys::optiga_util_write_data(
            util, oid, write_type, offset, input, input_len,
        )
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) unsafe fn crypt_symmetric_encrypt(
    crypt: *mut bitbox_securechip_sys::optiga_crypt_t,
    encryption_mode: bitbox_securechip_sys::optiga_symmetric_encryption_mode_t,
    symmetric_key_oid: bitbox_securechip_sys::optiga_key_id_t,
    plain_data: *const u8,
    plain_data_len: u32,
    iv: *const u8,
    iv_len: u16,
    associated_data: *const u8,
    associated_data_len: u16,
    encrypted_data: *mut u8,
    encrypted_data_len: *mut u32,
) -> bitbox_securechip_sys::optiga_lib_status_t {
    unsafe {
        bitbox_securechip_sys::optiga_crypt_symmetric_encrypt(
            crypt,
            encryption_mode,
            symmetric_key_oid,
            plain_data,
            plain_data_len,
            iv,
            iv_len,
            associated_data,
            associated_data_len,
            encrypted_data,
            encrypted_data_len,
        )
    }
}

pub(super) unsafe fn crypt_generate_auth_code(
    crypt: *mut bitbox_securechip_sys::optiga_crypt_t,
    rng_type: bitbox_securechip_sys::optiga_rng_type_t,
    optional_data: *const u8,
    optional_data_len: u16,
    random_data: *mut u8,
    random_data_len: u16,
) -> bitbox_securechip_sys::optiga_lib_status_t {
    unsafe {
        bitbox_securechip_sys::optiga_crypt_generate_auth_code(
            crypt,
            rng_type,
            optional_data,
            optional_data_len,
            random_data,
            random_data_len,
        )
    }
}

pub(super) unsafe fn crypt_ecdsa_sign(
    crypt: *mut bitbox_securechip_sys::optiga_crypt_t,
    digest: *const u8,
    digest_len: u8,
    private_key: bitbox_securechip_sys::optiga_key_id_t,
    signature: *mut u8,
    signature_len: *mut u16,
) -> bitbox_securechip_sys::optiga_lib_status_t {
    unsafe {
        bitbox_securechip_sys::optiga_crypt_ecdsa_sign(
            crypt,
            digest,
            digest_len,
            private_key,
            signature,
            signature_len,
        )
    }
}

pub(super) unsafe fn crypt_hmac_verify(
    crypt: *mut bitbox_securechip_sys::optiga_crypt_t,
    hmac_type: bitbox_securechip_sys::optiga_hmac_type_t,
    secret: u16,
    input: *const u8,
    input_len: u32,
    hmac: *const u8,
    hmac_len: u32,
) -> bitbox_securechip_sys::optiga_lib_status_t {
    unsafe {
        bitbox_securechip_sys::optiga_crypt_hmac_verify(
            crypt, hmac_type, secret, input, input_len, hmac, hmac_len,
        )
    }
}

pub(super) unsafe fn crypt_symmetric_generate_key(
    crypt: *mut bitbox_securechip_sys::optiga_crypt_t,
    key_type: bitbox_securechip_sys::optiga_symmetric_key_type_t,
    key_usage: u8,
    export_symmetric_key: u8,
    symmetric_key: *mut core::ffi::c_void,
) -> bitbox_securechip_sys::optiga_lib_status_t {
    unsafe {
        bitbox_securechip_sys::optiga_crypt_symmetric_generate_key(
            crypt,
            key_type,
            key_usage,
            export_symmetric_key,
            symmetric_key,
        )
    }
}

pub(super) unsafe fn crypt_clear_auto_state(
    crypt: *mut bitbox_securechip_sys::optiga_crypt_t,
    secret: u16,
) -> bitbox_securechip_sys::optiga_lib_status_t {
    unsafe { bitbox_securechip_sys::optiga_crypt_clear_auto_state(crypt, secret) }
}

pub(super) unsafe fn crypt_random(
    crypt: *mut bitbox_securechip_sys::optiga_crypt_t,
    rng_type: bitbox_securechip_sys::optiga_rng_type_t,
    out: *mut u8,
    out_len: u16,
) -> bitbox_securechip_sys::optiga_lib_status_t {
    unsafe { bitbox_securechip_sys::optiga_crypt_random(crypt, rng_type, out, out_len) }
}
