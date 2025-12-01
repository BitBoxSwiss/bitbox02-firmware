// Copyright 2020 Shift Cryptosecurity AG
// Copyright 2020 Shift Crypto AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub const MAX_UNLOCK_ATTEMPTS: u8 = bitbox02_sys::MAX_UNLOCK_ATTEMPTS as u8;

// deduct one for the null terminator.
pub const DEVICE_NAME_MAX_LEN: usize = bitbox02_sys::MEMORY_DEVICE_NAME_MAX_LEN as usize - 1;

// deduct one for the null terminator.
pub const MULTISIG_NAME_MAX_LEN: usize = bitbox02_sys::MEMORY_MULTISIG_NAME_MAX_LEN as usize - 1;

pub use bitbox02_sys::memory_ble_metadata_t as BleMetadata;

pub use bitbox02_sys::memory_result_t as MemoryError;

#[derive(Debug)]
pub struct Error;

pub fn get_device_name() -> String {
    let mut name = [0u8; DEVICE_NAME_MAX_LEN + 1];
    unsafe { bitbox02_sys::memory_get_device_name(name.as_mut_ptr().cast()) }
    crate::util::str_from_null_terminated(&name[..])
        .unwrap()
        .into()
}

pub fn set_device_name(name: &str) -> Result<(), Error> {
    match unsafe {
        bitbox02_sys::memory_set_device_name(
            crate::util::str_to_cstr_vec(name)
                .or(Err(Error))?
                .as_ptr()
                .cast(),
        )
    } {
        true => Ok(()),
        false => Err(Error),
    }
}

pub fn is_initialized() -> bool {
    unsafe { bitbox02_sys::memory_is_initialized() }
}

pub fn set_initialized() -> Result<(), ()> {
    match unsafe { bitbox02_sys::memory_set_initialized() } {
        true => Ok(()),
        false => Err(()),
    }
}

pub fn is_seeded() -> bool {
    unsafe { bitbox02_sys::memory_is_seeded() }
}

pub fn is_mnemonic_passphrase_enabled() -> bool {
    unsafe { bitbox02_sys::memory_is_mnemonic_passphrase_enabled() }
}

pub fn get_attestation_bootloader_hash() -> [u8; 32] {
    let mut hash = [0u8; 32];
    unsafe {
        bitbox02_sys::memory_get_attestation_bootloader_hash(hash.as_mut_ptr());
    }
    hash
}

pub fn get_attestation_pubkey_and_certificate(
    device_pubkey: &mut [u8; 64],
    certificate: &mut [u8; 64],
    root_pubkey_identifier: &mut [u8; 32],
) -> Result<(), ()> {
    match unsafe {
        bitbox02_sys::memory_get_attestation_pubkey_and_certificate(
            device_pubkey.as_mut_ptr(),
            certificate.as_mut_ptr(),
            root_pubkey_identifier.as_mut_ptr(),
        )
    } {
        true => Ok(()),
        false => Err(()),
    }
}

pub fn get_encrypted_seed_and_hmac() -> Result<alloc::vec::Vec<u8>, ()> {
    let mut out = vec![0u8; 96];
    let mut len = 0u8;
    match unsafe { bitbox02_sys::memory_get_encrypted_seed_and_hmac(out.as_mut_ptr(), &mut len) } {
        true => {
            out.truncate(len as _);
            Ok(out)
        }
        false => Err(()),
    }
}

pub fn get_noise_static_private_key() -> Result<zeroize::Zeroizing<[u8; 32]>, ()> {
    let mut out = zeroize::Zeroizing::new([0u8; 32]);
    match unsafe { bitbox02_sys::memory_get_noise_static_private_key(out.as_mut_ptr()) } {
        true => Ok(out),
        false => Err(()),
    }
}

pub fn check_noise_remote_static_pubkey(pubkey: &[u8; 32]) -> bool {
    unsafe { bitbox02_sys::memory_check_noise_remote_static_pubkey(pubkey.as_ptr()) }
}

pub fn add_noise_remote_static_pubkey(pubkey: &[u8; 32]) -> Result<(), ()> {
    match unsafe { bitbox02_sys::memory_add_noise_remote_static_pubkey(pubkey.as_ptr()) } {
        true => Ok(()),
        false => Err(()),
    }
}

