// SPDX-License-Identifier: Apache-2.0

#![no_std]

#[cfg(test)]
#[macro_use]
extern crate std;

#[macro_use]
mod c_alloc;
extern crate alloc;

#[cfg(feature = "firmware")]
pub mod async_usb;
#[cfg(any(
    feature = "firmware",
    all(feature = "bootloader", feature = "platform-bitbox02plus")
))]
mod communication_mode;
#[cfg(feature = "firmware")]
mod der;
#[cfg(feature = "firmware")]
mod firmware_c_api;
#[cfg(feature = "factory-setup")]
mod secp256k1;

#[cfg(feature = "app-u2f")]
// Stubs for C unit tests and C simulator - these are currently compiled and linked but they don't
// actually have to spawn/poll futures. The C simulator does not contain U2F, and the unit tests
// don't contain an executor.
#[cfg_attr(
    any(feature = "c-unit-testing", feature = "simulator-graphical"),
    path = "u2f_c_api_stubs.rs"
)]
mod u2f_c_api;

// Expose C interface defined in bitbox_aes
#[cfg(feature = "firmware")]
extern crate bitbox_aes;

// Expose C interface defined in bitbox02_rust
// Enable for firmware and for Nova bootloader (BitBox02 bootloader currently does not need it).
#[cfg(any(
    feature = "firmware",
    all(feature = "bootloader", feature = "platform-bitbox02plus")
))]
extern crate bitbox02_rust;

// Expose C interface defined in bitbox-framed-serial-link
extern crate bitbox_framed_serial_link;

// Expose C interface defined in bitbox-bytequeue
extern crate bitbox_bytequeue;
// Expose C interface defined in bitbox-da14531
extern crate bitbox_da14531;

// Expose C interface defined in util
extern crate util;

#[cfg(any(
    feature = "firmware",
    all(feature = "bootloader", feature = "platform-bitbox02plus")
))]
use core::ops::{Deref, DerefMut};
#[cfg(any(
    feature = "firmware",
    all(feature = "bootloader", feature = "platform-bitbox02plus")
))]
use core::sync::atomic::{AtomicBool, AtomicPtr, Ordering};
#[cfg(any(
    feature = "firmware",
    all(feature = "bootloader", feature = "platform-bitbox02plus")
))]
type HalImpl = bitbox02::hal::BitBox02Hal;

pub const BITBOX02_HAL_STORAGE_SIZE: usize = 1;

#[cfg(any(
    feature = "firmware",
    all(feature = "bootloader", feature = "platform-bitbox02plus")
))]
const _: () = assert!(core::mem::size_of::<HalImpl>() == 0);

#[cfg(any(
    feature = "firmware",
    all(feature = "bootloader", feature = "platform-bitbox02plus")
))]
const _: () = assert!(core::mem::align_of::<HalImpl>() == 1);

#[repr(C)]
pub struct BitBox02HAL {
    pub storage: [u8; BITBOX02_HAL_STORAGE_SIZE],
}

#[cfg(any(
    feature = "firmware",
    all(feature = "bootloader", feature = "platform-bitbox02plus")
))]
static BITBOX02_HAL: AtomicPtr<BitBox02HAL> = AtomicPtr::new(core::ptr::null_mut());

#[cfg(any(
    feature = "firmware",
    all(feature = "bootloader", feature = "platform-bitbox02plus")
))]
static BITBOX02_HAL_IN_USE: AtomicBool = AtomicBool::new(false);

#[cfg(any(
    feature = "firmware",
    all(feature = "bootloader", feature = "platform-bitbox02plus")
))]
struct HalGuard {
    hal: *mut BitBox02HAL,
}

#[cfg(any(
    feature = "firmware",
    all(feature = "bootloader", feature = "platform-bitbox02plus")
))]
impl Deref for HalGuard {
    type Target = HalImpl;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.hal.cast::<HalImpl>() }
    }
}

