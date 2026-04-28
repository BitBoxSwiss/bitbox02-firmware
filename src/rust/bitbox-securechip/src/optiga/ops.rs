// SPDX-License-Identifier: Apache-2.0

use crate::{Error, SecureChipError};
use alloc::{boxed::Box, vec::Vec};
use core::cell::UnsafeCell;
use core::future::poll_fn;
use core::task::{Poll, Waker};
use grounded::uninit::{GroundedArrayCell, GroundedCell};
use util::cell::SyncCell;
use zeroize::{Zeroize, Zeroizing};

const ARBITRARY_DATA_OBJECT_TYPE_3_MAX_SIZE: usize =
    bitbox_securechip_sys::ARBITRARY_DATA_OBJECT_TYPE_3_MAX_SIZE as usize;

// This is the biggest buffer we want to move through the async data-object helpers.
const ASYNC_BUF_MAX_SIZE: usize = ARBITRARY_DATA_OBJECT_TYPE_3_MAX_SIZE;

#[derive(Copy, Clone, Eq, PartialEq)]
enum AsyncOpState {
    // No async Rust wrapper owns the shared static buffers or the single global waker.
    Idle,
    // One live Rust future launched an Optiga command and still needs to observe the result.
    Running,
    // The Rust future was dropped after launching the command, but the C callback has not
    // completed yet. The old command may still be reading/writing the shared static buffers, so a
    // new call must not reuse them yet.
    Detached,
}

// The Optiga callback exposes only one shared status variable and one wakeup hook, so the Rust
// wrappers can support only one in-flight async operation at a time. The extra Detached state is
// needed because cancellation must not free the slot until the callback has really finished using
// the shared static buffers.
static STATE: SyncCell<AsyncOpState> = SyncCell::new(AsyncOpState::Idle);
static WAKER: WakerCell = WakerCell::new();

struct WakerCell {
    waker: UnsafeCell<Option<Waker>>,
}

unsafe impl Sync for WakerCell {}

impl WakerCell {
    const fn new() -> Self {
        Self {
            waker: UnsafeCell::new(None),
        }
    }

    fn register(&self, waker: &Waker) {
        critical_section::with(|_| unsafe {
            *self.waker.get() = Some(waker.clone());
        });
    }

    fn take(&self) -> Option<Waker> {
        critical_section::with(|_| unsafe { (*self.waker.get()).take() })
    }

    fn clear(&self) {
        drop(self.take());
    }
}

// The Optiga C API retains raw pointers to these statics until its async callback completes,
// potentially after the Rust future has been dropped. The surrounding async-op state machine
// ensures only one Rust wrapper owns the buffers at a time, so this wrapper exposes only raw
// pointers and byte-buffer operations and never hands out Rust references to the static storage.
struct StaticBytes<const N: usize>(GroundedArrayCell<u8, N>);

impl<const N: usize> StaticBytes<N> {
    const fn const_init() -> Self {
        Self(GroundedArrayCell::const_init())
    }

    fn as_mut_ptr(&self) -> *mut u8 {
        self.0.as_mut_ptr()
    }

    fn clear(&self) {
        unsafe {
            self.0.initialize_all_copied(0);
        }
    }

    fn copy_from_slice(&self, data: &[u8]) {
        unsafe {
            core::ptr::copy_nonoverlapping(data.as_ptr(), self.as_mut_ptr(), data.len());
        }
    }

    fn copy_to_slice(&self, out: &mut [u8]) {
        unsafe {
            core::ptr::copy_nonoverlapping(self.as_mut_ptr(), out.as_mut_ptr(), out.len());
        }
    }

