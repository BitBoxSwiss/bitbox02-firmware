// SPDX-License-Identifier: Apache-2.0

#![no_std]

#[cfg(test)]
#[macro_use]
extern crate std;

#[macro_use]
mod alloc;

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

// Expose C interface defined in util
extern crate util;

#[allow(unused)]
#[cfg(any(
    feature = "firmware",
    all(feature = "bootloader", feature = "platform-bitbox02plus")
))]
type HalImpl = bitbox02::hal::BitBox02Hal;

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
    bitbox02_rust::print_screen!(0, "Error: {}", info);
    cortex_m::asm::bkpt();
    loop {}
}
