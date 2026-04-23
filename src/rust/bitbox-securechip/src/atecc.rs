// SPDX-License-Identifier: Apache-2.0

use crate::{Error, Model, PasswordStretchAlgo, SecureChipError};
use alloc::{boxed::Box, format, string::String};
use bitbox_hal::{Memory, Random, timer::Timer};
use bitbox_securechip_sys::atecc_slot_t as Slot;
use core::fmt::Write;
use core::sync::atomic::{AtomicBool, Ordering};
use util::sha2::hmac_sha256_overwrite;
use zeroize::Zeroizing;

#[cfg(not(test))]
#[path = "atecc/ops.rs"]
mod ops;
#[cfg(test)]
#[path = "atecc/ops_fake.rs"]
mod ops;

const ATECC_OPS_STATUS_BUSY: i32 = bitbox_securechip_sys::ATECC_OPS_STATUS_BUSY as i32;
const KDF_LEN: usize = 32;
// This number of KDF iterations on the 2nd kdf slot when stretching the device password.
const KDF_NUM_ITERATIONS: usize = 2;
const RANDOM_RETRIES: usize = 5;
// ATECC encrypted read/write operates on 32-byte blocks. These helpers only support the standard
// full block size.
#[cfg(any(feature = "app-u2f", feature = "factory-setup"))]
const BLOCK_SIZE: usize = 32;
const COUNTER_MAX_VALUE: u32 = 2_097_151;
const SLOT_ROLLKEY: Slot = Slot::ATECC_SLOT_ROLLKEY;
const SLOT_KDF: Slot = Slot::ATECC_SLOT_KDF;
#[cfg(any(feature = "app-u2f", feature = "factory-setup"))]
const SLOT_DATA0: Slot = Slot::ATECC_SLOT_DATA0;

// Several ATECC flows derive TempKey-like state across multiple chip commands and host helpers, so
// public operations must not interleave even though the low-level async machine is per-command.
static OP_IN_FLIGHT: AtomicBool = AtomicBool::new(false);

struct OpGuard;

impl Drop for OpGuard {
    fn drop(&mut self) {
        OP_IN_FLIGHT.store(false, Ordering::Release);
    }
}

// Some ATECC operations derive host/device state across multiple awaited chip commands, e.g.
// authorize+derivekey, nonce+gendig+encrypted read/write, or nonce-load+sign. Those sequences must
// not interleave with another high-level ATECC call, even though `ops.rs` only serializes one
// low-level command at a time.
fn begin_high_level_op() -> Result<OpGuard, Error> {
    if OP_IN_FLIGHT.swap(true, Ordering::AcqRel) {
        Err(Error::from_status(ATECC_OPS_STATUS_BUSY))
    } else {
        Ok(OpGuard)
    }
}

fn zeroed_secret<const N: usize>() -> Box<Zeroizing<[u8; N]>> {
    Box::new(Zeroizing::new([0; N]))
}

fn status_from_error(err: Error) -> i32 {
    match err {
        Error::SecureChip(err) => err as i32,
        Error::Status(status) => status,
    }
}

async fn random_32_bytes<T: Timer>(
    random: &mut impl Random,
) -> Result<Box<Zeroizing<[u8; 32]>>, i32> {
    let securechip_random = random_inner::<T>().await.map_err(status_from_error)?;
    Ok(bitbox_core_utils::random::random_32_bytes_with_mixin(
        random,
        securechip_random.as_ref(),
    ))
}

fn load_auth_key(memory: &mut impl Memory) -> Box<Zeroizing<[u8; 32]>> {
    let mut auth_key = zeroed_secret();
    memory.get_auth_key(auth_key.as_mut());
    auth_key
}

fn load_io_protection_key(memory: &mut impl Memory) -> Box<Zeroizing<[u8; 32]>> {
    let mut io_protection_key = zeroed_secret();
    memory.get_io_protection_key(io_protection_key.as_mut());
    io_protection_key
}

fn load_encryption_key(memory: &mut impl Memory) -> Box<Zeroizing<[u8; 32]>> {
    let mut encryption_key = zeroed_secret();
    memory.get_encryption_key(encryption_key.as_mut());
    encryption_key
}

