// SPDX-License-Identifier: Apache-2.0

use alloc::vec::Vec;

/// Serialize a number in the VarInt encoding.
/// https://en.bitcoin.it/wiki/Protocol_documentation#Variable_length_integer
pub fn serialize_varint(value: u64) -> Vec<u8> {
    bitcoin::consensus::encode::serialize(&bitcoin::consensus::encode::VarInt(value))
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
}
