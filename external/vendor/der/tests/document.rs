//! `Document` and `SecretDocument` tests.

#![cfg(feature = "alloc")]
#![allow(missing_docs)]

use der::{Decode, Document, ErrorKind, Tag};

/// `Document` holds a DER-encoded `SEQUENCE`. Every way of building one from bytes must
/// enforce that, so the same bytes are accepted or rejected regardless of the constructor
/// (and a document written with `to_pem` can always be read back with `from_pem`).
#[test]
fn document_requires_sequence() {
    let integer: &[u8] = &[0x02, 0x01, 0x00];
    let err = Document::from_der(integer).unwrap_err();
    assert!(matches!(
        err.kind(),
        ErrorKind::TagUnexpected {
            expected: Some(Tag::Sequence),
            ..
        }
    ));
    assert!(Document::try_from(integer).is_err());
    assert!(Document::try_from(integer.to_vec()).is_err());

    let sequence: &[u8] = &[0x30, 0x03, 0x02, 0x01, 0x00];
    let doc = Document::from_der(sequence).unwrap();
    assert_eq!(doc.as_bytes(), sequence);
    assert_eq!(Document::try_from(sequence).unwrap(), doc);
    assert_eq!(Document::try_from(sequence.to_vec()).unwrap(), doc);
}
