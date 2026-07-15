// SPDX-License-Identifier: Apache-2.0

use bitbox_hal::timer::Timer;
use bitbox_securechip_sys::atecc_slot_t as Slot;
use core::time::Duration;
use util::cell::SyncCell;
use zeroize::Zeroizing;

#[cfg(not(test))]
#[path = "raw_ffi.rs"]
mod raw;
#[cfg(test)]
#[path = "raw_fake.rs"]
mod raw;
#[cfg(test)]
#[path = "testing.rs"]
pub(super) mod testing;

const ATECC_OPS_STATUS_BUSY: i32 = bitbox_securechip_sys::ATECC_OPS_STATUS_BUSY as i32;
pub(super) const NONCE_NUMIN_SIZE: usize = bitbox_securechip_sys::NONCE_NUMIN_SIZE as usize;
const BLOCK_SIZE: usize = 32;
const SIGNATURE_SIZE: usize = 64;

#[derive(Copy, Clone, Eq, PartialEq)]
enum AsyncOpState {
    // No async Rust wrapper owns the shared ATECC command context.
    Idle,
    // One live Rust future launched an ATECC command and still needs to observe the result.
    Running,
    // The Rust future was dropped after launching the command, but the C command has not
    // completed yet. The old command may still be using the shared command context, so a new call
    // must not reset it yet.
    Detached,
}

// The ATECC C API exposes only one shared async command context and status variable, so the Rust
// wrappers can support only one in-flight async operation at a time. The extra Detached state is
// needed because cancellation must not free the slot until polling has really observed completion
// of the old command.
static STATE: SyncCell<AsyncOpState> = SyncCell::new(AsyncOpState::Idle);

struct AsyncOpGuard {
    armed: bool,
}

impl AsyncOpGuard {
    fn new() -> Self {
        Self { armed: true }
    }

    fn disarm(&mut self) {
        self.armed = false;
    }
}

impl Drop for AsyncOpGuard {
    fn drop(&mut self) {
        if !self.armed {
            return;
        }

        if poll_status() == ATECC_OPS_STATUS_BUSY {
            STATE.write(AsyncOpState::Detached);
        } else {
            STATE.write(AsyncOpState::Idle);
        }
    }
}

fn poll_status() -> i32 {
    raw::status()
}

fn ensure_success(status: i32) -> Result<(), i32> {
    if status == 0 { Ok(()) } else { Err(status) }
}

pub(super) fn clear_io_temp_key() {
    raw::clear_io_temp_key();
}

async fn poll_once<T: Timer>() {
    let delay_ms = raw::poll_delay_ms() as u64;
    if delay_ms > 0 {
        T::delay_for(Duration::from_millis(delay_ms)).await;
    }
    raw::poll();
}

async fn reclaim_detached_op<T: Timer>() {
    match STATE.read() {
        AsyncOpState::Idle => {
            // Another path already observed completion and released the slot, so there is nothing
            // left to reclaim here.
            return;
        }
        AsyncOpState::Running => {
            // Sequential callers are required for this wrapper. Reaching this arm means some other
            // live future is still using the shared ATECC command context.
            panic!("concurrent async atecc operation not supported");
        }
        AsyncOpState::Detached => {
            // Detached means the old future is gone. Under the sequential-caller assumption, this
            // recovery future is now the only one that can wait until polling observes completion
            // and then release the shared command context for reuse.
        }
    }

    let mut guard = AsyncOpGuard::new();
    let _ = wait_until_not_busy::<T>().await;
    guard.disarm();
    STATE.write(AsyncOpState::Idle);
}

async fn begin_async_op<T: Timer>() -> AsyncOpGuard {
    loop {
        match STATE.read() {
            AsyncOpState::Idle => {
                // We are the only Rust future touching the shared ATECC command context. It is
                // safe to launch a new command unless the C status still reports a busy operation,
                // in which case a start helper would reset a command that may still be active.
                if poll_status() == ATECC_OPS_STATUS_BUSY {
                    STATE.write(AsyncOpState::Detached);
                    reclaim_detached_op::<T>().await;
                    continue;
                }
                STATE.write(AsyncOpState::Running);
                return AsyncOpGuard::new();
            }
            AsyncOpState::Running => {
                // Sequential callers are required. If we are asked to start another operation while
                // one live future still owns the shared command context, something above this
                // wrapper broke that invariant.
                panic!("concurrent async atecc operation not supported");
            }
            AsyncOpState::Detached => {
                // A previous future was cancelled after launching the command. Wait until polling
                // has really observed completion before reusing the shared command context.
                reclaim_detached_op::<T>().await;
            }
        }
    }
}