pub fn set_mnemonic_passphrase_enabled(enabled: bool) -> Result<(), ()> {
    match unsafe { bitbox02_sys::memory_set_mnemonic_passphrase_enabled(enabled) } {
        true => Ok(()),
        false => Err(()),
    }
}

pub fn set_seed_birthdate(timestamp: u32) -> Result<(), ()> {
    match unsafe { bitbox02_sys::memory_set_seed_birthdate(timestamp) } {
        true => Ok(()),
        false => Err(()),
    }
}

pub fn get_seed_birthdate() -> u32 {
    let mut timestamp = core::mem::MaybeUninit::uninit();
    unsafe {
        bitbox02_sys::memory_get_seed_birthdate(timestamp.as_mut_ptr());
        timestamp.assume_init()
    }
}

pub fn set_encrypted_seed_and_hmac(data: &[u8]) -> Result<(), ()> {
    if data.len() > u8::MAX as usize {
        return Err(());
    }
    match unsafe {
        bitbox02_sys::memory_set_encrypted_seed_and_hmac(data.as_ptr(), data.len() as u8)
    } {
        true => Ok(()),
        false => Err(()),
    }
}

pub fn reset_hww() -> Result<(), ()> {
    match unsafe { bitbox02_sys::memory_reset_hww() } {
        true => Ok(()),
        false => Err(()),
    }
}

pub fn smarteeprom_get_unlock_attempts() -> u8 {
    unsafe { bitbox02_sys::bitbox02_smarteeprom_get_unlock_attempts() }
}

pub fn smarteeprom_increment_unlock_attempts() {
    unsafe {
        bitbox02_sys::bitbox02_smarteeprom_increment_unlock_attempts();
    }
}

pub fn smarteeprom_reset_unlock_attempts() {
    unsafe {
        bitbox02_sys::bitbox02_smarteeprom_reset_unlock_attempts();
    }
}

/// Testing helper to set the recorded number of unlock attempts to a precise value.
/// Panics if `attempts` exceeds the maximum supported by the firmware.
#[cfg(feature = "testing")]
pub fn set_unlock_attempts_for_testing(attempts: u8) {
    assert!(attempts <= MAX_UNLOCK_ATTEMPTS);
    smarteeprom_reset_unlock_attempts();
    for _ in 0..attempts {
        smarteeprom_increment_unlock_attempts();
    }
}

pub fn multisig_set_by_hash(hash: &[u8], name: &str) -> Result<(), MemoryError> {
    if hash.len() != 32 {
        return Err(MemoryError::MEMORY_ERR_INVALID_INPUT);
    }
    match unsafe {
        bitbox02_sys::memory_multisig_set_by_hash(
            hash.as_ptr(),
            crate::util::str_to_cstr_vec(name)
                .or(Err(MemoryError::MEMORY_ERR_INVALID_INPUT))?
                .as_ptr()
                .cast(),
        )
    } {
        MemoryError::MEMORY_OK => Ok(()),
        err => Err(err),
    }
}

pub fn multisig_get_by_hash(hash: &[u8]) -> Option<String> {
    let mut name = [0u8; MULTISIG_NAME_MAX_LEN + 1];
    match unsafe {
        bitbox02_sys::memory_multisig_get_by_hash(hash.as_ptr(), name.as_mut_ptr().cast())
    } {
        true => Some(
            crate::util::str_from_null_terminated(&name[..])
                .unwrap()
                .into(),
        ),
        false => None,
    }
}

pub enum Platform {
    BitBox02,
    BitBox02Plus,
}

pub fn get_platform() -> Result<Platform, ()> {
    match unsafe { bitbox02_sys::memory_get_platform() as u32 } {
        bitbox02_sys::MEMORY_PLATFORM_BITBOX02 => Ok(Platform::BitBox02),
        bitbox02_sys::MEMORY_PLATFORM_BITBOX02_PLUS => Ok(Platform::BitBox02Plus),
        _ => Err(()),
    }
}

pub enum SecurechipType {
    Atecc,
    Optiga,
}