/**
 * This performs the CheckMac command on ATECC_SLOT_AUTHKEY. This needs to
 * be called before using any slot requiring auth and whose KeyConfig.AuthKey is
 * ATECC_SLOT_AUTHKEY.
 */
async fn authorize_key<T: Timer>(memory: &mut impl Memory) -> Result<(), i32> {
    let num_in = Zeroizing::new([0u8; ops::NONCE_NUMIN_SIZE]);
    let rand_out = ops::chip_nonce_rand::<T>(&num_in).await?;
    let auth_key = load_auth_key(memory);
    let response = ops::host_check_mac(&num_in, &rand_out, auth_key.as_ref()).await?;
    ops::chip_checkmac::<T>(&response).await
}

/**
 * Performs a roll-key operation on a ATECC_SLOT_ROLLKEY.
 * @return ATCA_SUCCESS on success.
 */
async fn rollkey<T: Timer>(memory: &mut impl Memory) -> Result<(), i32> {
    authorize_key::<T>(memory).await?;

    let num_in = Zeroizing::new([0u8; ops::NONCE_NUMIN_SIZE]);
    let _rand_out = ops::chip_nonce_rand::<T>(&num_in).await?;
    ops::chip_derivekey_rollkey::<T>().await
}

/**
 * Writes a new random key to ATECC_SLOT_KDF.
 * @return ATCA_SUCCESS on success.
 */
async fn update_kdf_key<T: Timer>(
    random: &mut impl Random,
    memory: &mut impl Memory,
) -> Result<(), i32> {
    let new_key = random_32_bytes::<T>(random).await?;
    let nonce_contribution = random_32_bytes::<T>(random).await?;

    authorize_key::<T>(memory).await?;

    let mut num_in = Zeroizing::new([0u8; ops::NONCE_NUMIN_SIZE]);
    (*num_in).copy_from_slice(&nonce_contribution[..ops::NONCE_NUMIN_SIZE]);
    let rand_out = ops::chip_nonce_rand::<T>(&num_in).await?;
    ops::host_nonce(&num_in, &rand_out).await?;
    ops::chip_gendig_encryption_key::<T>().await?;
    let encryption_key = load_encryption_key(memory);
    ops::host_gendig(encryption_key.as_ref()).await?;

    let (cipher, mac) = ops::host_write_auth_mac(SLOT_KDF, 0, new_key.as_ref()).await?;
    ops::chip_write_encrypted_block::<T>(SLOT_KDF, 0, &cipher, &mac).await
}

async fn atecc_kdf<T: Timer>(
    memory: &mut impl Memory,
    slot: Slot,
    msg: &[u8; 32],
) -> Result<Box<Zeroizing<[u8; 32]>>, Error> {
    authorize_key::<T>(memory)
        .await
        .map_err(Error::from_status)?;

    // The result is hkdf_extract with the msg as ikm (input key material) and
    // the slot key as the salt. hkdf info does not apply, as it is part of
    // hkdf_expand, which is not performed. hkdf_extract is simply hmac.
    // Python equivalent:
    // import hmac, hashlib; hmac.new(slot_key, msg, hashlib.sha256).digest()
    let (mut kdf_out, nonce_out) = ops::chip_kdf::<T>(slot, msg)
        .await
        .map_err(Error::from_status)?;

    // The chip output is encrypted with the io protection key.
    let io_protection_key = load_io_protection_key(memory);
    ops::host_kdf_decrypt(io_protection_key.as_ref(), &nonce_out, &mut kdf_out)
        .await
        .map_err(Error::from_status)?;
    Ok(Box::new(kdf_out))
}

