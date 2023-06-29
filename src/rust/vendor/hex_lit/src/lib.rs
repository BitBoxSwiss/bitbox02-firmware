//! Hex literals without proc macros.
//! 
//! This crate implements minimalistic hex literal macros without use of proc macros.
//! The advantages are much faster compile times, ability to work with non-literal const values and
//! easier auditing.
//! However, because of the use of `const fn` the crate has some limitations depending on the Rust
//! version.
//!
//! Either way, the resulting type is a byte array (`[u8; N]`) that doesn't force you to write down
//! its length. This is already very useful since the compiler can prove the length and you avoid
//! runtime allocations.
//!
//! The crate is `no_std` and does **not** require an allocator.
//!
//! ## Usage
//!
//! Just pass a `&str` *constant* (usually a literal) into the [`hex`] macro.
//!
//! Example
//!
//! ```rust
//! use hex_lit::hex;
//!
//! let array = hex!("2a15ff");
//! assert_eq!(&array, &[42, 21, 255]);
//!
//! ```
//!
//! The input MUST NOT contain any spaces or other separators and it MUST have even length.
//! Note that you can still separate long strings into chunks using the [`concat`] macro:
//!
//! ```rust
//! use hex_lit::hex;
//!
//! let array = hex!(concat!(
//!     "0000002a000000",
//!     "ffffffffffffff",
//! ));
//! assert_eq!(&array, &[0, 0, 0, 42, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255]);
//!
//! ```
//!
//! ## Features depending on Rust version
//!
//! * 1.41.1+ - the MSRV, use in const contexts is impossible, only the [`hex!`] macro is available.
//! * 1.46.0+ - usage in const contexts is available and (regardless of cargo features) correctness
//!             of input is checked at compile time. 
//! * 1.57+ - nicer error messages for bad inputs (regardless of cargo features)
//!
//! ## Cargo features
//!
//! * `rust_v_1_46` - acknowledges bumping MSRV to 1.46+ and enables usage in const context.
//!
//! Bumping MSRV is intentionally explicit.
//!
//! Because of improved input checking it is recommended to use Rust 1.46+, prefereably 1.57+ in CI
//! even if your targeted MSRV is lower.
//!
//! [`concat`]: core::concat

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]

// makes the passed function optionally const depending on rust version
macro_rules! maybe_const {
    ($($fun:tt)*) => {
        #[cfg(rust_v_1_46)]
        #[track_caller]
        #[inline]
        const $($fun)*

        #[cfg(not(rust_v_1_46))]
        $($fun)*
    };
}

// makes the passed function public and optionally const depending on rust version
macro_rules! pub_maybe_const {
    ($($fun:tt)*) => {
        // DO NOT CALL THIS OUTSIDE OF THIS CRATE!!!
        #[doc(hidden)]
        #[cfg(rust_v_1_46)]
        #[track_caller]
        #[inline]
        pub const $($fun)*

        // DO NOT CALL THIS OUTSIDE OF THIS CRATE!!!
        #[doc(hidden)]
        #[cfg(not(rust_v_1_46))]
        pub $($fun)*
    }
}

// Invoked when there's an invalid digit in the string to cause an error/panic
// This version is not const because const isn't supported anyway, so we just panic
#[cfg(not(rust_v_1_46))]
fn invalid_digit(digit: u8) {
    panic!("invalid hex digit: ASCII {}", digit);
}

// This version makes an out-of-bounds accesses to an array to trigger compilation failure.
#[cfg(all(not(rust_v_1_57), rust_v_1_46))]
#[track_caller]
const fn invalid_digit(digit: u8) {
    let digit = digit as usize;
    #[allow(unknown_lints)]
    #[allow(unconditional_panic)]
    // We add 10000 to ensure it will panic for any digit, we want to use digit in the expression
    // to display the number even though the message is messy.
    let _invalid_digit = [(); 0][digit + 10000];
}

// This version panics with a nice custom message
#[cfg(rust_v_1_57)]
#[track_caller]
const fn invalid_digit(digit: u8) {
    // custom message formatting because of rust limiation
    let mut buf = [b'i', b'n', b'v', b'a', b'l', b'i', b'd', b' ', b'h', b'e', b'x', b' ', b'd', b'i', b'g', b'i', b't', b':', b' ', b'A', b'S', b'C', b'I', b'I', b' ', b' ', b' ', b' '];
    if digit >= 100 {
        buf[buf.len() - 3] = digit / 100 + b'0';
    }
    if digit >= 10 {
        buf[buf.len() - 2] = (digit % 100) / 10 + b'0';
    }
    buf[buf.len() - 1] = digit % 10 + b'0';
    let message = unsafe { core::str::from_utf8_unchecked(&buf) };
    panic!("{}", message);
}

// Decodes a single hex digit (char) into an integer 0-15
maybe_const! {
    fn decode_digit(digit: u8) -> u8 {
        match digit {
            b'0'..=b'9' => digit - b'0',
            b'a'..=b'f' => digit - b'a' + 10,
            b'A'..=b'F' => digit - b'A' + 10,
            _ => {
                invalid_digit(digit);
                0
            }
        }
    }
}

