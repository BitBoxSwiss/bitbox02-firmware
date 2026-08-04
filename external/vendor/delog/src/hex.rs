//! Convenient `Display` and other traits for binary data.
//!
//! Standard Rust uses the `fmt::UpperHex` and `LowerHex` traits to implement hexadecimal
//! representations for use in format strings. For instance,
//! `format_args!("{:02X?}", &[7, 0xA1, 0xFF])` produces the string `"[07, A1, FF]"`.
//!
//! ```
//! assert_eq!(format!("{:02X?}", [7u8, 0xA1, 0xFF].as_ref()), "[07, A1, FF]");
//! ```
//!
//! However, this only works for the `Debug` trait, not `Display`; needs extra dancing
//! to pad with leading zeros, and is not very compact when debugging binary data formats.
//!
//! The idea of this module is to generate newtypes around byte arrays/slices, and implement the `fmt` traits on these.
//! In release builds, this is all compiled out and translates to direct instructions for the formatting machinery.
//!
//! The ide to implement truncated outputs comes from the `hex_fmt` library, which we refer to
//! for additional newtypes that can be used.
//!
//! ```
//! use delog::{hex_str, hexstr};
//!
//! let data = &[7u8, 0xA1, 255, 0xC7];
//!
//! assert_eq!(format!("{}", hexstr!(data)), "07A1FFC7");
//! assert_eq!(format!("{:x}", hexstr!(data)), "07a1ffc7");
//! assert_eq!(format!("{:2x}", hex_str!(data)), "07..c7");
//! assert_eq!(format!("{:<3x}", hex_str!(data)), "07 a1 ff..");
//! assert_eq!(format!("{:>3x}", hex_str!(data)), "..a1 ff c7");
//! assert_eq!(format!("{}", hex_str!(data, 2)), "07A1 FFC7");
//! assert_eq!(format!("{:<3}", hex_str!(data, sep: "|")), "07|A1|FF..");
//! ```

use core::fmt;
use core::marker::PhantomData;

///// re-export from `hex_fmt`
/////
//pub use hex_fmt::HexFmt;
///// re-export from `hex_fmt`
/////
//pub use hex_fmt::HexList;

/// A type that specifies an unsigned integer.
///
/// We use this instead of `typenum` as the latter currently lacks
/// a mapping from `usize` to the associated type.
pub trait Unsigned {
    /// The actual number.
    const N: usize;
}

/// Sorry little replacement for the missing int to Unsigned type map in `typenum`.
#[macro_export]
#[doc(hidden)]
macro_rules! typeint {
    ($name:ident, $n:expr) => {
        /// A type that represents the integer `N`.
        pub struct $name;
        impl $crate::hex::Unsigned for $name {
            const N: usize = $n;
        }
    };
}

/// A type that represents the integer `1`.
pub struct U1;
impl Unsigned for U1 {
    const N: usize = 1;
}

// /// A type representing the number 1.
// pub struct U1 {}
// impl Unsigned for U1 {
//     const N: usize = 1;
// }

/// A type that specifies a separator str.
pub trait Separator {
    /// The actual separator str.
    const SEPARATOR: &'static str;
}

#[macro_export]
#[doc(hidden)]
macro_rules! typesep {
    ($name:ident, $s:expr) => {
        /// A type that represents the integer `N`.
        pub struct $name;
        impl $crate::hex::Separator for $name {
            const SEPARATOR: &'static str = $s;
        }
    };
}

// /// New approach.
// pub trait HexStrTrait {
//     const BYTES_PER_BLOCK: usize;
//     const SEPARATOR: &'static str;
//     fn bytes(&self) -> &[u8];
// }

/// Zero-sized wrapper newtype, allowing grouping bytes in blocks of N hexadecimals
/// during formatting.
///
/// Use the method with the same name to construct this from your byte array or slice,
/// or preferrably the `hex_str!` or `hexstr!` macro.
pub struct HexStr<'a, T: ?Sized, U, S>
where
    U: Unsigned,
    S: Separator,
{
    /// The value to be formatted.
    pub value: &'a T,
    _bytes_per_block: PhantomData<U>,
    _separator: PhantomData<S>,
}

#[macro_export]
/// Compactly format byte arrays and slices as hexadecimals.
///
/// The second parameter refers to the number of bytes in a block (separated by spaces).
///
/// ```
/// use delog::hex_str;
/// let four_bytes = &[7u8, 0xA1, 255, 0xC7];
/// assert_eq!(format!("{:x}", hex_str!(four_bytes)), "07 a1 ff c7");
/// assert_eq!(format!("{}", hex_str!(four_bytes, 2)), "07A1 FFC7");
/// assert_eq!(format!("{}", hex_str!(four_bytes, 2, sep: "|")), "07A1|FFC7");
/// assert_eq!(format!("{}", hex_str!(four_bytes, 3)), "07A1FF C7");
/// ```
macro_rules! hex_str {
    ($array:expr) => { $crate::hex_str!($array, 1, sep: " ") };
    ($array:expr, sep: $separator:expr) => { $crate::hex_str!($array, 1, sep: $separator) };
    ($array:expr, $n:tt) => { $crate::hex_str!($array, $n, sep: " ") };
    ($array:expr, $n:tt, sep: $separator:expr) => {{
        $crate::typesep!(Separator, $separator);
        $crate::typeint!(Number, $n);
        $crate::hex::HexStr::<_, Number, Separator>($array)
    }};
}