// Read a "standard" sized block from a data slot (must be 32 bytes)
#[cfg(any(feature = "app-u2f", feature = "factory-setup"))]
async fn read_data_slot_block<T: Timer>(
    random: &mut impl Random,
    memory: &mut impl Memory,
    slot: Slot,
    block: u8,
) -> Result<Zeroizing<[u8; BLOCK_SIZE]>, i32> {
    let nonce_contribution = random_32_bytes::<T>(random).await?;

    authorize_key::<T>(memory).await?;

    let mut num_in = Zeroizing::new([0u8; ops::NONCE_NUMIN_SIZE]);
    (*num_in).copy_from_slice(&nonce_contribution[..ops::NONCE_NUMIN_SIZE]);
    let rand_out = ops::chip_nonce_rand::<T>(&num_in).await?;
    ops::host_nonce(&num_in, &rand_out).await?;
    ops::chip_gendig_encryption_key::<T>().await?;
    let encryption_key = load_encryption_key(memory);
    ops::host_gendig(encryption_key.as_ref()).await?;

    let mut data = ops::chip_read_block::<T>(slot, block).await?;
    ops::host_io_decrypt(&mut data).await?;
    Ok(data)
}

// Write a "standard" sized block from a data slot (must be 32 bytes)
#[cfg(any(feature = "app-u2f", feature = "factory-setup"))]
async fn write_data_slot_block<T: Timer>(
    random: &mut impl Random,
    memory: &mut impl Memory,
    bytes: &Zeroizing<[u8; BLOCK_SIZE]>,
    slot: Slot,
    block: u8,
) -> Result<(), i32> {
    let nonce_contribution = random_32_bytes::<T>(random).await?;

    authorize_key::<T>(memory).await?;

    let mut num_in = Zeroizing::new([0u8; ops::NONCE_NUMIN_SIZE]);
    (*num_in).copy_from_slice(&nonce_contribution[..ops::NONCE_NUMIN_SIZE]);
    let rand_out = ops::chip_nonce_rand::<T>(&num_in).await?;
    ops::host_nonce(&num_in, &rand_out).await?;
    ops::chip_gendig_encryption_key::<T>().await?;
    let encryption_key = load_encryption_key(memory);
    ops::host_gendig(encryption_key.as_ref()).await?;

    let (cipher, mac) = ops::host_write_auth_mac(slot, block, bytes).await?;
    ops::chip_write_encrypted_block::<T>(slot, block, &cipher, &mac).await?;

    // Double-check by reading it back and comparing.
    let written_bytes = read_data_slot_block::<T>(random, memory, slot, block).await?;
    if written_bytes[..BLOCK_SIZE] == bytes[..BLOCK_SIZE] {
        Ok(())
    } else {
        Err(-1)
    }
}

async fn random_inner<T: Timer>() -> Result<Box<Zeroizing<[u8; 32]>>, Error> {
    let mut last_err = 0;
    for _ in 0..RANDOM_RETRIES {
        match ops::chip_random::<T>().await {
            Ok(random) => return Ok(Box::new(random)),
            Err(err) => last_err = err,
        }
    }
    Err(Error::from_status(last_err))
}

async fn reset_keys_inner<T: Timer>(
    random: &mut impl Random,
    memory: &mut impl Memory,
) -> Result<(), ()> {
    rollkey::<T>(memory).await.map_err(|_| ())?;
    update_kdf_key::<T>(random, memory).await.map_err(|_| ())
}

async fn stretch_password_inner<T: Timer>(
    memory: &mut impl Memory,
    password: &str,
) -> Result<Box<Zeroizing<[u8; 32]>>, Error> {
    let password_salted_hashed =
        bitbox_core_utils::salt::hash_data(memory, password.as_bytes(), "keystore_seed_access_in")
            .map_err(|_| Error::SecureChip(SecureChipError::SC_ERR_SALT))?;

    let mut kdf_in = zeroed_secret::<KDF_LEN>();
    (*kdf_in).copy_from_slice(password_salted_hashed.as_slice());

    // First KDF on rollkey increments the monotonic counter. Call only once!
    let mut stretched = atecc_kdf::<T>(memory, SLOT_ROLLKEY, kdf_in.as_ref()).await?;
    // Second KDF does not use the counter and we call it multiple times.
    for _ in 0..KDF_NUM_ITERATIONS {
        (*kdf_in).copy_from_slice(stretched.as_slice());
        stretched = atecc_kdf::<T>(memory, SLOT_KDF, kdf_in.as_ref()).await?;
    }

    let password_salted_hashed =
        bitbox_core_utils::salt::hash_data(memory, password.as_bytes(), "keystore_seed_access_out")
            .map_err(|_| Error::SecureChip(SecureChipError::SC_ERR_SALT))?;
    hmac_sha256_overwrite(password_salted_hashed.as_slice(), &mut stretched);
    Ok(stretched)
}

