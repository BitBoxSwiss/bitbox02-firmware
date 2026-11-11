//! `SetOf` tests.

#![cfg(all(any(unix, windows), feature = "alloc", feature = "heapless"))]
#![allow(clippy::std_instead_of_alloc)]

use der::{DerOrd, asn1::SetOfVec};
use proptest::{prelude::*, string::*};
use std::collections::BTreeSet;

proptest! {
    #[test]
    fn sort_equiv(bytes in bytes_regex(".{0,64}").unwrap()) {
        let mut uniq = BTreeSet::new();

        // Ensure there are no duplicates
        if bytes.iter().copied().all(move |x| uniq.insert(x)) {
            let mut expected = bytes.clone();
            expected.sort_by(|a, b| a.der_cmp(b).unwrap());

            let set = SetOfVec::try_from(bytes).unwrap();
            prop_assert_eq!(expected.as_slice(), set.as_slice());
        }
    }
}

/// X.690 11.6: `SET OF` components are ordered by their encodings as octet strings, so the
/// constructed bit (0x20) of the identifier octet orders before the tag number.
mod encoding_order {
    use der::{
        Decode, Encode,
        asn1::{Any, AnyRef, SetOfRef, SetOfVec},
    };
    use hex_literal::hex;

    /// `SET OF { PrintableString "a", SEQUENCE {} }`: 0x13 < 0x30.
    const SORTED: &[u8] = &hex!("3105 130161 3000");

    #[test]
    fn setofvec_encodes_in_octet_order() {
        let mut set = SetOfVec::new();
        set.insert(Any::from_der(&hex!("3000")).unwrap()).unwrap();
        set.insert(Any::from_der(&hex!("130161")).unwrap()).unwrap();
        assert_eq!(set.to_der().unwrap(), SORTED);
    }

    #[test]
    fn setofref_accepts_octet_order() {
        let set = SetOfRef::<AnyRef<'_>>::from_der(SORTED).unwrap();
        assert_eq!(set.to_der().unwrap(), SORTED);
    }

    #[test]
    fn constructed_bit_before_tag_number() {
        // [APPLICATION 29] primitive (0x5d) before [APPLICATION 0] constructed (0x60)
        let mut set = SetOfVec::new();
        set.insert(Any::from_der(&hex!("6000")).unwrap()).unwrap();
        set.insert(Any::from_der(&hex!("5d00")).unwrap()).unwrap();
        assert_eq!(set.to_der().unwrap(), hex!("3104 5d00 6000"));
    }
}

/// Set ordering tests.
#[cfg(all(feature = "derive", feature = "oid"))]
mod ordering {
    use der::{
        Decode, Sequence, ValueOrd,
        asn1::{AnyRef, ObjectIdentifier, SetOf, SetOfVec},
    };
    use hex_literal::hex;

    /// X.501 `AttributeTypeAndValue`
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Sequence, ValueOrd)]
    pub struct AttributeTypeAndValue<'a> {
        pub oid: ObjectIdentifier,
        pub value: AnyRef<'a>,
    }

    const OUT_OF_ORDER_RDN_EXAMPLE: &[u8] =
        &hex!("311F301106035504030C0A4A4F484E20534D495448300A060355040A0C03313233");

    /// For compatibility reasons, we allow non-canonical DER with out-of-order
    /// sets in order to match the behavior of other implementations.
    #[test]
    fn allow_out_of_order_setof() {
        assert!(SetOf::<AttributeTypeAndValue<'_>, 2>::from_der(OUT_OF_ORDER_RDN_EXAMPLE).is_ok());
    }

    /// Same as above, with `SetOfVec` instead of `SetOf`.
    #[test]
    fn allow_out_of_order_setofvec() {
        assert!(SetOfVec::<AttributeTypeAndValue<'_>>::from_der(OUT_OF_ORDER_RDN_EXAMPLE).is_ok());
    }

    /// Test to ensure ordering is handled correctly.
    #[test]
    fn ordering_regression() {
        let der_bytes = hex!(
            "3139301906035504030C12546573742055736572393031353734333830301C060A0992268993F22C640101130E3437303031303030303134373333"
        );
        let set = SetOf::<AttributeTypeAndValue<'_>, 3>::from_der(&der_bytes).unwrap();
        let attr1 = set.get(0).unwrap();
        assert_eq!(ObjectIdentifier::new("2.5.4.3").unwrap(), attr1.oid);
    }
}

/// X.690 11.6: `SET OF UTCTime` is ordered by the `YYMMDDHHMMSSZ` encodings, so a 20xx time
/// (`00..49`) sorts before a 19xx time (`50..99`).
mod utctime_order {
    use der::{
        Decode, Encode,
        asn1::{SetOfVec, UtcTime},
    };
    use hex_literal::hex;

    const Y1999: [u8; 15] = hex!("170d 3939313233313233353935395a");
    const Y2000: [u8; 15] = hex!("170d 3030303130313030303030305a");

    #[test]
    fn setofvec_utctime_encodes_in_octet_order() {
        let mut set = SetOfVec::new();
        set.insert(UtcTime::from_der(&Y1999).unwrap()).unwrap();
        set.insert(UtcTime::from_der(&Y2000).unwrap()).unwrap();
        let mut want = hex!("311e").to_vec();
        want.extend_from_slice(&Y2000);
        want.extend_from_slice(&Y1999);
        assert_eq!(set.to_der().unwrap(), want);
    }
}