pub fn get_securechip_type() -> Result<SecurechipType, ()> {
    match unsafe { bitbox02_sys::memory_get_securechip_type() as u32 } {
        bitbox02_sys::MEMORY_SECURECHIP_TYPE_ATECC => Ok(SecurechipType::Atecc),
        bitbox02_sys::MEMORY_SECURECHIP_TYPE_OPTIGA => Ok(SecurechipType::Optiga),
        _ => Err(()),
    }
}

pub fn get_ble_metadata() -> BleMetadata {
    let mut metadata = core::mem::MaybeUninit::uninit();

    unsafe {
        bitbox02_sys::memory_get_ble_metadata(metadata.as_mut_ptr());
        metadata.assume_init()
    }
}

pub fn set_ble_metadata(metadata: &BleMetadata) -> Result<(), ()> {
    match unsafe { bitbox02_sys::memory_set_ble_metadata(metadata) } {
        true => Ok(()),
        false => Err(()),
    }
}

pub fn ble_enabled() -> bool {
    unsafe { bitbox02_sys::memory_ble_enabled() }
}

pub fn ble_enable(enable: bool) -> Result<(), ()> {
    let res = unsafe { bitbox02_sys::memory_ble_enable(enable) };
    if res { Ok(()) } else { Err(()) }
}

pub fn get_salt_root() -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
    let mut salt_root = zeroize::Zeroizing::new(vec![0u8; 32]);
    if unsafe { bitbox02_sys::memory_get_salt_root(salt_root.as_mut_ptr()) } {
        Ok(salt_root)
    } else {
        Err(())
    }
}

#[cfg(feature = "testing")]
pub fn set_salt_root(salt_root: &[u8; 32]) -> Result<(), ()> {
    match unsafe { bitbox02_sys::memory_set_salt_root(salt_root.as_ptr()) } {
        true => Ok(()),
        false => Err(()),
    }
}

#[cfg(test)]
fn fake_memory_factoryreset() {
    unsafe { bitbox02_sys::fake_memory_factoryreset() }
}

#[cfg(test)]
fn memory_bootloader_hash() -> [u8; 32] {
    let mut out = [0u8; 32];
    unsafe { bitbox02_sys::memory_bootloader_hash(out.as_mut_ptr()) };
    out
}

#[cfg(test)]
fn set_bootloader_hash_fake(hash: &[u8; 32]) {
    unsafe { bitbox02_sys::memory_set_bootloader_hash_fake(hash.as_ptr()) }
}

#[cfg(any(feature = "testing", feature = "simulator-graphical"))]
pub(crate) fn memory_setup(random_fn: unsafe extern "C" fn(*mut u8)) -> bool {
    unsafe {
        static mut MEMORY_IFS: bitbox02_sys::memory_interface_functions_t =
            bitbox02_sys::memory_interface_functions_t {
                random_32_bytes: None,
            };
        MEMORY_IFS.random_32_bytes = Some(random_fn);
        bitbox02_sys::memory_setup(core::ptr::addr_of!(MEMORY_IFS))
    }
}

#[cfg(test)]
fn set_attestation_device_pubkey(pubkey: &[u8; 64]) -> bool {
    unsafe { bitbox02_sys::memory_set_attestation_device_pubkey(pubkey.as_ptr()) }
}

#[cfg(test)]
fn set_attestation_certificate(
    pubkey: &[u8; 64],
    certificate: &[u8; 64],
    root_identifier: &[u8; 32],
) -> bool {
    unsafe {
        bitbox02_sys::memory_set_attestation_certificate(
            pubkey.as_ptr(),
            certificate.as_ptr(),
            root_identifier.as_ptr(),
        )
    }
}

#[cfg(test)]
fn get_io_protection_key(out: &mut [u8; 32]) {
    unsafe { bitbox02_sys::memory_get_io_protection_key(out.as_mut_ptr()) }
}

#[cfg(test)]
fn get_authorization_key(out: &mut [u8; 32]) {
    unsafe { bitbox02_sys::memory_get_authorization_key(out.as_mut_ptr()) }
}

#[cfg(test)]
fn get_encryption_key(out: &mut [u8; 32]) {
    unsafe { bitbox02_sys::memory_get_encryption_key(out.as_mut_ptr()) }
}

#[cfg(test)]
fn bootloader_set_flags(auto_enter: u8, upside_down: bool) -> bool {
    unsafe {
        bitbox02_sys::memory_bootloader_set_flags(
            bitbox02_sys::auto_enter_t { value: auto_enter },
            bitbox02_sys::upside_down_t { value: upside_down },
        )
    }
}