pub async fn attestation_sign<T: Timer>(
    memory: &mut impl Memory,
    challenge: &[u8; 32],
    signature: &mut [u8; 64],
) -> Result<(), ()> {
    let _guard = begin_high_level_op().map_err(|_| ())?;
    authorize_key::<T>(memory).await.map_err(|_| ())?;

    // Keep the device RNG seeded before loading the external digest, matching calib_sign().
    let _throwaway = ops::chip_random::<T>().await.map_err(|_| ())?;
    ops::chip_nonce_load_msgdigest::<T>(challenge)
        .await
        .map_err(|_| ())?;
    let chip_signature = ops::chip_sign_attestation::<T>().await.map_err(|_| ())?;
    signature.copy_from_slice(&chip_signature);
    Ok(())
}

pub async fn random<T: Timer>() -> Result<Box<Zeroizing<[u8; 32]>>, Error> {
    let _guard = begin_high_level_op()?;
    random_inner::<T>().await
}

pub async fn monotonic_increments_remaining<T: Timer>() -> Result<u32, ()> {
    let _guard = begin_high_level_op().map_err(|_| ())?;
    let counter = ops::chip_counter_read::<T>().await.map_err(|_| ())?;
    if COUNTER_MAX_VALUE < counter {
        panic!("ATECC returned an invalid value");
    }
    Ok(COUNTER_MAX_VALUE.wrapping_sub(counter))
}

pub async fn reset_keys<T: Timer>(
    random: &mut impl Random,
    memory: &mut impl Memory,
) -> Result<(), ()> {
    let _guard = begin_high_level_op().map_err(|_| ())?;
    reset_keys_inner::<T>(random, memory).await
}

pub async fn init_new_password<T: Timer>(
    random: &mut impl Random,
    memory: &mut impl Memory,
    password: &str,
    password_stretch_algo: PasswordStretchAlgo,
) -> Result<Box<Zeroizing<[u8; 32]>>, Error> {
    if password_stretch_algo != PasswordStretchAlgo::SECURECHIP_PASSWORD_STRETCH_ALGO_V0 {
        return Err(Error::SecureChip(
            SecureChipError::SC_ERR_INVALID_PASSWORD_STRETCH_ALGO,
        ));
    }

    let _guard = begin_high_level_op()?;
    reset_keys_inner::<T>(random, memory)
        .await
        .map_err(|_| Error::SecureChip(SecureChipError::SC_ATECC_ERR_RESET_KEYS))?;
    stretch_password_inner::<T>(memory, password).await
}

pub async fn stretch_password<T: Timer>(
    memory: &mut impl Memory,
    password: &str,
    password_stretch_algo: PasswordStretchAlgo,
) -> Result<Box<Zeroizing<[u8; 32]>>, Error> {
    if password_stretch_algo != PasswordStretchAlgo::SECURECHIP_PASSWORD_STRETCH_ALGO_V0 {
        return Err(Error::SecureChip(
            SecureChipError::SC_ERR_INVALID_PASSWORD_STRETCH_ALGO,
        ));
    }

    let _guard = begin_high_level_op()?;
    stretch_password_inner::<T>(memory, password).await
}

pub async fn kdf<T: Timer>(
    memory: &mut impl Memory,
    msg: &[u8; 32],
) -> Result<Box<Zeroizing<[u8; 32]>>, Error> {
    let _guard = begin_high_level_op()?;
    atecc_kdf::<T>(memory, SLOT_KDF, msg).await
}

