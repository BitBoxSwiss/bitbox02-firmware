//! ASN.1 `OPTIONAL` as mapped to Rust's `Option` type

use crate::{Choice, Decode, DerOrd, Encode, Error, Length, Reader, Tag, Writer};
use core::cmp::Ordering;

impl<'a, T> Decode<'a> for Option<T>
where
    T: Choice<'a>, // NOTE: all `Decode + Tagged` types receive a blanket `Choice` impl
{
    type Error = T::Error;

    fn decode<R: Reader<'a>>(reader: &mut R) -> Result<Option<T>, Self::Error> {
        if reader.is_finished() {
            return Ok(None);
        }

        if T::can_decode(Tag::peek(reader)?) {
            return T::decode(reader).map(Some);
        }

        Ok(None)
    }
}

impl<T> DerOrd for Option<T>
where
    T: DerOrd,
{
    fn der_cmp(&self, other: &Self) -> Result<Ordering, Error> {
        match (self, other) {
            (Some(a), Some(b)) => a.der_cmp(b),
            (Some(_), None) => Ok(Ordering::Greater),
            (None, Some(_)) => Ok(Ordering::Less),
            // Both absent: neither contributes any octets.
            (None, None) => Ok(Ordering::Equal),
        }
    }
}

impl<T> Encode for Option<T>
where
    T: Encode,
{
    fn encoded_len(&self) -> Result<Length, Error> {
        (&self).encoded_len()
    }

    fn encode(&self, writer: &mut impl Writer) -> Result<(), Error> {
        (&self).encode(writer)
    }
}

impl<T> Encode for &Option<T>
where
    T: Encode,
{
    fn encoded_len(&self) -> Result<Length, Error> {
        match self {
            Some(encodable) => encodable.encoded_len(),
            None => Ok(0u8.into()),
        }
    }

    fn encode(&self, writer: &mut impl Writer) -> Result<(), Error> {
        match self {
            Some(encodable) => encodable.encode(writer),
            None => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::DerOrd;
    use core::cmp::Ordering;

    #[test]
    fn der_cmp() {
        let none: Option<u8> = None;
        assert_eq!(none.der_cmp(&None), Ok(Ordering::Equal));
        assert_eq!(none.der_cmp(&Some(0)), Ok(Ordering::Less));
        assert_eq!(Some(0u8).der_cmp(&None), Ok(Ordering::Greater));
        assert_eq!(Some(1u8).der_cmp(&Some(2)), Ok(Ordering::Less));
        assert_eq!(Some(2u8).der_cmp(&Some(2)), Ok(Ordering::Equal));
    }
}