#[cfg(test)]
fn set_attestation_bootloader_hash(hash: &[u8; 32]) -> bool {
    unsafe { bitbox02_sys::memory_set_attestation_bootloader_hash(hash.as_ptr()) }
}

#[cfg(all(test, feature = "testing"))]
mod tests {
    use super::*;
    use crate::random;
    use crate::testing::mock_memory;
    use alloc::{format, string::String, vec, vec::Vec};
    use core::{
        ptr, slice,
        sync::atomic::{AtomicUsize, Ordering},
    };
    use hex_lit::hex;

    #[test]
    fn test_get_salt_root_roundtrip() {
        mock_memory();
        let original = get_salt_root().unwrap();

        let expected = hex!("00112233445566778899aabbccddeefffeeddccbbaa998877665544332211000");

        set_salt_root(&expected).unwrap();
        let salt_root = get_salt_root().unwrap();
        assert_eq!(salt_root.as_slice(), &expected);

        let erased = [0xffu8; 32];
        set_salt_root(&erased).unwrap();
        assert!(get_salt_root().is_err());

        set_salt_root(original.as_slice().try_into().unwrap()).unwrap();
    }

    #[test]
    fn test_get_set_device_name() {
        mock_memory();

        let original = get_device_name();
        assert_eq!(original, "My BitBox");

        let new_name = "Test device name";

        set_device_name(new_name).unwrap();
        assert_eq!(get_device_name(), new_name);

        // A name with the maximum allowed length is accepted.
        let max_len_name = "DeviceName_ABCDEFGHIJKLMNOPQRSTUVWXYZ_abcdefghijklmnopqrstuvwxy";
        assert_eq!(max_len_name.len(), DEVICE_NAME_MAX_LEN);
        set_device_name(max_len_name).unwrap();
        assert_eq!(get_device_name(), max_len_name);

        // Longer names are truncated to the maximum length.
        let long_name = format!("{max_len_name}foobar");
        set_device_name(&long_name).unwrap();
        let stored = get_device_name();
        assert_eq!(stored, max_len_name);

        // Invalid names are rejected and must not change the stored value.
        let invalid_names = [
            "",         // empty
            " name",    // leading space
            "name ",    // trailing space
            "foo\nbar", // control character
            "Ä",        // non-ASCII
            "漢字",     // non-ASCII
        ];

        for invalid in invalid_names {
            assert!(set_device_name(invalid).is_err());
            assert_eq!(get_device_name(), stored);
        }
    }

    #[test]
    fn test_is_set_initialized() {
        mock_memory();

        assert!(!is_initialized());
        assert!(!is_seeded());
        assert!(set_initialized().is_err());

        let seed_data: Vec<u8> = (0..96).map(|i| i as u8).collect();
        set_encrypted_seed_and_hmac(&seed_data).unwrap();
        assert!(is_seeded());
        assert!(!is_initialized());

        set_initialized().unwrap();
        assert!(is_initialized());
    }

    #[test]
    fn test_mnemonic_passphrase_enabled_roundtrip() {
        mock_memory();

        assert!(!is_mnemonic_passphrase_enabled());

        set_mnemonic_passphrase_enabled(true).unwrap();
        assert!(is_mnemonic_passphrase_enabled());

        set_mnemonic_passphrase_enabled(false).unwrap();
        assert!(!is_mnemonic_passphrase_enabled());
    }

    #[test]
    fn test_seed_birthdate_roundtrip() {
        mock_memory();

        assert_eq!(get_seed_birthdate(), 0);

        let timestamp: u32 = 0xABCDEF11;
        set_seed_birthdate(timestamp).unwrap();
        assert_eq!(get_seed_birthdate(), timestamp);
    }