#[cfg(any(feature = "app-u2f", feature = "factory-setup"))]
pub async fn u2f_counter_set<T: Timer>(
    random: &mut impl Random,
    memory: &mut impl Memory,
    counter: u32,
) -> Result<(), ()> {
    let _guard = begin_high_level_op().map_err(|_| ())?;
    let mut data = read_data_slot_block::<T>(random, memory, SLOT_DATA0, 0)
        .await
        .map_err(|_| ())?;
    (*data)[..4].copy_from_slice(&counter.to_le_bytes());
    write_data_slot_block::<T>(random, memory, &data, SLOT_DATA0, 0)
        .await
        .map_err(|_| ())
}

#[cfg(feature = "app-u2f")]
pub async fn u2f_counter_inc<T: Timer>(
    random: &mut impl Random,
    memory: &mut impl Memory,
) -> Result<u32, ()> {
    let _guard = begin_high_level_op().map_err(|_| ())?;
    let mut data = read_data_slot_block::<T>(random, memory, SLOT_DATA0, 0)
        .await
        .map_err(|_| ())?;
    let current = u32::from_le_bytes(data[..4].try_into().unwrap()).wrapping_add(1);
    (*data)[..4].copy_from_slice(&current.to_le_bytes());
    write_data_slot_block::<T>(random, memory, &data, SLOT_DATA0, 0)
        .await
        .map_err(|_| ())?;
    Ok(current)
}

pub async fn model<T: Timer>() -> Result<Model, ()> {
    let _guard = begin_high_level_op().map_err(|_| ())?;
    let revision = ops::chip_info_revision::<T>().await.map_err(|_| ())?;
    Ok(if revision[3] >= 0x03 {
        Model::ATECC_ATECC608B
    } else {
        Model::ATECC_ATECC608A
    })
}

fn manual_selftest_print_hex(print: &mut impl FnMut(&str), label: &str, bytes: &[u8]) {
    for (chunk_index, chunk) in bytes.chunks(32).enumerate() {
        let mut msg = String::new();
        if bytes.len() <= 32 {
            writeln!(&mut msg, "{label} ok").unwrap();
        } else {
            writeln!(&mut msg, "{label}[{chunk_index}] ok").unwrap();
        }
        for byte in chunk {
            write!(&mut msg, "{byte:02x}").unwrap();
        }
        print(&msg);
    }
}

fn manual_selftest_print_unit(print: &mut impl FnMut(&str), label: &str, result: Result<(), ()>) {
    match result {
        Ok(()) => print(&format!("{label}\nok")),
        Err(()) => print(&format!("{label}\nerr")),
    }
}

fn manual_selftest_model_name(model: Model) -> &'static str {
    match model {
        Model::ATECC_ATECC608A => "ATECC608A",
        Model::ATECC_ATECC608B => "ATECC608B",
        Model::OPTIGA_TRUST_M_V3 => "optiga?",
    }
}

fn manual_selftest_generate_attestation_key(
    memory: &mut impl Memory,
    print: &mut impl FnMut(&str),
) -> Option<[u8; 64]> {
    let auth_key = load_auth_key(memory);
    let mut pubkey = [0u8; 64];
    if unsafe {
        bitbox_securechip_sys::atecc_gen_attestation_key(auth_key.as_ptr(), pubkey.as_mut_ptr())
    } {
        manual_selftest_print_hex(print, "attest_pubkey", &pubkey);
        Some(pubkey)
    } else {
        print("attest_gen_key\nerr");
        None
    }
}

fn manual_selftest_verify_attestation_signature(
    pubkey: &[u8; 64],
    challenge: &[u8; 32],
    signature: &[u8; 64],
) -> Result<(), ()> {
    use p256::ecdsa::signature::hazmat::PrehashVerifier;

    let mut sec1_pubkey = [0u8; 65];
    sec1_pubkey[0] = 0x04;
    sec1_pubkey[1..].copy_from_slice(pubkey);
    let verifying_key = p256::ecdsa::VerifyingKey::from_sec1_bytes(&sec1_pubkey).map_err(|_| ())?;
    let signature = p256::ecdsa::Signature::from_slice(signature).map_err(|_| ())?;
    verifying_key
        .verify_prehash(challenge, &signature)
        .map_err(|_| ())
}