#[macro_export]
/// More compactly format byte arrays and slices as hexadecimals.
///
/// ```
/// use delog::hexstr;
/// let four_bytes = &[7u8, 0xA1, 255, 0xC7];
/// assert_eq!(format!("{}", hexstr!(four_bytes)), "07A1FFC7");
/// assert_eq!(format!("{:x}", hexstr!(four_bytes)), "07a1ffc7");
/// ```
macro_rules! hexstr {
    ($array:expr) => {
        $crate::hex_str!($array, sep: "")
    }
}

#[allow(non_snake_case)]
/// Explicitly construct a newtype to format with.
///
/// The first generic parameter specifies the block size in bytes, the second parameter
/// the separator string.
///
/// This function is just here for documentation of what the `hex_str!` macro
/// does internally (using the undocumented `typeint!` and `typesep!` macros).
///
/// ```
/// use delog::hex::{HexStr, Separator, Unsigned};
///
/// // typeint!(U3, 3);
/// struct U3 {}
/// impl Unsigned for U3 {
///     const N: usize = 3;
/// }
///
/// // typesep!(Pipe, "|");
/// struct Pipe {}
/// impl Separator for Pipe {
///     const SEPARATOR: &'static str  = "|";
/// }
///
/// let four_bytes = &[7u8, 0xA1, 255, 0xC7];
/// let hex_str = HexStr::<_, U3, Pipe>(four_bytes);
///
/// assert_eq!(format!("{}", hex_str), "07A1FF|C7");
/// ```
pub fn HexStr<T: ?Sized, U: Unsigned, S: Separator>(value: &T) -> HexStr<'_, T, U, S> {
    HexStr {
        value,
        _bytes_per_block: PhantomData,
        _separator: PhantomData,
    }
}

impl<T: ?Sized, S, U> fmt::Debug for HexStr<'_, T, U, S>
where
    T: AsRef<[u8]>,
    U: Unsigned,
    S: Separator,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt::Display::fmt(self, f)
    }
}

impl<T: ?Sized, U, S> fmt::Display for HexStr<'_, T, U, S>
where
    T: AsRef<[u8]>,
    U: Unsigned,
    S: Separator,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt::UpperHex::fmt(self, f)
    }
}

macro_rules! implement {
    ($Trait:ident, $padded_formatter:expr) => {
        impl<'a, T: ?Sized, U, S> fmt::$Trait for HexStr<'a, T, U, S>
        where
            T: AsRef<[u8]>,
            U: Unsigned,
            S: Separator,
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
                use core::fmt::{self, Alignment::*};
                let max_bytes = f.width().unwrap_or(usize::MAX);
                let bytes = self.value.as_ref();

                #[inline]
                fn nontruncated_fmt(
                    bytes: &[u8],
                    f: &mut fmt::Formatter<'_>,
                    chunk_size: usize,
                    separator: &str,
                ) -> Result<(), fmt::Error> {
                    let mut first = true;
                    for entry in bytes.chunks(chunk_size) {
                        if first {
                            first = false;
                        } else {
                            f.write_str(separator)?;
                        }
                        for byte in entry.iter() {
                            write!(f, $padded_formatter, byte)?;
                        }
                    }
                    Ok(())
                }

                // const ELLIPSIS: &str = "…";
                const ELLIPSIS: &str = "..";

                let chunk_size = U::N;
                let separator = S::SEPARATOR;

                if bytes.len() <= max_bytes {
                    nontruncated_fmt(bytes, f, chunk_size, separator)
                } else {
                    let align = f.align().unwrap_or(Center);
                    let (left, right) = match align {
                        Left => (max_bytes, 0),
                        Center => (max_bytes - max_bytes / 2, max_bytes / 2),
                        Right => (0, max_bytes),
                    };
                    nontruncated_fmt(&bytes[..left], f, chunk_size, separator)?;
                    // if left > 0 {
                    //     f.write_str(separator)?;
                    // }
                    f.write_str(ELLIPSIS)?;
                    // if right > 0 {
                    //     f.write_str(separator)?;
                    // }
                    nontruncated_fmt(&bytes[bytes.len() - right..], f, chunk_size, separator)?;
                    Ok(())
                }
            }
        }
    };
}

implement!(LowerHex, "{:02x}");
implement!(UpperHex, "{:02X}");

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hex_str() {
        let buf = [1u8, 2, 3, 0xA1, 0xB7, 0xFF, 0x3];
        insta::assert_debug_snapshot!(format_args!("'{}'", hex_str!(&buf)));
        insta::assert_debug_snapshot!(format_args!("'{}'", hex_str!(&buf, 2)));
        insta::assert_debug_snapshot!(format_args!("'{:x}'", hex_str!(&buf, 2)));
        insta::assert_debug_snapshot!(format_args!("'{}'", hex_str!(&buf, 4)));
        insta::assert_debug_snapshot!(format_args!("'{}'", hex_str!(&buf[..], 4)));
        insta::assert_debug_snapshot!(format_args!("'{}'", hex_str!(&buf, 4)));
        insta::assert_debug_snapshot!(format_args!("'{}'", hex_str!(&buf, 4)));
    }

    #[test]
    fn test_custom_hex_str() {
        let buf = [1u8, 2, 3, 0xA1, 0xB7, 0xFF, 0x3];
        typeint!(U3, 3);
        typesep!(Space, " ");
        insta::assert_debug_snapshot!(format_args!("'{:X}'", HexStr::<_, U3, Space>(&buf),));
    }
}
