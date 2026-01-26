// SPDX-License-Identifier: Apache-2.0

#![no_std]

use crc::Crc;
use util::bytes::{Bytes, BytesMut};

const SL_SOF: u8 = 0x7e;
const SL_ESCAPE: u8 = 0x7d;
const SL_XOR: u8 = 0x20;

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ProtocolPacketType {
    /// 0b00101101
    Ack = 0x2d,
    /// 0b01011010
    Nak = 0x5a,
    /// 0b00111100
    BleData = 0x3c,
    /// 0b10110100
    CtrlData = 0xb4,
    /// 0b01001011
    Ping = 0x4b,
}

fn format_byte(byte: u8, out: &mut [u8], idx: &mut usize) {
    assert!(*idx + 2 <= out.len());
    if byte == SL_SOF || byte == SL_ESCAPE {
        out[*idx] = SL_ESCAPE;
        out[*idx + 1] = byte ^ SL_XOR;
        *idx += 2;
    } else {
        out[*idx] = byte;
        *idx += 1;
    }
}

/// Formats a packet into buf for sending over serial. Worst case the buf_len needs to fit:
/// SOF - 1 byte
/// type - 1 byte
/// len - 2 bytes
/// payload - payload_len bytes
/// CRC - 2 bytes
/// EOF - 1 byte
///
/// Type, len, payload and crc will have some bytes escaped so worst case takes twice the space.
///
/// 2 + (1+2+payload_len+2)*2 = 2 + (5+payload_len)*2 = 12 + 2*payload_len
///
/// For example, 64 bytes require 140 byte buffer worst case.
///
/// Returns number of formatted bytes
pub fn protocol_format(out: &mut [u8], packet_type: ProtocolPacketType, payload: &[u8]) -> usize {
    let payload_len = u16::try_from(payload.len()).expect("payload too large");
    let len_bytes = payload_len.to_le_bytes();

    let mut idx = 0usize;
    assert!(idx < out.len());
    out[idx] = SL_SOF;
    idx += 1;

    format_byte(packet_type as u8, out, &mut idx);
    format_byte(len_bytes[0], out, &mut idx);
    format_byte(len_bytes[1], out, &mut idx);

    for &byte in payload {
        format_byte(byte, out, &mut idx);
    }

    let crc = Crc::<u16>::new(&crc::CRC_16_ARC);
    let mut digest = crc.digest();
    digest.update(&[packet_type as u8]);
    digest.update(&len_bytes);
    digest.update(payload);
    let crc = digest.finalize();

    let crc_bytes = crc.to_le_bytes();
    format_byte(crc_bytes[0], out, &mut idx);
    format_byte(crc_bytes[1], out, &mut idx);

    assert!(idx < out.len());
    out[idx] = SL_SOF;
    idx += 1;

    idx
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_da14531_protocol_format(
    mut out: BytesMut,
    packet_type: ProtocolPacketType,
    payload: Bytes,
) -> u16 {
    let len = protocol_format(out.as_mut(), packet_type, payload.as_ref());
    len.try_into().unwrap()
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_da14531_crc(data: Bytes) -> u16 {
    Crc::<u16>::new(&crc::CRC_16_ARC).checksum(data.as_ref())
}

#[cfg(test)]
mod tests {
    use super::*;
    use util::bytes::rust_util_bytes;

    #[test]
    fn test_rust_da14531_crc_empty() {
        let data = [];
        let crc = rust_da14531_crc(unsafe { rust_util_bytes(data.as_ptr(), data.len()) });
        assert_eq!(crc, 0x0000);
    }

    #[test]
    fn test_rust_da14531_crc_known() {
        let data = *b"123456789";
        let crc = rust_da14531_crc(unsafe { rust_util_bytes(data.as_ptr(), data.len()) });
        assert_eq!(crc, 0xbb3d);
    }
}
