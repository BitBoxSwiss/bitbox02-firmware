// SPDX-License-Identifier: Apache-2.0

use crate::ringbuffer::RingBuffer;

/// Set the product string of the BLE chip. The product string must be smaller than 64 bytes.
pub fn set_product(product: &str, queue: &mut RingBuffer) {
    let product = product.as_bytes();
    unsafe {
        bitbox02_sys::da14531_set_product(product.as_ptr(), product.len() as u16, &mut queue.inner)
    }
}

/// Set the device name of the BLE chip. The name must contain no null bytes.
pub fn set_name(name: &str, queue: &mut RingBuffer) {
    let c_name = util::strings::str_to_cstr_vec(name).unwrap();
    unsafe { bitbox02_sys::da14531_set_name(c_name.as_ptr(), &mut queue.inner) }
}

/// Power down the BLE chip.
pub fn power_down(queue: &mut RingBuffer) {
    unsafe { bitbox02_sys::da14531_power_down(&mut queue.inner) }
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
    const CTRL_CMD_PRODUCT_STRING: u8 = 7;
    const CTRL_CMD_BLE_POWER_DOWN: u8 = 12;

    fn drain(queue: &mut RingBuffer) -> Vec<u8> {
        let mut out = Vec::new();
        while queue.len() > 0 {
            out.push(queue.get().unwrap());
        }
        out
    }

    #[test]
    fn test_set_product() {
        let product = "foo bar";
        let mut buf = [0u8; 256];
        let mut queue = RingBuffer::new(&mut buf);

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
        let mut buf = [0u8; 256];
        let mut queue = RingBuffer::new(&mut buf);

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
        let mut buf = [0u8; 256];
        let mut queue = RingBuffer::new(&mut buf);

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
}