#[cfg(any(
    feature = "firmware",
    all(feature = "bootloader", feature = "platform-bitbox02plus")
))]
impl DerefMut for HalGuard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.hal.cast::<HalImpl>() }
    }
}

#[cfg(any(
    feature = "firmware",
    all(feature = "bootloader", feature = "platform-bitbox02plus")
))]
impl Drop for HalGuard {
    fn drop(&mut self) {
        BITBOX02_HAL_IN_USE.store(false, Ordering::Relaxed);
    }
}

#[cfg(any(
    feature = "firmware",
    all(feature = "bootloader", feature = "platform-bitbox02plus")
))]
impl bitbox_hal::Hal for HalGuard {
    type Ui = <HalImpl as bitbox_hal::Hal>::Ui;
    type Random = <HalImpl as bitbox_hal::Hal>::Random;
    type Sd = <HalImpl as bitbox_hal::Hal>::Sd;
    type SecureChip = <HalImpl as bitbox_hal::Hal>::SecureChip;
    type Memory = <HalImpl as bitbox_hal::Hal>::Memory;
    type System = <HalImpl as bitbox_hal::Hal>::System;

    fn as_mut(
        &mut self,
    ) -> bitbox_hal::HalSubsystems<
        '_,
        Self::Ui,
        Self::Random,
        Self::Sd,
        Self::SecureChip,
        Self::Memory,
        Self::System,
    > {
        bitbox_hal::Hal::as_mut(&mut **self)
    }
}

#[cfg(any(
    feature = "firmware",
    all(feature = "bootloader", feature = "platform-bitbox02plus")
))]
#[unsafe(no_mangle)]
/// # Safety
///
/// `hal` must be a valid, non-null pointer to writable `BitBox02HAL` storage.
pub unsafe extern "C" fn rust_bitbox02hal_init(hal: *mut BitBox02HAL) {
    assert!(!hal.is_null());
    BITBOX02_HAL.store(hal, Ordering::Relaxed);
    BITBOX02_HAL_IN_USE.store(false, Ordering::Relaxed);
    unsafe { hal.cast::<HalImpl>().write(HalImpl::take().unwrap()) };
}

#[cfg(not(any(
    feature = "firmware",
    all(feature = "bootloader", feature = "platform-bitbox02plus")
)))]
#[unsafe(no_mangle)]
/// # Safety
///
/// `hal` must be a valid, non-null pointer to writable `BitBox02HAL` storage.
pub unsafe extern "C" fn rust_bitbox02hal_init(hal: *mut BitBox02HAL) {
    assert!(!hal.is_null());
}

#[cfg(any(
    feature = "firmware",
    all(feature = "bootloader", feature = "platform-bitbox02plus")
))]
unsafe fn acquire_bitbox02hal(hal: *mut BitBox02HAL) -> HalGuard {
    assert!(!hal.is_null());
    assert_eq!(BITBOX02_HAL.load(Ordering::Relaxed), hal);
    assert!(!BITBOX02_HAL_IN_USE.swap(true, Ordering::Relaxed));
    HalGuard { hal }
}

#[cfg(feature = "firmware")]
#[cfg_attr(any(test, feature = "c-unit-testing"), allow(dead_code))]
unsafe fn panic_hal_mut<'a>() -> Option<&'a mut HalImpl> {
    let hal = BITBOX02_HAL.load(Ordering::Relaxed);
    if hal.is_null() {
        None
    } else {
        Some(unsafe { &mut *hal.cast::<HalImpl>() })
    }
}

#[cfg(all(test, feature = "firmware"))]
fn make_test_hal() -> BitBox02HAL {
    BitBox02HAL {
        storage: [0; BITBOX02_HAL_STORAGE_SIZE],
    }
}