    fn zeroize(&self) {
        let (ptr, len) = self.0.get_ptr_len();
        unsafe {
            core::slice::from_raw_parts_mut(ptr, len).zeroize();
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_optiga_callback_wake() {
    if let Some(waker) = WAKER.take() {
        waker.wake();
    }
}

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

        WAKER.clear();
        let status = unsafe { bitbox_securechip_sys::optiga_ops_get_status() };
        if status == bitbox_securechip_sys::OPTIGA_LIB_BUSY as _ {
            STATE.write(AsyncOpState::Detached);
        } else {
            STATE.write(AsyncOpState::Idle);
        }
    }
}

async fn reclaim_detached_op() {
    match STATE.read() {
        AsyncOpState::Idle => {
            // Another path already observed the callback and released the slot, so there is
            // nothing left to reclaim here.
            return;
        }
        AsyncOpState::Running => {
            // Sequential callers are required for this wrapper. Reaching this arm means some
            // other live future is still using the shared static buffers and the global waker.
            panic!("concurrent async optiga operation not supported");
        }
        AsyncOpState::Detached => {
            // Detached means the old future is gone. Under the sequential-caller assumption,
            // this recovery future is now the only one that can wait for the late callback and
            // then release the shared static buffers for reuse.
            WAKER.clear();
        }
    }

    let mut guard = AsyncOpGuard::new();
    let _ = wait_until_not_busy().await;
    guard.disarm();
    WAKER.clear();
    STATE.write(AsyncOpState::Idle);
}

async fn begin_async_op() -> AsyncOpGuard {
    loop {
        match STATE.read() {
            AsyncOpState::Idle => {
                // We are the only Rust future touching the shared static buffers, so it is safe
                // to hand their addresses to the C library and reuse the global waker slot.
                STATE.write(AsyncOpState::Running);
                WAKER.clear();
                unsafe {
                    bitbox_securechip_sys::optiga_ops_set_status_busy();
                }
                return AsyncOpGuard::new();
            }
            AsyncOpState::Running => {
                // Sequential callers are required. If we are asked to start another operation
                // while one live future still owns the shared static buffers, something above this
                // wrapper broke that invariant.
                panic!("concurrent async optiga operation not supported");
            }
            AsyncOpState::Detached => {
                // A previous future was cancelled after launching the command. Wait until its
                // callback has really landed before reusing the shared static buffers.
                reclaim_detached_op().await;
            }
        }
    }
}

fn end_async_op() {
    WAKER.clear();
    STATE.write(AsyncOpState::Idle);
}

async fn wait_with_cleanup(
    mut guard: AsyncOpGuard,
    initial_status: bitbox_securechip_sys::optiga_lib_status_t,
) -> Result<(), bitbox_securechip_sys::optiga_lib_status_t> {
    let result = wait(initial_status).await;
    guard.disarm();
    end_async_op();
    result
}

async fn wait_until_not_busy() -> bitbox_securechip_sys::optiga_lib_status_t {
    poll_fn(|cx| {
        let status = unsafe { bitbox_securechip_sys::optiga_ops_get_status() };
        if status == bitbox_securechip_sys::OPTIGA_LIB_BUSY as _ {
            // Register first, then re-check status to avoid missing a callback that fires between
            // the initial busy check and storing the waker.
            WAKER.register(cx.waker());
            let status = unsafe { bitbox_securechip_sys::optiga_ops_get_status() };
            if status == bitbox_securechip_sys::OPTIGA_LIB_BUSY as _ {
                Poll::Pending
            } else {
                WAKER.clear();
                Poll::Ready(status)
            }
        } else {
            WAKER.clear();
            Poll::Ready(status)
        }
    })
    .await
}

async fn wait(
    initial_status: bitbox_securechip_sys::optiga_lib_status_t,
) -> Result<(), bitbox_securechip_sys::optiga_lib_status_t> {
    if initial_status != bitbox_securechip_sys::OPTIGA_LIB_SUCCESS as _ {
        return Err(initial_status);
    }

    match wait_until_not_busy().await {
        status if status == bitbox_securechip_sys::OPTIGA_LIB_SUCCESS as _ => Ok(()),
        status => Err(status),
    }
}

pub(super) async fn util_read_data(oid: u16, offset: u16, out: &mut [u8]) -> Result<(), Error> {
    // Static because the Optiga library keeps raw pointers to this buffer and length until the
    // async callback completes, and the Rust future may be dropped before that happens.
    static BUF: StaticBytes<ASYNC_BUF_MAX_SIZE> = StaticBytes::const_init();
    static LEN: GroundedCell<u16> = GroundedCell::const_init();
    if out.len() > ASYNC_BUF_MAX_SIZE {
        panic!("optiga async read larger than max supported size");
    }
    let requested_len: u16 = out.len().try_into().unwrap();

    let util = unsafe { bitbox_securechip_sys::optiga_util_instance() };

    let guard = begin_async_op().await;
    BUF.clear();
    unsafe {
        LEN.get().write(requested_len);
    }
    let result = wait_with_cleanup(guard, unsafe {
        bitbox_securechip_sys::optiga_util_read_data(util, oid, offset, BUF.as_mut_ptr(), LEN.get())
    })
    .await
    .map_err(|status| Error::from_status(status as i32));
    if let Err(err) = result {
        BUF.zeroize();
        return Err(err);
    }

    if unsafe { LEN.get().read() } != requested_len {
        BUF.zeroize();
        return Err(Error::SecureChip(
            SecureChipError::SC_OPTIGA_ERR_UNEXPECTED_LEN,
        ));
    }
    BUF.copy_to_slice(out);
    BUF.zeroize();
    Ok(())
}

pub(super) async fn crypt_hmac(
    hmac_type: bitbox_securechip_sys::optiga_hmac_type_t,
    secret: u16,
    msg: &[u8; super::KDF_LEN],
    mac_out: &mut [u8; super::KDF_LEN],
) -> Result<(), Error> {
    // Static because the Optiga library keeps raw pointers to the input, output and length until
    // the async callback completes, and the Rust future may be dropped before that happens.
    static INPUT: StaticBytes<{ super::KDF_LEN }> = StaticBytes::const_init();
    static MAC: StaticBytes<{ super::KDF_LEN }> = StaticBytes::const_init();
    static MAC_LEN: GroundedCell<u32> = GroundedCell::const_init();

    let crypt = unsafe { bitbox_securechip_sys::optiga_crypt_instance() };

    let guard = begin_async_op().await;
    INPUT.copy_from_slice(msg);
    MAC.clear();
    unsafe {
        MAC_LEN.get().write(super::KDF_LEN as u32);
    }
    let result = wait_with_cleanup(guard, unsafe {
        bitbox_securechip_sys::optiga_crypt_hmac(
            crypt,
            hmac_type,
            secret,
            INPUT.as_mut_ptr(),
            super::KDF_LEN as u32,
            MAC.as_mut_ptr(),
            MAC_LEN.get(),
        )
    })
    .await
    .map_err(|status| Error::from_status(status as i32));
    if let Err(err) = result {
        INPUT.zeroize();
        MAC.zeroize();
        return Err(err);
    }

    if unsafe { MAC_LEN.get().read() as usize } != super::KDF_LEN {
        INPUT.zeroize();
        MAC.zeroize();
        return Err(Error::SecureChip(
            SecureChipError::SC_OPTIGA_ERR_UNEXPECTED_LEN,
        ));
    }
    MAC.copy_to_slice(mac_out);
    INPUT.zeroize();
    MAC.zeroize();
    Ok(())
}

pub(super) async fn util_write_data(
    oid: u16,
    write_type: u8,
    offset: u16,
    buffer: &[u8],
) -> Result<(), Error> {
    // Static because the Optiga library keeps a raw pointer to the input buffer until the async
    // callback completes, and the Rust future may be dropped before that happens.
    static INPUT: StaticBytes<ASYNC_BUF_MAX_SIZE> = StaticBytes::const_init();

    if buffer.len() > ASYNC_BUF_MAX_SIZE {
        panic!("optiga async write larger than max supported size");
    }
    let input_len: u16 = buffer.len().try_into().unwrap();
    let util = unsafe { bitbox_securechip_sys::optiga_util_instance() };

    let guard = begin_async_op().await;
    INPUT.copy_from_slice(buffer);
    let result = wait_with_cleanup(guard, unsafe {
        bitbox_securechip_sys::optiga_util_write_data(
            util,
            oid,
            write_type,
            offset,
            INPUT.as_mut_ptr(),
            input_len,
        )
    })
    .await
    .map_err(|status| Error::from_status(status as i32));
    INPUT.zeroize();
    result
}

pub(super) async fn crypt_symmetric_encrypt(
    encryption_mode: bitbox_securechip_sys::optiga_symmetric_encryption_mode_t,
    symmetric_key_oid: bitbox_securechip_sys::optiga_key_id_t,
    plain_data: &[u8; super::KDF_LEN],
    encrypted_data: &mut [u8; 16],
) -> Result<(), Error> {
    // Static because the Optiga library keeps raw pointers to the input, output and length until
    // the async callback completes, and the Rust future may be dropped before that happens.
    static INPUT: StaticBytes<{ super::KDF_LEN }> = StaticBytes::const_init();
    static OUTPUT: StaticBytes<16> = StaticBytes::const_init();
    static OUTPUT_LEN: GroundedCell<u32> = GroundedCell::const_init();

    let crypt = unsafe { bitbox_securechip_sys::optiga_crypt_instance() };
    let input_len = super::KDF_LEN as u32;

    let guard = begin_async_op().await;
    INPUT.copy_from_slice(plain_data);
    OUTPUT.clear();
    unsafe {
        OUTPUT_LEN.get().write(16);
    }
    let result = wait_with_cleanup(guard, unsafe {
        bitbox_securechip_sys::optiga_crypt_symmetric_encrypt(
            crypt,
            encryption_mode,
            symmetric_key_oid,
            INPUT.as_mut_ptr(),
            input_len,
            core::ptr::null(),
            0,
            core::ptr::null(),
            0,
            OUTPUT.as_mut_ptr(),
            OUTPUT_LEN.get(),
        )
    })
    .await
    .map_err(|status| Error::from_status(status as i32));
    if let Err(err) = result {
        INPUT.zeroize();
        OUTPUT.zeroize();
        return Err(err);
    }

    if unsafe { OUTPUT_LEN.get().read() } != 16 {
        INPUT.zeroize();
        OUTPUT.zeroize();
        return Err(Error::SecureChip(
            SecureChipError::SC_OPTIGA_ERR_UNEXPECTED_LEN,
        ));
    }
    OUTPUT.copy_to_slice(encrypted_data);
    INPUT.zeroize();
    OUTPUT.zeroize();
    Ok(())
}

pub(super) async fn crypt_generate_auth_code(
    rng_type: bitbox_securechip_sys::optiga_rng_type_t,
    random_data: &mut [u8; 32],
) -> Result<(), Error> {
    // Static because the Optiga library keeps a raw pointer to the output buffer until the async
    // callback completes, and the Rust future may be dropped before that happens.
    static RANDOM: StaticBytes<32> = StaticBytes::const_init();

    let crypt = unsafe { bitbox_securechip_sys::optiga_crypt_instance() };

    let guard = begin_async_op().await;
    RANDOM.clear();
    let result = wait_with_cleanup(guard, unsafe {
        bitbox_securechip_sys::optiga_crypt_generate_auth_code(
            crypt,
            rng_type,
            core::ptr::null(),
            0,
            RANDOM.as_mut_ptr(),
            32,
        )
    })
    .await
    .map_err(|status| Error::from_status(status as i32));
    if let Err(err) = result {
        RANDOM.zeroize();
        return Err(err);
    }

    RANDOM.copy_to_slice(random_data);
    RANDOM.zeroize();
    Ok(())
}

pub(super) async fn crypt_ecdsa_sign(
    digest: &[u8; super::KDF_LEN],
    private_key: bitbox_securechip_sys::optiga_key_id_t,
) -> Result<Vec<u8>, Error> {
    const ECDSA_SIGNATURE_MAX_LEN: usize = 70;

    // Static because the Optiga library keeps raw pointers to the input, output and length until
    // the async callback completes, and the Rust future may be dropped before that happens.
    static DIGEST: StaticBytes<{ super::KDF_LEN }> = StaticBytes::const_init();
    static SIGNATURE: StaticBytes<ECDSA_SIGNATURE_MAX_LEN> = StaticBytes::const_init();
    static SIGNATURE_LEN: GroundedCell<u16> = GroundedCell::const_init();

    let crypt = unsafe { bitbox_securechip_sys::optiga_crypt_instance() };

    let guard = begin_async_op().await;
    DIGEST.copy_from_slice(digest);
    SIGNATURE.clear();
    unsafe {
        SIGNATURE_LEN.get().write(ECDSA_SIGNATURE_MAX_LEN as u16);
    }
    let result = wait_with_cleanup(guard, unsafe {
        bitbox_securechip_sys::optiga_crypt_ecdsa_sign(
            crypt,
            DIGEST.as_mut_ptr(),
            super::KDF_LEN as u8,
            private_key,
            SIGNATURE.as_mut_ptr(),
            SIGNATURE_LEN.get(),
        )
    })
    .await
    .map_err(|status| Error::from_status(status as i32));
    if let Err(err) = result {
        DIGEST.zeroize();
        SIGNATURE.zeroize();
        return Err(err);
    }

    let signature_len = unsafe { SIGNATURE_LEN.get().read() as usize };
    if signature_len > ECDSA_SIGNATURE_MAX_LEN {
        DIGEST.zeroize();
        SIGNATURE.zeroize();
        return Err(Error::SecureChip(
            SecureChipError::SC_OPTIGA_ERR_UNEXPECTED_LEN,
        ));
    }

    let mut signature = alloc::vec![0; signature_len];
    SIGNATURE.copy_to_slice(signature.as_mut_slice());
    DIGEST.zeroize();
    SIGNATURE.zeroize();
    Ok(signature)
}

pub(super) async fn crypt_hmac_verify(
    hmac_type: bitbox_securechip_sys::optiga_hmac_type_t,
    secret: u16,
    input_data: &[u8; super::KDF_LEN],
    hmac: &[u8; super::KDF_LEN],
) -> Result<(), Error> {
    // Static because the Optiga library keeps raw pointers to the input buffers until the async
    // callback completes, and the Rust future may be dropped before that happens.
    static INPUT: StaticBytes<{ super::KDF_LEN }> = StaticBytes::const_init();
    static HMAC: StaticBytes<{ super::KDF_LEN }> = StaticBytes::const_init();

    let crypt = unsafe { bitbox_securechip_sys::optiga_crypt_instance() };

    let guard = begin_async_op().await;
    INPUT.copy_from_slice(input_data);
    HMAC.copy_from_slice(hmac);
    let result = wait_with_cleanup(guard, unsafe {
        bitbox_securechip_sys::optiga_crypt_hmac_verify(
            crypt,
            hmac_type,
            secret,
            INPUT.as_mut_ptr(),
            super::KDF_LEN as u32,
            HMAC.as_mut_ptr(),
            super::KDF_LEN as u32,
        )
    })
    .await
    .map_err(|status| Error::from_status(status as i32));
    INPUT.zeroize();
    HMAC.zeroize();
    result
}

pub(super) async fn crypt_symmetric_generate_key(
    key_type: bitbox_securechip_sys::optiga_symmetric_key_type_t,
    key_usage: bitbox_securechip_sys::optiga_key_usage_t,
) -> Result<(), Error> {
    // Static because the Optiga library keeps a raw pointer to the key id output until the async
    // callback completes, and the Rust future may be dropped before that happens.
    static KEYID: GroundedCell<bitbox_securechip_sys::optiga_key_id_t> = GroundedCell::uninit();

    let crypt = unsafe { bitbox_securechip_sys::optiga_crypt_instance() };

    let guard = begin_async_op().await;
    unsafe {
        KEYID
            .get()
            .write(super::key_id_from_oid(super::OID_AES_SYMKEY));
    }
    wait_with_cleanup(guard, unsafe {
        bitbox_securechip_sys::optiga_crypt_symmetric_generate_key(
            crypt,
            key_type,
            key_usage as u8,
            0,
            KEYID.get().cast(),
        )
    })
    .await
    .map_err(|status| Error::from_status(status as i32))
}

pub(super) async fn crypt_clear_auto_state(secret: u16) -> Result<(), Error> {
    let crypt = unsafe { bitbox_securechip_sys::optiga_crypt_instance() };
    let guard = begin_async_op().await;
    wait_with_cleanup(guard, unsafe {
        bitbox_securechip_sys::optiga_crypt_clear_auto_state(crypt, secret)
    })
    .await
    .map_err(|status| Error::from_status(status as i32))
}

pub(super) async fn crypt_random(
    rng_type: bitbox_securechip_sys::optiga_rng_type_t,
    out: &mut [u8; 32],
) -> Result<(), Error> {
    // Static because the Optiga library keeps a raw pointer to this buffer until the async
    // callback completes, and the Rust future may be dropped before that happens.
    static BUF: StaticBytes<32> = StaticBytes::const_init();

    let crypt = unsafe { bitbox_securechip_sys::optiga_crypt_instance() };

    let guard = begin_async_op().await;
    BUF.clear();
    let result = wait_with_cleanup(guard, unsafe {
        bitbox_securechip_sys::optiga_crypt_random(crypt, rng_type, BUF.as_mut_ptr(), 32)
    })
    .await
    .map_err(|status| Error::from_status(status as i32));
    if let Err(err) = result {
        BUF.zeroize();
        return Err(err);
    }

    BUF.copy_to_slice(out);
    BUF.zeroize();
    Ok(())
}

pub(super) fn random_32_bytes(
    random: &mut impl bitbox_hal::Random,
    mixin: &[u8; super::KDF_LEN],
) -> Result<Box<Zeroizing<[u8; super::KDF_LEN]>>, Error> {
    Ok(bitbox_core_utils::random::random_32_bytes_with_mixin(
        random, mixin,
    ))
}