fn end_async_op() {
    STATE.write(AsyncOpState::Idle);
}

async fn wait_with_cleanup<T: Timer>(
    mut guard: AsyncOpGuard,
    initial_status: i32,
) -> Result<(), i32> {
    let result = wait::<T>(initial_status).await;
    guard.disarm();
    end_async_op();
    result
}

async fn wait_until_not_busy<T: Timer>() -> i32 {
    loop {
        match poll_status() {
            ATECC_OPS_STATUS_BUSY => poll_once::<T>().await,
            status => return status,
        }
    }
}

async fn wait<T: Timer>(initial_status: i32) -> Result<(), i32> {
    if initial_status != 0 {
        return Err(initial_status);
    }

    match wait_until_not_busy::<T>().await {
        0 => Ok(()),
        status => Err(status),
    }
}

pub(super) async fn chip_nonce_rand<T: Timer>(
    num_in: &[u8; NONCE_NUMIN_SIZE],
) -> Result<Zeroizing<[u8; 32]>, i32> {
    let mut rand_out = Zeroizing::new([0u8; 32]);
    let guard = begin_async_op::<T>().await;
    wait_with_cleanup::<T>(guard, raw::start_nonce_rand(num_in)).await?;
    ensure_success(raw::read_random_response(&mut rand_out))?;
    Ok(rand_out)
}

pub(super) async fn chip_checkmac<T: Timer>(response: &Zeroizing<[u8; 32]>) -> Result<(), i32> {
    let guard = begin_async_op::<T>().await;
    wait_with_cleanup::<T>(guard, raw::start_checkmac(response)).await
}

pub(super) async fn chip_random<T: Timer>() -> Result<Zeroizing<[u8; 32]>, i32> {
    let mut rand_out = Zeroizing::new([0u8; 32]);
    let guard = begin_async_op::<T>().await;
    wait_with_cleanup::<T>(guard, raw::start_random()).await?;
    ensure_success(raw::read_random_response(&mut rand_out))?;
    Ok(rand_out)
}

pub(super) async fn chip_counter_read<T: Timer>() -> Result<u32, i32> {
    let mut counter = 0u32;
    let guard = begin_async_op::<T>().await;
    wait_with_cleanup::<T>(guard, raw::start_counter_read()).await?;
    ensure_success(raw::read_counter_response(&mut counter))?;
    Ok(counter)
}

pub(super) async fn chip_info_revision<T: Timer>() -> Result<[u8; 4], i32> {
    let mut revision = [0u8; 4];
    let guard = begin_async_op::<T>().await;
    wait_with_cleanup::<T>(guard, raw::start_info_revision()).await?;
    ensure_success(raw::read_info_response(&mut revision))?;
    Ok(revision)
}

pub(super) async fn chip_kdf<T: Timer>(
    slot: Slot,
    msg: &[u8; 32],
) -> Result<(Zeroizing<[u8; 32]>, Zeroizing<[u8; 32]>), i32> {
    let mut kdf_out = Zeroizing::new([0u8; 32]);
    let mut nonce_out = Zeroizing::new([0u8; 32]);
    let guard = begin_async_op::<T>().await;
    wait_with_cleanup::<T>(guard, raw::start_kdf(slot, msg)).await?;
    ensure_success(raw::read_kdf_response(&mut kdf_out, &mut nonce_out))?;
    Ok((kdf_out, nonce_out))
}

pub(super) async fn chip_derivekey_rollkey<T: Timer>() -> Result<(), i32> {
    let guard = begin_async_op::<T>().await;
    wait_with_cleanup::<T>(guard, raw::start_derivekey_rollkey()).await
}

pub(super) async fn chip_nonce_load_msgdigest<T: Timer>(msg: &[u8; 32]) -> Result<(), i32> {
    let guard = begin_async_op::<T>().await;
    wait_with_cleanup::<T>(guard, raw::start_nonce_load_msgdigest(msg)).await
}

