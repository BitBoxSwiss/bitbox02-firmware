//! Derived `ValueOrd` on structs with `OPTIONAL` fields.

#![cfg(all(feature = "derive", feature = "alloc", feature = "heapless"))]

use core::cmp::Ordering;
use der::{
    Decode, DerOrd, Encode, Sequence, ValueOrd,
    asn1::{SetOfRef, SetOfVec},
};
use hex_literal::hex;

/// `SEQUENCE { opt [0] INTEGER OPTIONAL, n INTEGER }`
#[derive(Clone, Debug, Eq, PartialEq, Sequence, ValueOrd)]
struct S {
    #[asn1(context_specific = "0", optional = "true")]
    opt: Option<u8>,
    n: u8,
}

const N1: S = S { opt: None, n: 1 };
const N2: S = S { opt: None, n: 2 };

/// An absent `OPTIONAL` field on both sides must not decide the order: the fields after it do.
#[test]
fn absent_optional_field_is_equal() {
    assert_eq!(N1.value_cmp(&N2), Ok(Ordering::Less));
    assert_eq!(N2.value_cmp(&N1), Ok(Ordering::Greater));
    assert_eq!(N1.der_cmp(&N1), Ok(Ordering::Equal));
}

/// X.690 11.6: `30 03 02 01 01` sorts before `30 03 02 01 02`.
#[test]
fn set_of_with_absent_optional_field() {
    let mut set = SetOfVec::new();
    set.insert(N2.clone()).unwrap();
    set.insert(N1.clone()).unwrap();
    assert_eq!(set.to_der().unwrap(), hex!("310a 3003020101 3003020102"));

    // the reverse order is not DER
    let wrong = hex!("310a 3003020102 3003020101");
    assert!(SetOfRef::<S>::from_der(&wrong).is_err());
}
