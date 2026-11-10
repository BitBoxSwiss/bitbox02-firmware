//! ASN.1 `BOOLEAN` support.

use crate::{
    DecodeValue, EncodeValue, Error, ErrorKind, FixedTag, Header, Length, Reader, Result, Tag,
    Writer, asn1::AnyRef, ord::OrdIsValueOrd,
};

/// Byte used to encode `true` in ASN.1 DER. From X.690 Section 11.1:
///
/// > If the encoding represents the boolean value TRUE, its single contents
/// > octet shall have all eight bits set to one.
const TRUE_OCTET: u8 = 0b11111111;

/// Byte used to encode `false` in ASN.1 DER.
const FALSE_OCTET: u8 = 0b00000000;

impl<'a> DecodeValue<'a> for bool {
    type Error = Error;

    fn decode_value<R: Reader<'a>>(reader: &mut R, header: Header) -> Result<Self> {
        if header.length() != Length::ONE {
            return Err(reader.error(ErrorKind::Length { tag: Self::TAG }));
        }

        match reader.read_byte()? {
            FALSE_OCTET => Ok(false),
            TRUE_OCTET => Ok(true),
            _ => Err(reader.error(Self::TAG.non_canonical_error())),
        }
    }
}

impl EncodeValue for bool {
    fn value_len(&self) -> Result<Length> {
        Ok(Length::ONE)
    }

    fn encode_value(&self, writer: &mut impl Writer) -> Result<()> {
        writer.write_byte(if *self { TRUE_OCTET } else { FALSE_OCTET })
    }
}

impl FixedTag for bool {
    const TAG: Tag = Tag::Boolean;
}

impl OrdIsValueOrd for bool {}

impl TryFrom<AnyRef<'_>> for bool {
    type Error = Error;

    fn try_from(any: AnyRef<'_>) -> Result<bool> {
        any.decode_as()
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::bool_assert_comparison)]
mod tests {
    use crate::{Decode, Encode, asn1::AnyRef};

    #[test]
    fn try_from_any() {
        let true_value = AnyRef::from_der(&[0x01, 0x01, 0xFF]).unwrap();
        let false_value = AnyRef::from_der(&[0x01, 0x01, 0x00]).unwrap();
        assert_eq!(bool::try_from(true_value).unwrap(), true);
        assert_eq!(bool::try_from(false_value).unwrap(), false);
    }

    #[test]
    fn try_from_any_rejects_invalid_input() {
        for bytes in [
            &[0x02, 0x01, 0x00][..],   // Wrong tag: INTEGER.
            &[0x01, 0x00],             // Empty BOOLEAN.
            &[0x01, 0x02, 0x00, 0x00], // Overlong BOOLEAN.
            &[0x01, 0x01, 0x01],       // Noncanonical TRUE.
        ] {
            let any = AnyRef::from_der(bytes).unwrap();
            assert!(bool::try_from(any).is_err());
        }
    }

    #[test]
    fn decode() {
        assert_eq!(true, bool::from_der(&[0x01, 0x01, 0xFF]).unwrap());
        assert_eq!(false, bool::from_der(&[0x01, 0x01, 0x00]).unwrap());
    }

    #[test]
    fn encode() {
        let mut buffer = [0u8; 3];
        assert_eq!(
            &[0x01, 0x01, 0xFF],
            true.encode_to_slice(&mut buffer).unwrap()
        );
        assert_eq!(
            &[0x01, 0x01, 0x00],
            false.encode_to_slice(&mut buffer).unwrap()
        );
    }

    #[test]
    fn reject_non_canonical() {
        assert!(bool::from_der(&[0x01, 0x01, 0x01]).is_err());
    }
}
