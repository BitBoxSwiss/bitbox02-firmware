//! Regression tests for BER indefinite-length scanning.

#![cfg(feature = "ber")]
#![allow(clippy::unwrap_used, reason = "tests")]

use der::{
    Decode, EncodingRules, ErrorKind, Header, Length, Reader, SliceReader, Tag, asn1::AnyRef,
};
use hex_literal::hex;

/// Wrap a `NULL` in `depth` indefinite-length `SEQUENCE`s.
fn nested_sequences(depth: usize) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(4 * depth + 2);
    for _ in 0..depth {
        bytes.extend_from_slice(&[0x30, 0x80]);
    }
    bytes.extend_from_slice(&[0x05, 0x00]);
    bytes.resize(4 * depth + 2, 0);
    bytes
}

fn assert_length(bytes: &[u8]) {
    let mut reader = SliceReader::new_with_encoding_rules(bytes, EncodingRules::Ber).unwrap();
    let header = Header::decode(&mut reader).unwrap();
    assert_eq!(header.tag(), Tag::Sequence);
    assert_eq!(usize::try_from(header.length()).unwrap(), bytes.len() - 2);
    assert_eq!(reader.position(), Length::new(2));
    assert_eq!(AnyRef::from_ber(bytes).unwrap().value(), &bytes[2..]);
}

#[test]
fn nested_indefinite_lengths() {
    for depth in [1, 2, 64] {
        assert_length(&nested_sequences(depth));
    }
}

#[test]
fn indefinite_length_nesting_limit() {
    let bytes = nested_sequences(65);
    let err = AnyRef::from_ber(&bytes).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::NestingDepth);
}

#[test]
fn deeply_nested_indefinite_lengths() {
    let bytes = nested_sequences(100_000);
    let err = AnyRef::from_ber(&bytes).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::NestingDepth);

    #[cfg(feature = "alloc")]
    assert_eq!(
        der::asn1::Any::from_ber(&bytes).unwrap_err().kind(),
        ErrorKind::NestingDepth
    );
}

#[test]
fn indefinite_length_siblings() {
    let mut bytes = vec![0x30, 0x80];
    for _ in 0..100 {
        bytes.extend_from_slice(&nested_sequences(1));
    }
    bytes.extend_from_slice(&[0, 0]);
    assert_length(&bytes);
}

#[test]
fn mixed_definite_and_indefinite_lengths() {
    // Definite-length contents, including apparent headers and EOCs, are opaque.
    assert_length(&hex!(
        "30 80 30 02 05 00 04 04 30 80 00 00 30 80 05 00 00 00 05 00 00 00"
    ));

    // Constructed OCTET STRING: use the encoded constructed bit, not `Tag::is_constructed`.
    assert_length(&hex!("30 80 24 80 04 02 00 00 00 00 00 00"));

    // A constructed context-specific tag in long form.
    assert_length(&hex!("30 80 BF 81 00 80 05 00 00 00 00 00"));
}

#[test]
fn primitive_indefinite_lengths() {
    for bytes in [
        hex!("04 80 00 00").as_slice(),
        &hex!("30 80 04 80 00 00 00 00"),
        &hex!("30 80 9F 81 00 80 00 00 00 00"),
    ] {
        assert_eq!(
            AnyRef::from_ber(bytes).unwrap_err().kind(),
            ErrorKind::IndefiniteLength
        );
    }
}

#[test]
fn malformed_indefinite_lengths() {
    for bytes in [
        hex!("30 80").as_slice(),
        &hex!("30 80 00"),
        &hex!("30 80 30 80 00 00"),
        &hex!("30 80 04"),
        &hex!("30 80 04 05 00 00"),
    ] {
        assert!(matches!(
            AnyRef::from_ber(bytes).unwrap_err().kind(),
            ErrorKind::Incomplete { .. }
        ));
    }

    assert_eq!(
        AnyRef::from_ber(&hex!("30 80 00 01")).unwrap_err().kind(),
        ErrorKind::IndefiniteLength
    );
    assert_eq!(
        AnyRef::from_ber(&hex!("30 80 04 85")).unwrap_err().kind(),
        ErrorKind::Length {
            tag: Tag::OctetString
        }
    );
}

#[test]
fn der_rejects_indefinite_lengths() {
    assert_eq!(
        AnyRef::from_der(&nested_sequences(1)).unwrap_err().kind(),
        ErrorKind::IndefiniteLength
    );
}