    #[test]
    fn test_ble_metadata_roundtrip() {
        mock_memory();

        let new_metadata = BleMetadata {
            allowed_firmware_hash: [0x11u8; 32],
            active_index: 1,
            firmware_sizes: [1024u16, 2048u16],
            firmware_checksums: [0xAAu8, 0xBBu8],
        };

        set_ble_metadata(&new_metadata).unwrap();
        let readback = get_ble_metadata();

        assert_eq!(
            readback.allowed_firmware_hash,
            new_metadata.allowed_firmware_hash
        );
        assert_eq!(readback.active_index, new_metadata.active_index);
        assert_eq!(readback.firmware_sizes, new_metadata.firmware_sizes);
        assert_eq!(readback.firmware_checksums, new_metadata.firmware_checksums);
    }

    #[test]
    fn test_ble_enable_and_enabled() {
        mock_memory();

        // Default after factory setup is BLE enabled.
        assert!(ble_enabled());

        ble_enable(false).unwrap();
        assert!(!ble_enabled());

        ble_enable(true).unwrap();
        assert!(ble_enabled());
    }

    #[test]
    fn test_noise_static_key_and_remote_pubkeys() {
        mock_memory();

        let static_key = get_noise_static_private_key().unwrap();
        assert!(static_key.iter().any(|&b| b != 0xff));

        let pubkeys: [[u8; 32]; 6] = [
            [0x11u8; 32],
            [0x22u8; 32],
            [0x33u8; 32],
            [0x44u8; 32],
            [0x55u8; 32],
            [0x66u8; 32],
        ];

        // Initially, none of the pubkeys are stored.
        for key in pubkeys.iter() {
            assert!(!check_noise_remote_static_pubkey(key));
        }

        // Add the first five pubkeys. All added ones must be found, the rest not.
        for i in 0..5 {
            add_noise_remote_static_pubkey(&pubkeys[i]).unwrap();
            for key in pubkeys.iter().take(i + 1) {
                assert!(check_noise_remote_static_pubkey(key));
            }
            for key in pubkeys.iter().skip(i + 1) {
                assert!(!check_noise_remote_static_pubkey(key));
            }
        }

        // Adding a sixth pubkey should evict the oldest one and keep the
        // latest five.
        add_noise_remote_static_pubkey(&pubkeys[5]).unwrap();
        assert!(!check_noise_remote_static_pubkey(&pubkeys[0]));
        for key in pubkeys.iter().skip(1) {
            assert!(check_noise_remote_static_pubkey(key));
        }

        // Adding an already stored pubkey is a no-op.
        add_noise_remote_static_pubkey(&pubkeys[5]).unwrap();
        assert!(!check_noise_remote_static_pubkey(&pubkeys[0]));
        for key in pubkeys.iter().skip(1) {
            assert!(check_noise_remote_static_pubkey(key));
        }
    }

    static RAND_FIXTURES: [[u8; 32]; 7] = [
        // salt root
        hex!("bdb9ca4975e59e1b61d9141c5e79688cba7b3989b52b782de2e7e49b07ec8fae"),
        // io_protection_key
        hex!("28309e5a2e3bcf4aac94c0e59010fa3492e10839efb5b66192ad18f66a80510b"),
        // io_protection_key_split
        hex!("ae5be44d8b71a6041a7e9733e55f8c88b79dd552107624e0a916c10d8755e04e"),
        // authorization_key
        hex!("62c741d9ce7832e856ec06f6351cefcd9e7c5ca607938abb709770a5f2dbebcb"),
        // authorization_key_split
        hex!("20742d5a582f1f25b6e9d1c1e8b1effb40cfac855667ea7f49968af7f7eb5c19"),
        // encryption_key
        hex!("ed183784cbd297f9c2c241d0dd7cd16d62366c44b833ddf2c012fb4b49e1e8f3"),
        // encryption_key_split
        hex!("19f60ee825e752150d308817348c0fa6b3fe4f604c85c17e2eb97ada604a476f"),
    ];
    static RAND_FIXTURE_INDEX: AtomicUsize = AtomicUsize::new(0);

    unsafe extern "C" fn memory_setup_rand_mock_test_functional(buf_out: *mut u8) {
        let index = RAND_FIXTURE_INDEX.fetch_add(1, Ordering::SeqCst);
        let fixture = RAND_FIXTURES.get(index).expect("unexpected RNG request");
        unsafe { ptr::copy_nonoverlapping(fixture.as_ptr(), buf_out, fixture.len()) };
    }

