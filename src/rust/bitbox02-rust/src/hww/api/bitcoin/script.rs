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

use alloc::vec::Vec;

// https://en.bitcoin.it/wiki/Script
pub const OP_0: u8 = 0;
pub const OP_1: u8 = 0x51;
pub const OP_HASH160: u8 = 0xa9;
pub const OP_DUP: u8 = 0x76;
pub const OP_EQUALVERIFY: u8 = 0x88;
pub const OP_CHECKSIG: u8 = 0xac;
pub const OP_EQUAL: u8 = 0x87;

/// Serialize a number in the VarInt encoding.
/// https://en.bitcoin.it/wiki/Protocol_documentation#Variable_length_integer
pub fn serialize_varint(value: u64) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    match value {
        0..=0xFC => out.push(value as _),
        0xFD..=0xFFFF => {
            out.push(0xFD);
            out.extend_from_slice(&(value as u16).to_le_bytes());
        }
        0x10000..=0xFFFFFFFF => {
            out.push(0xFE);
            out.extend_from_slice(&(value as u32).to_le_bytes());
        }
        _ => {
            out.push(0xFF);
            out.extend_from_slice(&value.to_le_bytes());
        }
    }
    out
}

/// Performs a data push onto `v`: the varint length of data followed by data.
pub fn push_data(v: &mut Vec<u8>, data: &[u8]) {
    v.extend_from_slice(&serialize_varint(data.len() as _));
    v.extend_from_slice(data);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_varint() {
        // Go script to generate the test vectors below.
        /*
        package main

        import (
            "bytes"
            "encoding/hex"
            "fmt"
            "regexp"

            "github.com/btcsuite/btcd/wire"
        )

        func main() {
            formatHex := func(v []byte) string {
                h := hex.EncodeToString(v)
                return regexp.MustCompile("(..)").ReplaceAllString(h, "\\x$1")
            }
            for _, val := range []uint64{0, 1, 2, 127, 128, 255, 256, 257, 300, 0xFFFF - 1, 0xFFFF, 0xFFFF + 1, 0xFFFF + 100, 0xFFFFFFFF - 1, 0xFFFFFFFF, 0xFFFFFFFF + 1, 0xFFFFFFFF + 100, 0XFFFFFFFFFFFFFFFF} {
                buf := new(bytes.Buffer)
                wire.WriteVarInt(buf, 0, val)
                fmt.Printf("assert_eq!(serialize_varint(0x%x), b\"%s\");\n", val, formatHex(buf.Bytes()))
            }
        }
        */

        assert_eq!(serialize_varint(0x0), b"\x00");
        assert_eq!(serialize_varint(0x1), b"\x01");
        assert_eq!(serialize_varint(0x2), b"\x02");
        assert_eq!(serialize_varint(0x7f), b"\x7f");
        assert_eq!(serialize_varint(0x80), b"\x80");
        assert_eq!(serialize_varint(0xff), b"\xfd\xff\x00");
        assert_eq!(serialize_varint(0x100), b"\xfd\x00\x01");
        assert_eq!(serialize_varint(0x101), b"\xfd\x01\x01");
        assert_eq!(serialize_varint(0x12c), b"\xfd\x2c\x01");
        assert_eq!(serialize_varint(0xfffe), b"\xfd\xfe\xff");
        assert_eq!(serialize_varint(0xffff), b"\xfd\xff\xff");
        assert_eq!(serialize_varint(0x10000), b"\xfe\x00\x00\x01\x00");
        assert_eq!(serialize_varint(0x10063), b"\xfe\x63\x00\x01\x00");
        assert_eq!(serialize_varint(0xfffffffe), b"\xfe\xfe\xff\xff\xff");
        assert_eq!(serialize_varint(0xffffffff), b"\xfe\xff\xff\xff\xff");
        assert_eq!(
            serialize_varint(0x100000000),
            b"\xff\x00\x00\x00\x00\x01\x00\x00\x00"
        );
        assert_eq!(
            serialize_varint(0x100000063),
            b"\xff\x63\x00\x00\x00\x01\x00\x00\x00"
        );
        assert_eq!(
            serialize_varint(0xffffffffffffffff),
            b"\xff\xff\xff\xff\xff\xff\xff\xff\xff"
        );
    }

    #[test]
    fn test_push_data() {
        assert_eq!(
            {
                let mut v = Vec::new();
                push_data(&mut v, b"");
                v
            },
            vec![0]
        );

        // Data with length 255.
        assert_eq!(
            {
                let mut v = Vec::new();
                push_data(&mut v, b"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb");
                v
            },
            b"\xfd\xff\x00bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb".to_vec(),
        );
    }
}
