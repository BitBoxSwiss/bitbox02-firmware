// SPDX-License-Identifier: Apache-2.0

#![no_std]

extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;
use bitbox_bytequeue::{ByteQueue, RustByteQueue};
use bitbox_framed_serial_link::{ProtocolPacketType, protocol_format};
use util::bytes::Bytes;

const CTRL_CMD_DEVICE_NAME: u8 = 1;
const CTRL_CMD_BLE_STATUS: u8 = 5;
const CTRL_CMD_PRODUCT_STRING: u8 = 7;
const CTRL_CMD_BLE_CHIP_RESET: u8 = 8;
const CTRL_CMD_BLE_POWER_DOWN: u8 = 12;
const CTRL_PAYLOAD_MAX_LEN: usize = 64;

fn enqueue_ctrl_data(payload: &[u8], queue: &mut ByteQueue) {
    let mut frame = vec![0u8; 12 + payload.len() * 2];
    let frame_len = protocol_format(&mut frame, ProtocolPacketType::CtrlData, payload);
    for &byte in &frame[..frame_len] {
        queue.put(byte);
    }
}

/// Set the product string of the BLE chip. The product string must be smaller than 64 bytes.
pub fn set_product(product: &str, queue: &mut ByteQueue) {
    set_product_bytes(product.as_bytes(), queue);
}

/// Set the product string of the BLE chip. The product string must be smaller than 64 bytes.
pub fn set_product_bytes(product: &[u8], queue: &mut ByteQueue) {
    assert!(
        product.len() < CTRL_PAYLOAD_MAX_LEN,
        "product string too large"
    );

    let mut payload = Vec::with_capacity(1 + product.len());
    payload.push(CTRL_CMD_PRODUCT_STRING);
    payload.extend_from_slice(product);
    enqueue_ctrl_data(&payload, queue);
}

/// Set the device name of the BLE chip.
pub fn set_name(name: &str, queue: &mut ByteQueue) {
    set_name_bytes(name.as_bytes(), queue);
}

/// Set the device name of the BLE chip.
pub fn set_name_bytes(name: &[u8], queue: &mut ByteQueue) {
    let mut payload = Vec::with_capacity(1 + name.len());
    payload.push(CTRL_CMD_DEVICE_NAME);
    payload.extend_from_slice(name);
    enqueue_ctrl_data(&payload, queue);
}

/// Power down the BLE chip.
pub fn power_down(queue: &mut ByteQueue) {
    enqueue_ctrl_data(&[CTRL_CMD_BLE_POWER_DOWN, 0], queue);
}

/// Reset the BLE chip.
pub fn reset(queue: &mut ByteQueue) {
    enqueue_ctrl_data(&[CTRL_CMD_BLE_CHIP_RESET], queue);
}

/// Ask the BLE chip for its current connection state.
pub fn get_connection_state(queue: &mut ByteQueue) {
    enqueue_ctrl_data(&[CTRL_CMD_BLE_STATUS], queue);
}

fn queue_from_ptr(uart_out: *mut RustByteQueue) -> *mut ByteQueue {
    assert!(!uart_out.is_null());
    uart_out.cast::<ByteQueue>()
}

/// # Safety
///
/// `uart_out` must point to a valid `RustByteQueue` allocated by Rust.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_da14531_power_down(uart_out: *mut RustByteQueue) {
    power_down(unsafe { &mut *queue_from_ptr(uart_out) });
}

/// # Safety
///
/// `uart_out` must point to a valid `RustByteQueue` allocated by Rust.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_da14531_reset(uart_out: *mut RustByteQueue) {
    reset(unsafe { &mut *queue_from_ptr(uart_out) });
}

/// # Safety
///
/// `product` must reference a valid byte buffer for the duration of this call.
/// `uart_out` must point to a valid `RustByteQueue` allocated by Rust.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_da14531_set_product(product: Bytes, uart_out: *mut RustByteQueue) {
    set_product_bytes(product.as_ref(), unsafe { &mut *queue_from_ptr(uart_out) });
}

/// # Safety
///
/// `name` must reference a valid byte buffer for the duration of this call.
/// `uart_out` must point to a valid `RustByteQueue` allocated by Rust.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_da14531_set_name(name: Bytes, uart_out: *mut RustByteQueue) {
    set_name_bytes(name.as_ref(), unsafe { &mut *queue_from_ptr(uart_out) });
}