    unsafe extern "C" fn mcu_random_adapter(buf_out: *mut u8) {
        let slice = unsafe { slice::from_raw_parts_mut(buf_out, 32) };
        let array: &mut [u8; 32] = slice.try_into().unwrap();
        random::mcu_32_bytes(array);
    }

    #[test]
    fn test_memory_multisig() {
        mock_memory();

        let hashes: [[u8; 32]; 6] = [
            *b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            *b"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
            *b"cccccccccccccccccccccccccccccccc",
            *b"dddddddddddddddddddddddddddddddd",
            *b"eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee",
            *b"ffffffffffffffffffffffffffffffff",
        ];
        let names = ["name1", "name2", "name3", "name4", "name5", "name6"];

        assert!(multisig_get_by_hash(&hashes[0]).is_none());

        // set
        assert!(multisig_set_by_hash(&hashes[0], names[0]).is_ok());
        assert!(multisig_set_by_hash(&hashes[1], names[1]).is_ok());
        // overwrite with the same is possible
        assert!(multisig_set_by_hash(&hashes[1], names[1]).is_ok());

        // get
        assert!(multisig_get_by_hash(&hashes[0]).is_some());
        assert_eq!(multisig_get_by_hash(&hashes[0]).as_deref(), Some(names[0]));
        assert_eq!(multisig_get_by_hash(&hashes[1]).as_deref(), Some(names[1]));

        // rename
        let name0_renamed = "name 1 renamed";
        assert!(multisig_set_by_hash(&hashes[0], name0_renamed).is_ok());
        assert_eq!(
            multisig_get_by_hash(&hashes[0]).as_deref(),
            Some(name0_renamed)
        );

        // rename to a name which already exists fails (duplicate name).
        let err = multisig_set_by_hash(&hashes[0], names[1]).unwrap_err();
        assert_eq!(err, MemoryError::MEMORY_ERR_DUPLICATE_NAME);
        // was in fact not renamed
        assert_eq!(
            multisig_get_by_hash(&hashes[0]).as_deref(),
            Some(name0_renamed)
        );
    }

    #[test]
    fn test_memory_multisig_invalid() {
        mock_memory();

        // invalid hash
        let invalid_hash = [0xFFu8; 32];
        let err = multisig_set_by_hash(&invalid_hash, "foo").unwrap_err();
        assert_eq!(err, MemoryError::MEMORY_ERR_INVALID_INPUT);

        // invalid name
        let valid_hash = *b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        let err = multisig_set_by_hash(&valid_hash, "").unwrap_err();
        assert_eq!(err, MemoryError::MEMORY_ERR_INVALID_INPUT);
    }

    #[test]
    fn test_memory_multisig_full() {
        mock_memory();

        // Only 25 slots available.
        let limit = bitbox02_sys::MEMORY_MULTISIG_NUM_ENTRIES as usize;
        let mut hashes = vec![[0u8; 32]; limit + 1];
        let mut names: Vec<String> = Vec::with_capacity(limit + 1);
        for (i, hash) in hashes.iter_mut().enumerate() {
            hash.fill((i + i) as u8);
            names.push(format!("name{i}"));
        }

        for i in 0..limit {
            assert!(multisig_set_by_hash(&hashes[i], &names[i]).is_ok());
        }

        let err = multisig_set_by_hash(&hashes[limit], &names[limit]).unwrap_err();
        assert_eq!(err, MemoryError::MEMORY_ERR_FULL);
    }

    #[test]
    fn test_encrypted_seed_and_hmac_roundtrip() {
        mock_memory();

        assert!(!is_seeded());
        let seed_data: Vec<u8> = (0..96).map(|i| i as u8).collect();
        set_encrypted_seed_and_hmac(&seed_data).unwrap();
        assert!(is_seeded());

        let stored = get_encrypted_seed_and_hmac().unwrap();
        assert_eq!(stored, seed_data);

        let oversized = vec![0u8; 97];
        assert!(set_encrypted_seed_and_hmac(&oversized).is_err());
        assert_eq!(get_encrypted_seed_and_hmac().unwrap(), stored);
    }