// Keep this as a numeric literal so cbindgen reliably exports it to C.
// The static assert below enforces consistency with the Rust HAL constant.
/// Maximum device name length in bytes, including null terminator.
pub const MEMORY_DEVICE_MAX_LEN_WITH_NULL: u8 = 64;
// Keep this as a numeric literal so cbindgen reliably exports it to C.
// The static assert below enforces consistency with the Rust HAL constant.
/// Maximum multisig account name length in bytes, including null terminator.
pub const MEMORY_MULTISIG_NAME_MAX_LEN_WITH_NULL: u8 = 31;

const _: [(); bitbox_hal::memory::DEVICE_NAME_MAX_LEN + 1] =
    [(); MEMORY_DEVICE_MAX_LEN_WITH_NULL as usize];
const _: [(); bitbox_hal::memory::MULTISIG_NAME_MAX_LEN + 1] =
    [(); MEMORY_MULTISIG_NAME_MAX_LEN_WITH_NULL as usize];

// Keep these as numeric literals so cbindgen reliably exports them to C.
// The static asserts below enforce consistency with other Rust constants.
/// Erase sector size of SPI memory in bytes.
const BITBOX02_MEMORY_SPI_ERASE_GRANULARITY: u32 = 4096;
// Keep this as a numeric literal so cbindgen reliably exports it to C.
// The static assert below enforces consistency with the Rust HAL constant.
/// Maximum size in bytes of a BLE firmware image.
pub const MEMORY_SPI_BLE_FIRMWARE_MAX_SIZE: u32 = 32 * 1024;
// Keep this as a numeric literal so cbindgen reliably exports it to C.
/// Start address of BLE firmware slot 1 in SPI memory. Cannot change this as it defines the memory
/// layout.
pub const BITBOX02_MEMORY_SPI_BLE_FIRMWARE_1_ADDR: u32 = 0x00;
/// Start address of BLE firmware slot 2 in SPI memory. Cannot change this as it defines the memory
/// layout.
pub const BITBOX02_MEMORY_SPI_BLE_FIRMWARE_2_ADDR: u32 = MEMORY_SPI_BLE_FIRMWARE_MAX_SIZE;

const _: [(); bitbox_hal::memory::BLE_FIRMWARE_MAX_SIZE] =
    [(); MEMORY_SPI_BLE_FIRMWARE_MAX_SIZE as usize];
const _: [(); bitbox02::spi_mem::BLE_FIRMWARE_1_ADDR as usize] =
    [(); BITBOX02_MEMORY_SPI_BLE_FIRMWARE_1_ADDR as usize];
const _: [(); bitbox02::spi_mem::BLE_FIRMWARE_2_ADDR as usize] =
    [(); BITBOX02_MEMORY_SPI_BLE_FIRMWARE_2_ADDR as usize];

// Addresses must be aligned to an erase sector
const _: [(); 0] = [(); (BITBOX02_MEMORY_SPI_BLE_FIRMWARE_1_ADDR
    % BITBOX02_MEMORY_SPI_ERASE_GRANULARITY) as usize];
const _: [(); 0] = [(); (BITBOX02_MEMORY_SPI_BLE_FIRMWARE_2_ADDR
    % BITBOX02_MEMORY_SPI_ERASE_GRANULARITY) as usize];

// Whenever execution reaches somewhere it isn't supposed to rust code will "panic". Our panic
// handler will print the available information on the screen and over RTT. If we compile with
// `panic=abort` this code will never get executed.
#[cfg_attr(feature = "bootloader", allow(unused_variables))]
#[cfg(not(any(
    feature = "testing",
    feature = "c-unit-testing",
    feature = "simulator-graphical"
)))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    #[cfg(feature = "firmware")]
    ::util::log::log!("{}", info);
    #[cfg(feature = "firmware")]
    {
        use bitbox_hal::{Hal, Ui};

        let msg = alloc::format!("Error: {}", info);
        if let Some(hal) = unsafe { crate::panic_hal_mut() } {
            hal.ui()
                .print_screen(core::time::Duration::from_millis(0), &msg);
        }
    }
    cortex_m::asm::bkpt();
    loop {}
}
