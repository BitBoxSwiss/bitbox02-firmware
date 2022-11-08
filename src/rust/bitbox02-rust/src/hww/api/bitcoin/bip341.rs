// Copyright 2022 Shift Crypto AG
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

/// https://github.com/bitcoin/bips/blob/bb8dc57da9b3c6539b88378348728a2ff43f7e9c/bip-0341.mediawiki#common-signature-message
pub struct Args {
    // Transaction data:
    pub version: u32,
    pub locktime: u32,
    pub hash_prevouts: [u8; 32],
    pub hash_amounts: [u8; 32],
    pub hash_scriptpubkeys: [u8; 32],
    pub hash_sequences: [u8; 32],
    pub hash_outputs: [u8; 32],
    // Data about this input:
    pub input_index: u32,
}

/// Compute the BIP341 signature hash.
///
/// https://github.com/bitcoin/bips/blob/bb8dc57da9b3c6539b88378348728a2ff43f7e9c/bip-0341.mediawiki#common-signature-message
///
/// The hash_type is assumed 0 (`SIGHASH_DEFAULT`). The `ext_flag` is
/// assumed 0 and `annex` is assumed to be not present.
pub fn sighash(args: &Args) -> [u8; 32] {
    let tag = Sha256::digest(b"TapSighash");
    let mut ctx = Sha256::new();
    ctx.update(tag);
    ctx.update(tag);
    // Sighash epoch 0
    ctx.update(0u8.to_le_bytes());
    // Control:
    ctx.update(0u8.to_le_bytes());
    // Transaction data:
    ctx.update(args.version.to_le_bytes());
    ctx.update(args.locktime.to_le_bytes());
    ctx.update(args.hash_prevouts);
    ctx.update(args.hash_amounts);
    ctx.update(args.hash_scriptpubkeys);
    ctx.update(args.hash_sequences);
    ctx.update(args.hash_outputs);
    // spend_type is 0 because ext_flag is 0 and annex is absent.
    ctx.update(0u8.to_le_bytes());
    // Data about this input:
    ctx.update(args.input_index.to_le_bytes());

    ctx.finalize().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sighash() {
        assert_eq!(
            // Test vector from:
            // https://github.com/bitcoin/bips/blob/97e02b2223b21753acefa813a4e59dbb6e849e77/bip-0341/wallet-test-vectors.json#L350-L355
            // It is the only test vector with hash type 0.
            sighash(&Args {
                version: 2,
                locktime: 500000000,
                hash_prevouts: *b"\xe3\xb3\x3b\xb4\xef\x3a\x52\xad\x1f\xff\xb5\x55\xc0\xd8\x28\x28\xeb\x22\x73\x70\x36\xea\xeb\x02\xa2\x35\xd8\x2b\x90\x9c\x4c\x3f",
                hash_amounts: *b"\x58\xa6\x96\x4a\x4f\x5f\x8f\x0b\x64\x2d\xed\x0a\x8a\x55\x3b\xe7\x62\x2a\x71\x9d\xa7\x1d\x1f\x5b\xef\xce\xfc\xde\xe8\xe0\xfd\xe6",
                hash_scriptpubkeys: *b"\x23\xad\x0f\x61\xad\x2b\xca\x5b\xa6\xa7\x69\x3f\x50\xfc\xe9\x88\xe1\x7c\x37\x80\xbf\x2b\x1e\x72\x0c\xfb\xb3\x8f\xbd\xd5\x2e\x21",
                hash_sequences: *b"\x18\x95\x9c\x72\x21\xab\x5c\xe9\xe2\x6c\x3c\xd6\x7b\x22\xc2\x4f\x8b\xaa\x54\xba\xc2\x81\xd8\xe6\xb0\x5e\x40\x0e\x6c\x3a\x95\x7e",
                hash_outputs: *b"\xa2\xe6\xda\xb7\xc1\xf0\xdc\xd2\x97\xc8\xd6\x16\x47\xfd\x17\xd8\x21\x54\x1e\xa6\x9c\x3c\xc3\x7d\xcb\xad\x7f\x90\xd4\xeb\x4b\xc5",
                input_index: 4,
            }),
            *b"\x4f\x90\x0a\x0b\xae\x3f\x14\x46\xfd\x48\x49\x0c\x29\x58\xb5\xa0\x23\x22\x8f\x01\x66\x1c\xda\x34\x96\xa1\x1d\xa5\x02\xa7\xf7\xef");
    }
}
