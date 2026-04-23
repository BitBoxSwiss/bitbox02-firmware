// SPDX-License-Identifier: Apache-2.0

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

/// Parse an ECC signature as returned by the Optiga Trust M.
/// See Solution Reference Manual 6.2.2, example for ECC NIST-P256 signature.
/// The input is the DER encoding of the signature R/S values encoded as two DER "INTEGER"s.
/// It is the same encoding as a regular DER signature, but without the `0x30` sequence header.
/// `sig_compact_out` will contain the 32-byte R and 32-byte S values.
pub(super) fn parse_optiga_signature(
    sig_der: &[u8],
    sig_compact_out: &mut [u8; 64],
) -> Result<(), ()> {
    let (first, second) = parse_two_int256s(sig_der)?;
    sig_compact_out[..32].copy_from_slice(&first);
    sig_compact_out[32..].copy_from_slice(&second);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_lit::hex;

    #[test]
    fn test_parse_optiga_signature() {
        let sig_der = hex!("02021234020300abcd");
        let mut sig_compact = [0u8; 64];
        parse_optiga_signature(&sig_der, &mut sig_compact).unwrap();
        assert_eq!(
            sig_compact,
            hex!(
                "0000000000000000000000000000000000000000000000000000000000001234\
                 000000000000000000000000000000000000000000000000000000000000abcd"
            ),
        );
    }
}