    #[test]
    fn test_memory_attestation() {
        mock_memory();

        let expected_pubkey = [0x55u8; 64];
        let expected_certificate = [0x66u8; 64];
        let expected_root_pubkey_identifier = [0x77u8; 32];
        let mut pubkey = [0u8; 64];
        let mut certificate = [0u8; 64];
        let mut root_identifier = [0u8; 32];

        // Setup not done yet.
        assert!(
            get_attestation_pubkey_and_certificate(
                &mut pubkey,
                &mut certificate,
                &mut root_identifier
            )
            .is_err()
        );

        assert!(set_attestation_device_pubkey(&expected_pubkey));

        // Setup not done yet.
        assert!(
            get_attestation_pubkey_and_certificate(
                &mut pubkey,
                &mut certificate,
                &mut root_identifier
            )
            .is_err()
        );

        let wrong_pubkey = [0x11u8; 64];
        // Pubkey has to match the previously stored pubkey.
        assert!(!set_attestation_certificate(
            &wrong_pubkey,
            &expected_certificate,
            &expected_root_pubkey_identifier,
        ));

        assert!(set_attestation_certificate(
            &expected_pubkey,
            &expected_certificate,
            &expected_root_pubkey_identifier,
        ));

        // Setup done.
        get_attestation_pubkey_and_certificate(&mut pubkey, &mut certificate, &mut root_identifier)
            .unwrap();
        assert_eq!(pubkey, expected_pubkey);
        assert_eq!(certificate, expected_certificate);
        assert_eq!(root_identifier, expected_root_pubkey_identifier);
    }

    // Test a series of write/read operations.
    #[test]
    fn test_memory_setup_functional() {
        fake_memory_factoryreset();
        RAND_FIXTURE_INDEX.store(0, Ordering::SeqCst);
        assert!(memory_setup(memory_setup_rand_mock_test_functional));

        let mut io_protection_key = [0u8; 32];
        let mut authorization_key = [0u8; 32];
        let mut encryption_key = [0u8; 32];

        get_io_protection_key(&mut io_protection_key);
        get_authorization_key(&mut authorization_key);
        get_encryption_key(&mut encryption_key);

        let expected_io_protection_key =
            hex!("866b7a17a54a694eb6ea57d6754f76bc257cdd6bffc392813bbbd9fbedd5b145");
        let expected_authorization_key =
            hex!("42b36c8396572dcde005d737ddad0036deb3f02351f460c43901fa520530b7d2");
        let expected_encryption_key =
            hex!("f4ee396cee35c5eccff2c9c7e9f0decbd1c82324f4b61c8ceeab819129abaf9c");

        assert_eq!(io_protection_key, expected_io_protection_key);
        assert_eq!(authorization_key, expected_authorization_key);
        assert_eq!(encryption_key, expected_encryption_key);

        assert!(memory_setup(memory_setup_rand_mock_test_functional));

        // Run again, shouldn't do anything. Other operations modifying the same memory chunk
        // shouldn't change the secure chip keys.
        assert!(bootloader_set_flags(bitbox02_sys::secfalse_u8 as u8, true));

        get_io_protection_key(&mut io_protection_key);
        get_authorization_key(&mut authorization_key);
        get_encryption_key(&mut encryption_key);

        assert_eq!(io_protection_key, expected_io_protection_key);
        assert_eq!(authorization_key, expected_authorization_key);
        assert_eq!(encryption_key, expected_encryption_key);
    }

    #[test]
    fn test_attestation_bootloader_hash() {
        fake_memory_factoryreset();
        assert!(memory_setup(mcu_random_adapter));

        let mock1 = hex!("0322b3191aab5bc415c5bafac5333445175be2faa8333ac3abee4cd17e49082a");
        let mock2 = hex!("6cad6abc3fd447a58d7a262d7606a040e49e82b00648623625883e9fc0faa8ad");

        set_bootloader_hash_fake(&mock1);

        let hash = memory_bootloader_hash();
        assert_eq!(hash, mock1);
        assert_eq!(get_attestation_bootloader_hash(), mock1);

        assert!(set_attestation_bootloader_hash(&mock1));
        assert_eq!(get_attestation_bootloader_hash(), mock1);

        set_bootloader_hash_fake(&mock2);
        let hash = memory_bootloader_hash();
        assert_eq!(hash, mock2);

        assert_eq!(get_attestation_bootloader_hash(), mock1);

        reset_hww().unwrap();

        assert_eq!(get_attestation_bootloader_hash(), mock1);
    }
}
