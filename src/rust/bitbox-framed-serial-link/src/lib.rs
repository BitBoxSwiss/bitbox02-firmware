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
    extern crate alloc;
    use alloc::vec;
    use hex_lit::hex;
    use util::bytes::rust_util_bytes;

    fn assert_format_case(packet_type: ProtocolPacketType, payload: &[u8], expected: &[u8]) {
        let mut out = vec![0u8; expected.len()];
        let len = protocol_format(&mut out, packet_type, payload);
        assert_eq!(len, expected.len());
        assert_eq!(&out[..len], expected);
    }

    // Verifies a small payload that requires no escaping is formatted correctly.
    #[test]
    fn test_format_basic_no_escaping() {
        let payload = hex!("010203");
        let expected = hex!("7eb403000102034f157e");
        assert_format_case(ProtocolPacketType::CtrlData, &payload, &expected);
    }

    // Verifies payload bytes requiring escaping are encoded correctly.
    #[test]
    fn test_format_payload_bytes_escaped() {
        let payload = [SL_SOF, SL_ESCAPE, 0x55];
        let expected = hex!("7eb403007d5e7d5d55dec37e");
        assert_format_case(ProtocolPacketType::CtrlData, &payload, &expected);
    }

    // Verifies framing/CRC for a zero-length payload.
    #[test]
    fn test_format_zero_length_payload() {
        let payload: [u8; 0] = [];
        let expected = hex!("7eb4000040267e");
        assert_format_case(ProtocolPacketType::CtrlData, &payload, &expected);
    }
    // Verifies every single-byte payload value formats correctly.
    #[test]
    fn test_format_payload_all_bytes() {
        let fixtures: [&[u8]; 256] = [
            &hex!("7e3c0100005d907e"),
            &hex!("7e3c0100019c507e"),
            &hex!("7e3c010002dc517e"),
            &hex!("7e3c0100031d917e"),
            &hex!("7e3c0100045c537e"),
            &hex!("7e3c0100059d937e"),
            &hex!("7e3c010006dd927e"),
            &hex!("7e3c0100071c527e"),
            &hex!("7e3c0100085c567e"),
            &hex!("7e3c0100099d967e"),
            &hex!("7e3c01000add977e"),
            &hex!("7e3c01000b1c577e"),
            &hex!("7e3c01000c5d957e"),
            &hex!("7e3c01000d9c557e"),
            &hex!("7e3c01000edc547e"),
            &hex!("7e3c01000f1d947e"),
            &hex!("7e3c0100105c5c7e"),
            &hex!("7e3c0100119d9c7e"),
            &hex!("7e3c010012dd9d7e"),
            &hex!("7e3c0100131c5d7e"),
            &hex!("7e3c0100145d9f7e"),
            &hex!("7e3c0100159c5f7e"),
            &hex!("7e3c010016dc5e7e"),
            &hex!("7e3c0100171d9e7e"),
            &hex!("7e3c0100185d9a7e"),
            &hex!("7e3c0100199c5a7e"),
            &hex!("7e3c01001adc5b7e"),
            &hex!("7e3c01001b1d9b7e"),
            &hex!("7e3c01001c5c597e"),
            &hex!("7e3c01001d9d997e"),
            &hex!("7e3c01001edd987e"),
            &hex!("7e3c01001f1c587e"),
            &hex!("7e3c0100205c487e"),
            &hex!("7e3c0100219d887e"),
            &hex!("7e3c010022dd897e"),
            &hex!("7e3c0100231c497e"),
            &hex!("7e3c0100245d8b7e"),
            &hex!("7e3c0100259c4b7e"),
            &hex!("7e3c010026dc4a7e"),
            &hex!("7e3c0100271d8a7e"),
            &hex!("7e3c0100285d8e7e"),
            &hex!("7e3c0100299c4e7e"),
            &hex!("7e3c01002adc4f7e"),
            &hex!("7e3c01002b1d8f7e"),
            &hex!("7e3c01002c5c4d7e"),
            &hex!("7e3c01002d9d8d7e"),
            &hex!("7e3c01002edd8c7e"),
            &hex!("7e3c01002f1c4c7e"),
            &hex!("7e3c0100305d847e"),
            &hex!("7e3c0100319c447e"),
            &hex!("7e3c010032dc457e"),
            &hex!("7e3c0100331d857e"),
            &hex!("7e3c0100345c477e"),
            &hex!("7e3c0100359d877e"),
            &hex!("7e3c010036dd867e"),
            &hex!("7e3c0100371c467e"),
            &hex!("7e3c0100385c427e"),
            &hex!("7e3c0100399d827e"),
            &hex!("7e3c01003add837e"),
            &hex!("7e3c01003b1c437e"),
            &hex!("7e3c01003c5d817e"),
            &hex!("7e3c01003d9c417e"),
            &hex!("7e3c01003edc407e"),
            &hex!("7e3c01003f1d807e"),
            &hex!("7e3c0100405c607e"),
            &hex!("7e3c0100419da07e"),
            &hex!("7e3c010042dda17e"),
            &hex!("7e3c0100431c617e"),
            &hex!("7e3c0100445da37e"),
            &hex!("7e3c0100459c637e"),
            &hex!("7e3c010046dc627e"),
            &hex!("7e3c0100471da27e"),
            &hex!("7e3c0100485da67e"),
            &hex!("7e3c0100499c667e"),
            &hex!("7e3c01004adc677e"),
            &hex!("7e3c01004b1da77e"),
            &hex!("7e3c01004c5c657e"),
            &hex!("7e3c01004d9da57e"),
            &hex!("7e3c01004edda47e"),
            &hex!("7e3c01004f1c647e"),
            &hex!("7e3c0100505dac7e"),
            &hex!("7e3c0100519c6c7e"),
            &hex!("7e3c010052dc6d7e"),
            &hex!("7e3c0100531dad7e"),
            &hex!("7e3c0100545c6f7e"),
            &hex!("7e3c0100559daf7e"),
            &hex!("7e3c010056ddae7e"),
            &hex!("7e3c0100571c6e7e"),
            &hex!("7e3c0100585c6a7e"),
            &hex!("7e3c0100599daa7e"),
            &hex!("7e3c01005addab7e"),
            &hex!("7e3c01005b1c6b7e"),
            &hex!("7e3c01005c5da97e"),
            &hex!("7e3c01005d9c697e"),
            &hex!("7e3c01005edc687e"),
            &hex!("7e3c01005f1da87e"),
            &hex!("7e3c0100605db87e"),
            &hex!("7e3c0100619c787e"),
            &hex!("7e3c010062dc797e"),
            &hex!("7e3c0100631db97e"),
            &hex!("7e3c0100645c7b7e"),
            &hex!("7e3c0100659dbb7e"),
            &hex!("7e3c010066ddba7e"),
            &hex!("7e3c0100671c7a7e"),
            &hex!("7e3c0100685c7d5e7e"),
            &hex!("7e3c0100699dbe7e"),
            &hex!("7e3c01006addbf7e"),
            &hex!("7e3c01006b1c7f7e"),
            &hex!("7e3c01006c5dbd7e"),
            &hex!("7e3c01006d9c7d5d7e"),
            &hex!("7e3c01006edc7c7e"),
            &hex!("7e3c01006f1dbc7e"),
            &hex!("7e3c0100705c747e"),
            &hex!("7e3c0100719db47e"),
            &hex!("7e3c010072ddb57e"),
            &hex!("7e3c0100731c757e"),
            &hex!("7e3c0100745db77e"),
            &hex!("7e3c0100759c777e"),
            &hex!("7e3c010076dc767e"),
            &hex!("7e3c0100771db67e"),
            &hex!("7e3c0100785db27e"),
            &hex!("7e3c0100799c727e"),
            &hex!("7e3c01007adc737e"),
            &hex!("7e3c01007b1db37e"),
            &hex!("7e3c01007c5c717e"),
            &hex!("7e3c01007d5d9db17e"),
            &hex!("7e3c01007d5eddb07e"),
            &hex!("7e3c01007f1c707e"),
            &hex!("7e3c0100805c307e"),
            &hex!("7e3c0100819df07e"),
            &hex!("7e3c010082ddf17e"),
            &hex!("7e3c0100831c317e"),
            &hex!("7e3c0100845df37e"),
            &hex!("7e3c0100859c337e"),
            &hex!("7e3c010086dc327e"),
            &hex!("7e3c0100871df27e"),
            &hex!("7e3c0100885df67e"),
            &hex!("7e3c0100899c367e"),
            &hex!("7e3c01008adc377e"),
            &hex!("7e3c01008b1df77e"),
            &hex!("7e3c01008c5c357e"),
            &hex!("7e3c01008d9df57e"),
            &hex!("7e3c01008eddf47e"),
            &hex!("7e3c01008f1c347e"),
            &hex!("7e3c0100905dfc7e"),
            &hex!("7e3c0100919c3c7e"),
            &hex!("7e3c010092dc3d7e"),
            &hex!("7e3c0100931dfd7e"),
            &hex!("7e3c0100945c3f7e"),
            &hex!("7e3c0100959dff7e"),
            &hex!("7e3c010096ddfe7e"),
            &hex!("7e3c0100971c3e7e"),
            &hex!("7e3c0100985c3a7e"),
            &hex!("7e3c0100999dfa7e"),
            &hex!("7e3c01009addfb7e"),
            &hex!("7e3c01009b1c3b7e"),
            &hex!("7e3c01009c5df97e"),
            &hex!("7e3c01009d9c397e"),
            &hex!("7e3c01009edc387e"),
            &hex!("7e3c01009f1df87e"),
            &hex!("7e3c0100a05de87e"),
            &hex!("7e3c0100a19c287e"),
            &hex!("7e3c0100a2dc297e"),
            &hex!("7e3c0100a31de97e"),
            &hex!("7e3c0100a45c2b7e"),
            &hex!("7e3c0100a59deb7e"),
            &hex!("7e3c0100a6ddea7e"),
            &hex!("7e3c0100a71c2a7e"),
            &hex!("7e3c0100a85c2e7e"),
            &hex!("7e3c0100a99dee7e"),
            &hex!("7e3c0100aaddef7e"),
            &hex!("7e3c0100ab1c2f7e"),
            &hex!("7e3c0100ac5ded7e"),
            &hex!("7e3c0100ad9c2d7e"),
            &hex!("7e3c0100aedc2c7e"),
            &hex!("7e3c0100af1dec7e"),
            &hex!("7e3c0100b05c247e"),
            &hex!("7e3c0100b19de47e"),
            &hex!("7e3c0100b2dde57e"),
            &hex!("7e3c0100b31c257e"),
            &hex!("7e3c0100b45de77e"),
            &hex!("7e3c0100b59c277e"),
            &hex!("7e3c0100b6dc267e"),
            &hex!("7e3c0100b71de67e"),
            &hex!("7e3c0100b85de27e"),
            &hex!("7e3c0100b99c227e"),
            &hex!("7e3c0100badc237e"),
            &hex!("7e3c0100bb1de37e"),
            &hex!("7e3c0100bc5c217e"),
            &hex!("7e3c0100bd9de17e"),
            &hex!("7e3c0100bedde07e"),
            &hex!("7e3c0100bf1c207e"),
            &hex!("7e3c0100c05dc07e"),
            &hex!("7e3c0100c19c007e"),
            &hex!("7e3c0100c2dc017e"),
            &hex!("7e3c0100c31dc17e"),
            &hex!("7e3c0100c45c037e"),
            &hex!("7e3c0100c59dc37e"),
            &hex!("7e3c0100c6ddc27e"),
            &hex!("7e3c0100c71c027e"),
            &hex!("7e3c0100c85c067e"),
            &hex!("7e3c0100c99dc67e"),
            &hex!("7e3c0100caddc77e"),
            &hex!("7e3c0100cb1c077e"),
            &hex!("7e3c0100cc5dc57e"),
            &hex!("7e3c0100cd9c057e"),
            &hex!("7e3c0100cedc047e"),
            &hex!("7e3c0100cf1dc47e"),
            &hex!("7e3c0100d05c0c7e"),
            &hex!("7e3c0100d19dcc7e"),
            &hex!("7e3c0100d2ddcd7e"),
            &hex!("7e3c0100d31c0d7e"),
            &hex!("7e3c0100d45dcf7e"),
            &hex!("7e3c0100d59c0f7e"),
            &hex!("7e3c0100d6dc0e7e"),
            &hex!("7e3c0100d71dce7e"),
            &hex!("7e3c0100d85dca7e"),
            &hex!("7e3c0100d99c0a7e"),
            &hex!("7e3c0100dadc0b7e"),
            &hex!("7e3c0100db1dcb7e"),
            &hex!("7e3c0100dc5c097e"),
            &hex!("7e3c0100dd9dc97e"),
            &hex!("7e3c0100deddc87e"),
            &hex!("7e3c0100df1c087e"),
            &hex!("7e3c0100e05c187e"),
            &hex!("7e3c0100e19dd87e"),
            &hex!("7e3c0100e2ddd97e"),
            &hex!("7e3c0100e31c197e"),
            &hex!("7e3c0100e45ddb7e"),
            &hex!("7e3c0100e59c1b7e"),
            &hex!("7e3c0100e6dc1a7e"),
            &hex!("7e3c0100e71dda7e"),
            &hex!("7e3c0100e85dde7e"),
            &hex!("7e3c0100e99c1e7e"),
            &hex!("7e3c0100eadc1f7e"),
            &hex!("7e3c0100eb1ddf7e"),
            &hex!("7e3c0100ec5c1d7e"),
            &hex!("7e3c0100ed9ddd7e"),
            &hex!("7e3c0100eedddc7e"),
            &hex!("7e3c0100ef1c1c7e"),
            &hex!("7e3c0100f05dd47e"),
            &hex!("7e3c0100f19c147e"),
            &hex!("7e3c0100f2dc157e"),
            &hex!("7e3c0100f31dd57e"),
            &hex!("7e3c0100f45c177e"),
            &hex!("7e3c0100f59dd77e"),
            &hex!("7e3c0100f6ddd67e"),
            &hex!("7e3c0100f71c167e"),
            &hex!("7e3c0100f85c127e"),
            &hex!("7e3c0100f99dd27e"),
            &hex!("7e3c0100faddd37e"),
            &hex!("7e3c0100fb1c137e"),
            &hex!("7e3c0100fc5dd17e"),
            &hex!("7e3c0100fd9c117e"),
            &hex!("7e3c0100fedc107e"),
            &hex!("7e3c0100ff1dd07e"),
        ];
        for (value, expected) in fixtures.iter().enumerate() {
            let payload = [value as u8];
            assert_format_case(ProtocolPacketType::BleData, &payload, expected);
        }
    }

    // Verifies escaping when the low payload length byte matches SOF/ESCAPE.
    #[test]
    fn test_format_len_low_escaping() {
        let payload_7d = vec![0x11; 0x7d];
        let mut expected_7d = hex!("7eb47d5d00").to_vec();
        expected_7d.extend(core::iter::repeat_n(0x11, payload_7d.len()));
        expected_7d.extend_from_slice(&hex!("08a17e"));
        assert_format_case(ProtocolPacketType::CtrlData, &payload_7d, &expected_7d);

        let payload_7e = vec![0x11; 0x7e];
        let mut expected_7e = hex!("7eb47d5e00").to_vec();
        expected_7e.extend(core::iter::repeat_n(0x11, payload_7e.len()));
        expected_7e.extend_from_slice(&hex!("602e7e"));
        assert_format_case(ProtocolPacketType::CtrlData, &payload_7e, &expected_7e);
    }

    // Verifies escaping when the high payload length byte matches SOF/ESCAPE.
    #[test]
    fn test_format_len_high_escaping() {
        let payload_len_7d00 = 0x7d00usize;
        let payload_len_7e00 = 0x7e00usize;
        let payload = vec![0x22; payload_len_7e00];

        let mut expected_7d00 = hex!("7eb4007d5d").to_vec();
        expected_7d00.extend(core::iter::repeat_n(0x22, payload_len_7d00));
        expected_7d00.extend_from_slice(&hex!("c2f87e"));
        assert_eq!(expected_7d00.len(), 32008);
        assert_format_case(
            ProtocolPacketType::CtrlData,
            &payload[..payload_len_7d00],
            &expected_7d00,
        );

        let mut expected_7e00 = hex!("7eb4007d5e").to_vec();
        expected_7e00.extend(core::iter::repeat_n(0x22, payload_len_7e00));
        expected_7e00.extend_from_slice(&hex!("7ce97e"));
        assert_eq!(expected_7e00.len(), 32264);
        assert_format_case(ProtocolPacketType::CtrlData, &payload, &expected_7e00);
    }

    // Verifies CRC bytes requiring escaping are encoded correctly.
    #[test]
    fn test_format_crc_escaping() {
        let payload_low = hex!("6800");
        let payload_high = hex!("0001");
        let expected_low = hex!("7e3c020068007d5e7d5d7e");
        let expected_high = hex!("7e3c02000001907d5d7e");
        assert_format_case(ProtocolPacketType::BleData, &payload_low, &expected_low);
        assert_format_case(ProtocolPacketType::BleData, &payload_high, &expected_high);
    }

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