pub(super) async fn chip_sign_attestation<T: Timer>() -> Result<[u8; SIGNATURE_SIZE], i32> {
    let mut signature = [0u8; SIGNATURE_SIZE];
    let guard = begin_async_op::<T>().await;
    wait_with_cleanup::<T>(guard, raw::start_sign_attestation()).await?;
    ensure_success(raw::read_sign_response(&mut signature))?;
    Ok(signature)
}

pub(super) async fn chip_gendig_encryption_key<T: Timer>() -> Result<(), i32> {
    let guard = begin_async_op::<T>().await;
    wait_with_cleanup::<T>(guard, raw::start_gendig_encryption_key()).await
}

#[cfg(any(feature = "app-u2f", feature = "factory-setup"))]
pub(super) async fn chip_read_block<T: Timer>(
    slot: Slot,
    block: u8,
) -> Result<Zeroizing<[u8; BLOCK_SIZE]>, i32> {
    let mut data = Zeroizing::new([0u8; BLOCK_SIZE]);
    let guard = begin_async_op::<T>().await;
    wait_with_cleanup::<T>(guard, raw::start_read_block(slot, block)).await?;
    ensure_success(raw::read_block_response(&mut data))?;
    Ok(data)
}

pub(super) async fn chip_write_encrypted_block<T: Timer>(
    slot: Slot,
    block: u8,
    value: &Zeroizing<[u8; BLOCK_SIZE]>,
    mac: &Zeroizing<[u8; BLOCK_SIZE]>,
) -> Result<(), i32> {
    let guard = begin_async_op::<T>().await;
    wait_with_cleanup::<T>(
        guard,
        raw::start_write_encrypted_block(slot, block, value, mac),
    )
    .await
}

// The C helper first calculates the contents of TempKey with atcah_nonce(), then computes the
// client response with atcah_check_mac(). In this CheckMac mode, the first SHA block comes from
// the slot key and the second from TempKey, so Rust keeps those two host steps bundled here.
pub(super) fn host_check_mac(
    num_in: &[u8; NONCE_NUMIN_SIZE],
    rand_out: &Zeroizing<[u8; 32]>,
    auth_key: &[u8; 32],
) -> Result<Zeroizing<[u8; 32]>, i32> {
    let mut response = Zeroizing::new([0u8; 32]);
    ensure_success(raw::auth_compute_response(
        num_in,
        rand_out,
        auth_key,
        &mut response,
    ))?;
    Ok(response)
}

pub(super) fn host_kdf_decrypt(
    io_protection_key: &[u8; 32],
    nonce_out: &Zeroizing<[u8; 32]>,
    data: &mut Zeroizing<[u8; 32]>,
) -> Result<(), i32> {
    ensure_success(raw::kdf_decrypt(io_protection_key, nonce_out, data))
}

pub(super) fn host_nonce(
    num_in: &[u8; NONCE_NUMIN_SIZE],
    rand_out: &Zeroizing<[u8; 32]>,
) -> Result<(), i32> {
    ensure_success(raw::io_prepare_tempkey(num_in, rand_out))
}

pub(super) fn host_gendig(encryption_key: &[u8; 32]) -> Result<(), i32> {
    ensure_success(raw::io_apply_gendig(encryption_key))
}

pub(super) fn host_write_auth_mac(
    slot: Slot,
    block: u8,
    input_data: &Zeroizing<[u8; BLOCK_SIZE]>,
) -> Result<(Zeroizing<[u8; BLOCK_SIZE]>, Zeroizing<[u8; BLOCK_SIZE]>), i32> {
    let mut encrypted = Zeroizing::new([0u8; BLOCK_SIZE]);
    let mut mac = Zeroizing::new([0u8; BLOCK_SIZE]);
    ensure_success(raw::io_prepare_encrypted_write(
        slot,
        block,
        input_data,
        &mut encrypted,
        &mut mac,
    ))?;
    Ok((encrypted, mac))
}

#[cfg(any(feature = "app-u2f", feature = "factory-setup"))]
pub(super) fn host_io_decrypt(data: &mut Zeroizing<[u8; BLOCK_SIZE]>) -> Result<(), i32> {
    ensure_success(raw::io_decrypt_block(data))
}