// Decodes a single byte in the string at given **output** position.
pub_maybe_const! {
    fn decode_byte(hex: &str, pos: usize) -> u8 {
        let c1 = decode_digit(hex.as_bytes()[pos * 2]);
        let c2 = decode_digit(hex.as_bytes()[pos* 2 + 1]);
        c1 << 4 | c2
    }
}

// Internal macro implementation simplifies dispatch
// DO NOT CALL THIS OUTSIDE OF THIS CRATE!!!
#[doc(hidden)]
#[macro_export]
macro_rules! hex_impl {
    ($hex:expr) => {
        {
            const HEX: &str = $hex;
            // Fails if the length is not even
            const _HEX_LENGTH_MUST_BE_EVEN: () = [()][$hex.len() % 2];
            let mut out = [0u8; HEX.len() / 2];
            // mut refs are not allowed in const fns so we must have the decoding loop here
            // we minimized its size using a separate decoding function
            let mut pos = 0;
            loop {
                if pos >= out.len() {
                    break;
                }
                out[pos] = $crate::decode_byte(HEX, pos);
                pos += 1
            }
            out
        }
    }
}

// Does nothing exept for blocking usage of hex macro in const context without the rust_v_1_46
// feature enabled.
// DO NOT CALL THIS OUTSIDE OF THIS CRATE!!!
#[doc(hidden)]
#[cfg(not(feature = "rust_v_1_46"))]
pub fn msrv_opt_in() {}

// DO NOT CALL THIS OUTSIDE OF THIS CRATE!!!
#[doc(hidden)]
#[cfg(feature = "rust_v_1_46")]
pub const fn msrv_opt_in() {}

/// Creates a byte array const value from hex &str const value.
///
/// Accepts const `&str` as an input. Refer to the crate documentation to learn more.
#[cfg(not(rust_v_1_46))]
#[macro_export]
macro_rules! hex {
    ($hex:expr) => {
        $crate::hex_impl!($hex)
    }
}

/// Creates a byte array const value from hex &str const value.
///
/// Accepts const `&str` as an input. Refer to the crate documentation to learn more.
#[cfg(rust_v_1_46)]
#[macro_export]
macro_rules! hex {
    ($hex:expr) => {
        {
            // pass through const to force const eval
            const TMP: [u8; $hex.len() / 2] = $crate::hex_impl!($hex);
            // block usage in const context unless the feature is explicitly turned on
            $crate::msrv_opt_in();
            TMP
        }
    }
}

/// Creates a constant byte array of given name.
///
/// This is a convenience macro so that you don't have to write out the type. However it is
/// intentionally not public - public constants must be explicitly typed to avoid accidental change
/// of the type.
///
/// ## Example
///
/// ```
/// use hex_lit::hex_const;
/// // same as writing `const FOO: [u8; 2] = [0x00, 0xff];`
/// hex_const!(FOO = "00ff");
///
/// assert_eq!(FOO, [0, 255]);
/// ```
#[cfg(feature = "rust_v_1_46")]
#[cfg_attr(docsrs, doc(cfg(feature = "rust_v_1_46")))]
#[macro_export]
macro_rules! hex_const {
    ($name:ident = $hex:expr) => {
        const $name: [u8; $hex.len() / 2] = $crate::hex_impl!($hex);
    }
}

/// Creates a static byte array of given name.
///
/// This is a convenience macro so that you don't have to write out the type. However it is
/// intentionally not public - public statics must be explicitly typed to avoid accidental change of
/// the type.
///
/// ## Example
///
/// ```
/// use hex_lit::hex_static;
/// // same as writing `static FOO: [u8; 2] = [0x00, 0xff];`
/// hex_static!(FOO = "00ff");
///
/// assert_eq!(FOO, [0, 255]);
/// ```
#[cfg(feature = "rust_v_1_46")]
#[cfg_attr(docsrs, doc(cfg(feature = "rust_v_1_46")))]
#[macro_export]
macro_rules! hex_static {
    ($name:ident = $hex:expr) => {
        static $name: [u8; $hex.len() / 2] = $crate::hex_impl!($hex);
    }
}

#[cfg(test)]
mod tests {
    use super::hex;

    #[test]
    fn msrv_empty() {
        let arr = hex!("");
        assert_eq!(&arr, &[]);
    }

    #[test]
    fn msrv_one() {
        let arr = hex!("2a");
        assert_eq!(&arr, &[42u8]);
    }

    #[test]
    fn msrv_several() {
        let arr = hex!("2a15ff");
        assert_eq!(&arr, &[42u8, 21, 255]);
    }

    #[test]
    fn const_val_works() {
        const VAL: &str = "2a15ff";
        let arr = hex!(VAL);
        assert_eq!(&arr, &[42u8, 21, 255]);
    }

    #[test]
    #[cfg(not(rust_v_1_46))]
    #[should_panic]
    fn invalid_digit_120() {
        hex!("xx");
    }

    #[test]
    #[cfg(not(rust_v_1_46))]
    #[should_panic]
    fn invalid_digit_32() {
        hex!(" x");
    }
}