/// # Safety
///
/// `uart_out` must point to a valid `RustByteQueue` allocated by Rust.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_da14531_get_connection_state(uart_out: *mut RustByteQueue) {
    get_connection_state(unsafe { &mut *queue_from_ptr(uart_out) });
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate alloc;
    use alloc::vec;
    use alloc::vec::Vec;
    use bitbox_framed_serial_link::{ProtocolPacketType, protocol_format};
    use hex_lit::hex;

    const CTRL_CMD_DEVICE_NAME: u8 = 1;
    const CTRL_CMD_BLE_STATUS: u8 = 5;
    const CTRL_CMD_PRODUCT_STRING: u8 = 7;
    const CTRL_CMD_BLE_CHIP_RESET: u8 = 8;
    const CTRL_CMD_BLE_POWER_DOWN: u8 = 12;

    fn drain(queue: &mut ByteQueue) -> Vec<u8> {
        let mut out = Vec::new();
        while queue.num() > 0 {
            out.push(queue.get().unwrap());
        }
        out
    }

    #[test]
    fn test_set_product() {
        let product = "foo bar";
        let mut queue = ByteQueue::with_capacity(64);

        set_product(product, &mut queue);

        let actual = drain(&mut queue);

        let mut payload = vec![CTRL_CMD_PRODUCT_STRING];
        payload.extend_from_slice(product.as_bytes());
        let mut expected = vec![0u8; 140];
        let expected_len = protocol_format(&mut expected, ProtocolPacketType::CtrlData, &payload);
        expected.truncate(expected_len);
        assert_eq!(actual, expected);
        assert_eq!(actual, hex!("7eb4080007666f6f206261725b057e").to_vec());
    }

    #[test]
    fn test_power_down() {
        let mut queue = ByteQueue::with_capacity(64);

        power_down(&mut queue);

        let actual = drain(&mut queue);

        let payload = vec![CTRL_CMD_BLE_POWER_DOWN, 0];
        let mut expected = vec![0u8; 140];
        let expected_len = protocol_format(&mut expected, ProtocolPacketType::CtrlData, &payload);
        expected.truncate(expected_len);
        assert_eq!(actual, expected);
        assert_eq!(actual, hex!("7eb402000c00b4a27e").to_vec());
    }

    #[test]
    fn test_set_name() {
        let name = "foo bar";
        let mut queue = ByteQueue::with_capacity(64);

        set_name(name, &mut queue);

        let actual = drain(&mut queue);

        let mut payload = vec![CTRL_CMD_DEVICE_NAME];
        payload.extend_from_slice(name.as_bytes());
        let mut expected = vec![0u8; 140];
        let expected_len = protocol_format(&mut expected, ProtocolPacketType::CtrlData, &payload);
        expected.truncate(expected_len);
        assert_eq!(actual, expected);
        assert_eq!(actual, hex!("7eb4080001666f6f20626172db2f7e").to_vec());
    }

    #[test]
    fn test_reset() {
        let mut queue = ByteQueue::with_capacity(64);

        reset(&mut queue);

        let actual = drain(&mut queue);

        let payload = vec![CTRL_CMD_BLE_CHIP_RESET];
        let mut expected = vec![0u8; 140];
        let expected_len = protocol_format(&mut expected, ProtocolPacketType::CtrlData, &payload);
        expected.truncate(expected_len);
        assert_eq!(actual, expected);
        assert_eq!(actual, hex!("7eb401000877f67e").to_vec());
    }

    #[test]
    fn test_get_connection_state() {
        let mut queue = ByteQueue::with_capacity(64);

        get_connection_state(&mut queue);

        let actual = drain(&mut queue);

        let payload = vec![CTRL_CMD_BLE_STATUS];
        let mut expected = vec![0u8; 140];
        let expected_len = protocol_format(&mut expected, ProtocolPacketType::CtrlData, &payload);
        expected.truncate(expected_len);
        assert_eq!(actual, expected);
        assert_eq!(actual, hex!("7eb4010005b6337e").to_vec());
    }
}