/// Runs a manual public ATECC API smoke-test and reports every result through `print`.
///
/// This is intended for temporary on-device diagnostics. The key-reset and password-init steps
/// mutate the secure-chip password-stretching keys, and the U2F steps mutate the U2F counter.
pub async fn manual_selftest<T: Timer>(
    random: &mut impl Random,
    memory: &mut impl Memory,
    mut print: impl FnMut(&str),
) {
    print("ATECC self-test\nstart");

    match model::<T>().await {
        Ok(model) => print(&format!("model ok\n{}", manual_selftest_model_name(model))),
        Err(()) => print("model\nerr"),
    }

    match self::random::<T>().await {
        Ok(random) => manual_selftest_print_hex(&mut print, "random", random.as_slice()),
        Err(err) => print(&format!("random\nerr {}", status_from_error(err))),
    }

    match monotonic_increments_remaining::<T>().await {
        Ok(remaining) => print(&format!("mono_remaining ok\n{remaining}")),
        Err(()) => print("mono_remaining\nerr"),
    }

    let kdf_msg = [0x11; 32];
    manual_selftest_print_hex(&mut print, "kdf_msg", &kdf_msg);
    match kdf::<T>(memory, &kdf_msg).await {
        Ok(kdf_out) => manual_selftest_print_hex(&mut print, "kdf", kdf_out.as_slice()),
        Err(err) => print(&format!("kdf\nerr {}", status_from_error(err))),
    }

    let challenge = [0x22; 32];
    let mut signature = [0u8; 64];
    if let Some(attestation_pubkey) = manual_selftest_generate_attestation_key(memory, &mut print) {
        manual_selftest_print_hex(&mut print, "attest_msg", &challenge);
        match attestation_sign::<T>(memory, &challenge, &mut signature).await {
            Ok(()) => {
                manual_selftest_print_hex(&mut print, "attest_sig", &signature);
                manual_selftest_print_unit(
                    &mut print,
                    "attest_verify",
                    manual_selftest_verify_attestation_signature(
                        &attestation_pubkey,
                        &challenge,
                        &signature,
                    ),
                );
            }
            Err(()) => print("attest_sign\nerr"),
        }
    }

    print("reset_keys\nmutates keys");
    manual_selftest_print_unit(
        &mut print,
        "reset_keys",
        reset_keys::<T>(random, memory).await,
    );

    let password = "atecc-selftest";
    print("init_new_password\nmutates keys");
    let initialized_password = match init_new_password::<T>(
        random,
        memory,
        password,
        PasswordStretchAlgo::SECURECHIP_PASSWORD_STRETCH_ALGO_V0,
    )
    .await
    {
        Ok(stretched) => {
            manual_selftest_print_hex(&mut print, "init_new_pw", stretched.as_slice());
            Some(stretched)
        }
        Err(err) => {
            print(&format!("init_new_pw\nerr {}", status_from_error(err)));
            None
        }
    };

    match stretch_password::<T>(
        memory,
        password,
        PasswordStretchAlgo::SECURECHIP_PASSWORD_STRETCH_ALGO_V0,
    )
    .await
    {
        Ok(stretched) => {
            manual_selftest_print_hex(&mut print, "stretch_pw", stretched.as_slice());
            if let Some(initialized_password) = initialized_password {
                if initialized_password.as_slice() == stretched.as_slice() {
                    print("stretch_cmp\nmatch");
                } else {
                    print("stretch_cmp\nmismatch");
                }
            }
        }
        Err(err) => print(&format!("stretch_pw\nerr {}", status_from_error(err))),
    }

    #[cfg(any(feature = "app-u2f", feature = "factory-setup"))]
    {
        print("u2f_counter_set\nmutates counter");
        manual_selftest_print_unit(
            &mut print,
            "u2f_counter_set",
            u2f_counter_set::<T>(random, memory, 1).await,
        );
    }

    #[cfg(feature = "app-u2f")]
    match u2f_counter_inc::<T>(random, memory).await {
        Ok(counter) => print(&format!("u2f_counter_inc ok\n{counter}")),
        Err(()) => print("u2f_counter_inc\nerr"),
    }

    #[cfg(not(any(feature = "app-u2f", feature = "factory-setup")))]
    {
        print("u2f_counter_set\nskip: feature off");
    }

    #[cfg(not(feature = "app-u2f"))]
    {
        print("u2f_counter_inc\nskip: feature off");
    }

    print("ATECC self-test\ndone");
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitbox_platform_host::memory::FakeMemory;
    use bitbox_platform_host::random::TestingRandom;
    use core::{task::Poll, time::Duration};
    use hex_lit::hex;

    const SALT_ROOT_FIXED: [u8; 32] = [0x42; 32];
    const EXPECTED_STRETCHED_OUT_V0: [u8; 32] =
        hex!("868f9f41fe3122cdcd0be180779401d7f68e1fa8744a5bbd4f3f117a316da1d5");

    struct TestTimer;

    impl Timer for TestTimer {
        async fn delay_for(_duration: Duration) {}
    }

    fn setup_test() -> (
        std::sync::MutexGuard<'static, ()>,
        FakeMemory,
        TestingRandom,
    ) {
        let guard = ops::test_lock();
        ops::test_reset();
        let mut memory = FakeMemory::new();
        memory.set_salt_root(&SALT_ROOT_FIXED);
        (guard, memory, TestingRandom::new())
    }

    #[async_test::test]
    #[allow(clippy::await_holding_lock)]
    async fn test_reset_keys() {
        let (_guard, mut memory, mut random) = setup_test();
        reset_keys::<TestTimer>(&mut random, &mut memory)
            .await
            .unwrap();
        assert_eq!(ops::test_get_derivekey_rollkey_calls(), 1);
        assert_eq!(ops::test_get_kdf_key_write_calls(), 1);
    }

    #[async_test::test]
    #[allow(clippy::await_holding_lock)]
    async fn test_stretch_password() {
        let (_guard, mut memory, _random) = setup_test();
        let stretched = stretch_password::<TestTimer>(
            &mut memory,
            "pw",
            PasswordStretchAlgo::SECURECHIP_PASSWORD_STRETCH_ALGO_V0,
        )
        .await
        .unwrap();
        assert_eq!(stretched.as_slice(), EXPECTED_STRETCHED_OUT_V0.as_slice(),);
        assert_eq!(ops::test_get_rollkey_kdf_calls(), 1);
        assert_eq!(ops::test_get_kdf_calls(), KDF_NUM_ITERATIONS);
    }

    #[test]
    fn test_random_busy() {
        let (_guard, _memory, _random) = setup_test();
        ops::test_block_next_chip_command();
        let mut first: util::bb02_async::Task<_> = alloc::boxed::Box::pin(random::<TestTimer>());
        assert!(matches!(util::bb02_async::spin(&mut first), Poll::Pending));

        assert_eq!(
            util::bb02_async::block_on(random::<TestTimer>()),
            Err(Error::Status(ATECC_OPS_STATUS_BUSY)),
        );

        ops::test_unblock_chip_command();
        drop(util::bb02_async::block_on(first).unwrap());
    }

    #[test]
    fn test_model_busy() {
        let (_guard, _memory, _random) = setup_test();
        ops::test_block_next_chip_command();
        let mut first: util::bb02_async::Task<_> = alloc::boxed::Box::pin(random::<TestTimer>());
        assert!(matches!(util::bb02_async::spin(&mut first), Poll::Pending));

        assert_eq!(util::bb02_async::block_on(model::<TestTimer>()), Err(()));

        ops::test_unblock_chip_command();
        drop(util::bb02_async::block_on(first).unwrap());
    }

    #[test]
    fn test_drop_releases_high_level_guard() {
        let (_guard, _memory, _random) = setup_test();
        ops::test_block_next_chip_command();
        let mut first: util::bb02_async::Task<_> = alloc::boxed::Box::pin(random::<TestTimer>());
        assert!(matches!(util::bb02_async::spin(&mut first), Poll::Pending));

        drop(first);

        assert!(util::bb02_async::block_on(random::<TestTimer>()).is_ok());
    }
}
