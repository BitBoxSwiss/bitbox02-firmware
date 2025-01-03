// Copyright 2024 Shift Crypto AG
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

use der::asn1::UintRef;
use der::{Decode, SliceReader};

fn parse_int256(decoder: &mut SliceReader) -> Result<[u8; 32], ()> {
    let int = UintRef::decode(decoder).map_err(|_| ())?;
    let int_bytes = int.as_bytes();
    if int_bytes.len() > 32 {
        return Err(());
    }
    let mut array = [0u8; 32];
    let start_index = 32 - int_bytes.len();
    array[start_index..].copy_from_slice(int_bytes);
    Ok(array)
}

fn parse_two_int256s(data: &[u8]) -> Result<([u8; 32], [u8; 32]), ()> {
    let mut decoder = SliceReader::new(data).map_err(|_| ())?;

    let first = parse_int256(&mut decoder)?;
    let second = parse_int256(&mut decoder)?;

    Ok((first, second))
}

/// Parse a ECC signature as returned by the Optiga Trust M.
/// See Solution Reference Manual 6.2.2, example for ECC NIST-P256 signature.
/// https://github.com/Infineon/optiga-trust-m-overview/blob/98b2b9c178f0391b1ab26b52082899704dab688a/docs/pdf/OPTIGA_Trust_M_Datasheet_v3.70.pdf
/// The input is the DER encoding of the signature R/S values encoded as two DER "INGEGER".
/// It's the same encoding as a regular DER-signature, but without the 0x30 sequence header.
/// sig_compact_out must be 64 bytes and will contain the R/S values (each 32 bytes).
#[no_mangle]
pub extern "C" fn rust_der_parse_optiga_signature(
    sig_der: crate::util::Bytes,
    mut sig_compact_out: crate::util::BytesMut,
) -> bool {
    match parse_two_int256s(sig_der.as_ref()) {
        Ok((first, second)) => {
            sig_compact_out.as_mut()[..32].copy_from_slice(&first);
            sig_compact_out.as_mut()[32..].copy_from_slice(&second);
            true
        }
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_der_parse_optiga_signature() {
        let sig_der = b"\x02\x02\x12\x34\x02\x03\x00\xab\xcd";
        let mut sig_compact = [0u8; 64];
        assert!(rust_der_parse_optiga_signature(
            unsafe { crate::util::rust_util_bytes(sig_der.as_ptr(), sig_der.len()) },
            unsafe {
                crate::util::rust_util_bytes_mut(sig_compact.as_mut_ptr(), sig_compact.len())
            },
        ));
        assert_eq!(
            hex::encode(sig_compact),
            "0000000000000000000000000000000000000000000000000000000000001234000000000000000000000000000000000000000000000000000000000000abcd",
        );
    }
}
