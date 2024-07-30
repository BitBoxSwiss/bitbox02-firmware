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

use sha2::Digest;
use sha2::Sha256;

use super::script::serialize_varint;

/// https://github.com/bitcoin/bips/blob/master/bip-0143.mediawiki#specification
pub struct Args<'a> {
    pub version: u32,
    pub hash_prevouts: [u8; 32],
    pub hash_sequence: [u8; 32],
    pub outpoint_hash: [u8; 32],
    pub outpoint_index: u32,
    // The script used in the script code, without the VarInt length prefix.
    pub sighash_script: &'a [u8],
    pub prevout_value: u64,
    pub sequence: u32,
    pub hash_outputs: [u8; 32],
    pub locktime: u32,
    pub sighash_flags: u32,
}

/// Compute the BIP143 signature hash:
/// https://github.com/bitcoin/bips/blob/master/bip-0143.mediawiki#specification
pub fn sighash(args: &Args) -> [u8; 32] {
    let mut ctx = Sha256::new();
    // https://github.com/bitcoin/bips/blob/master/bip-0143.mediawiki#specification
    // 1.
    ctx.update(args.version.to_le_bytes());
    // 2.
    ctx.update(args.hash_prevouts);
    // 3.
    ctx.update(args.hash_sequence);
    // 4.
    ctx.update(args.outpoint_hash);
    ctx.update(args.outpoint_index.to_le_bytes());
    // 5.
    ctx.update(serialize_varint(args.sighash_script.len() as u64));
    ctx.update(args.sighash_script);
    // 6.
    ctx.update(args.prevout_value.to_le_bytes());
    // 7.
    ctx.update(args.sequence.to_le_bytes());
    // 8.
    ctx.update(args.hash_outputs);
    // 9.
    ctx.update(args.locktime.to_le_bytes());
    // 10.
    ctx.update(args.sighash_flags.to_le_bytes());
    Sha256::digest(ctx.finalize()).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sighash() {
        assert_eq!(
        // First test vector taken from:
        // https://github.com/bitcoin/bips/blob/7e3284dafda168da34888977dbf4a55519b0c54d/bip-0143.mediawiki#native-p2wpkh
        sighash(
            &Args{
                version:        1,
                hash_prevouts:  *b"\x96\xb8\x27\xc8\x48\x3d\x4e\x9b\x96\x71\x2b\x67\x13\xa7\xb6\x8d\x6e\x80\x03\xa7\x81\xfe\xba\x36\xc3\x11\x43\x47\x0b\x4e\xfd\x37",
                hash_sequence:  *b"\x52\xb0\xa6\x42\xee\xa2\xfb\x7a\xe6\x38\xc3\x6f\x62\x52\xb6\x75\x02\x93\xdb\xe5\x74\xa8\x06\x98\x4b\x8e\x4d\x85\x48\x33\x9a\x3b",
                outpoint_hash:  *b"\xef\x51\xe1\xb8\x04\xcc\x89\xd1\x82\xd2\x79\x65\x5c\x3a\xa8\x9e\x81\x5b\x1b\x30\x9f\xe2\x87\xd9\xb2\xb5\x5d\x57\xb9\x0e\xc6\x8a",
                outpoint_index: 1,
                sighash_script: b"\x76\xa9\x14\x1d\x0f\x17\x2a\x0e\xcb\x48\xae\xe1\xbe\x1f\x26\x87\xd2\x96\x3a\xe3\x3f\x71\xa1\x88\xac",
                prevout_value: 600000000,
                sequence:      0xFFFFFFFF,
                hash_outputs:   *b"\x86\x3e\xf3\xe1\xa9\x2a\xfb\xfd\xb9\x7f\x31\xad\x0f\xc7\x68\x3e\xe9\x43\xe9\xab\xcf\x25\x01\x59\x0f\xf8\xf6\x55\x1f\x47\xe5\xe5",
                locktime:      17,
                sighash_flags: 1,
            }),
            *b"\xc3\x7a\xf3\x11\x16\xd1\xb2\x7c\xaf\x68\xaa\xe9\xe3\xac\x82\xf1\x47\x79\x29\x01\x4d\x5b\x91\x76\x57\xd0\xeb\x49\x47\x8c\xb6\x70"
        );
    }
}
