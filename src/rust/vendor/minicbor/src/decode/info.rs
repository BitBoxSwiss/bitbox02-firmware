//! Introspection utilities.

use crate::{BYTES, TEXT, ARRAY, MAP, SIMPLE, TAGGED, SIGNED, UNSIGNED, Decoder, data::Type};
use super::{decoder::{info_of, type_of}, Error};

/// Information about a CBOR item size.
///
/// # Usage example
///
/// ```rust
/// use minicbor::decode::info::Size;
///
/// let val  = vec![1, 2, 3, 4, 5];
/// let cbor = minicbor::to_vec(val)?;
/// let hlen = Size::head(cbor[0])?;
/// let size = Size::tail(&cbor[.. hlen])?;
/// assert_eq!(Size::Items(5), size);
/// # Ok::<_, Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Size {
    /// The item consists only of the head.
    Head,
    /// The item is some text or byte string with the given number of bytes.
    Bytes(u64),
    /// The item is an array or map with the given number of items.
    Items(u64),
    /// The item is an indefinite value.
    Indef
}

impl Size {
    /// Given the first byte, derive the length of the item head.
    pub fn head(fst: u8) -> Result<usize, Error> {
        match info_of(fst) {
            0 ..= 0x17 => Ok(1),
            0x18 => Ok(2),
            0x19 => Ok(3),
            0x1a => Ok(5),
            0x1b => Ok(9),
            0x1f => match type_of(fst) {
                BYTES | TEXT | ARRAY | MAP | SIMPLE => Ok(1),
                _ => Err(Error::message("invalid data item head"))
            }
            _ => Err(Error::message("invalid data item head"))
        }
    }

    /// Given the item head, derive the CBOR item's size information.
    pub fn tail(head: &[u8]) -> Result<Self, Error> {
        let fst = head.first().copied().ok_or_else(Error::end_of_input)?;
        match type_of(fst) {
            UNSIGNED | SIGNED | TAGGED | SIMPLE => Ok(Self::Head),
            BYTES | TEXT => match info_of(fst) {
                0x1f => Ok(Self::Indef),
                info => {
                    let mut d = Decoder::new(&head[1 ..]);
                    let p = d.position();
                    let n = d.unsigned(info, p)?;
                    Ok(Self::Bytes(n))
                }
            }
            ARRAY | MAP => match info_of(fst) {
                0x1f => Ok(Self::Indef),
                info => {
                    let mut d = Decoder::new(&head[1 ..]);
                    let p = d.position();
                    let n = d.unsigned(info, p)?;
                    Ok(Self::Items(n))
                }
            }
            n => {
                let t = Type::Unknown(n);
                Err(Error::type_mismatch(t).at(0).with_message("unknown cbor type"))
            }
        }
    }
}

