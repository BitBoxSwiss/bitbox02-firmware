// SPDX-License-Identifier: CC0-1.0

//! Hex encode/decode a type that wraps an array - using an implementation of the `DisplayHex` trait.
//!
//! For an example using the standard library `fmt` traits see `./wrap_array_fmt_traits.rs`.

use hex_conservative::display::DisplayArray;
use hex_conservative::{DisplayHex, FromHex, HexToArrayError, HexToBytesError};

fn main() {
    let hex = "00000000cafebabedeadbeefcafebabedeadbeefcafebabedeadbeefcafebabe";
    println!("\nParse from hex: {}\n", hex);

    let array = <[u8; 32] as FromHex>::from_hex(hex).expect("failed to parse array");
    let wrap = Wrap::from_hex(hex).expect("failed to parse wrapped array");

    println!("Print an array using traits from the standard libraries `fmt` module along with the provided implementation of `DisplayHex`:\n");
    println!("LowerHex: {:x}", array.as_hex());
    println!("UpperHex: {:X}", array.as_hex());
    println!("Display: {}", array.as_hex());
    println!("Debug: {:?}", array.as_hex());
    println!("Debug pretty: {:#?}", array.as_hex());

    println!("\n");

    println!(
        "Print the wrapped array directly using traits from the standard libraries `fmt` module:\n"
    );
    println!("LowerHex: {:x}", wrap.as_hex());
    println!("UpperHex: {:X}", wrap.as_hex());
    println!("Display: {}", wrap.as_hex());
    println!("Debug: {:?}", wrap.as_hex());
    println!("Debug pretty: {:#?}", wrap.as_hex());

    #[cfg(feature = "alloc")]
    {
        // We cannot call `to_string` on the wrapped type to allocate a string, if you wish to
        // use that trait method see `./wrap_array_fmt_traits.rs`.
        let array_hex = array.to_lower_hex_string();
        let wrap_hex = wrap.to_lower_hex_string();
        assert_eq!(array_hex, wrap_hex);
    }
}

pub struct Wrap([u8; 32]);

impl FromHex for Wrap {
    type Err = HexToArrayError;

    fn from_byte_iter<I>(iter: I) -> Result<Self, Self::Err>
    where
        I: Iterator<Item = Result<u8, HexToBytesError>> + ExactSizeIterator + DoubleEndedIterator,
    {
        Ok(Self(FromHex::from_byte_iter(iter)?))
    }
}

/// Use `DisplayArray` to display the `Wrap` type.
///
// TODO: The following statement is not true because `DislpayByteSlice` does not prefix pretty debug with 0x - fix this.
/// Alternately use `DisplayByteSlice` which will achieve exactly the same thing.
// impl<'a> DisplayHex for &'a Wrap {
//     type Display = DisplayByteSlice<'a>;
//     fn as_hex(self) -> Self::Display { self.0.as_ref().as_hex() }
//     fn hex_reserve_suggestion(self) -> usize { self.0.as_ref().hex_reserve_suggestion() }
// }
impl<'a> DisplayHex for &'a Wrap {
    type Display = DisplayArray<core::slice::Iter<'a, u8>, [u8; 64]>;
    fn as_hex(self) -> Self::Display { DisplayArray::new(self.0.iter()) }
    fn hex_reserve_suggestion(self) -> usize { 64 }
}
