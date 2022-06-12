//! Functions for decoding Base58 encoded strings.

use core::fmt;

#[cfg(feature = "alloc")]
use alloc::{vec, vec::Vec};

use crate::Check;
#[cfg(feature = "check")]
use crate::CHECKSUM_LEN;

use crate::Alphabet;

/// A builder for setting up the alphabet and output of a base58 decode.
///
/// See the documentation for [`bs58::decode`](crate::decode()) for a more
/// high level view of how to use this.
#[allow(missing_debug_implementations)]
pub struct DecodeBuilder<'a, I: AsRef<[u8]>> {
    input: I,
    alpha: &'a Alphabet,
    check: Check,
}

/// A specialized [`Result`](core::result::Result) type for [`bs58::decode`](module@crate::decode)
pub type Result<T> = core::result::Result<T, Error>;

/// Errors that could occur when decoding a Base58 encoded string.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Error {
    /// The output buffer was too small to contain the entire input.
    BufferTooSmall,

    /// The input contained a character that was not part of the current Base58
    /// alphabet.
    InvalidCharacter {
        /// The unexpected character.
        character: char,
        /// The (byte) index in the input string the character was at.
        index: usize,
    },

    /// The input contained a multi-byte (or non-utf8) character which is
    /// unsupported by this Base58 decoder.
    NonAsciiCharacter {
        /// The (byte) index in the input string the start of the character was
        /// at.
        index: usize,
    },

    #[cfg(feature = "check")]
    #[cfg_attr(docsrs, doc(cfg(feature = "check")))]
    /// The checksum did not match the payload bytes
    InvalidChecksum {
        ///The given checksum
        checksum: [u8; CHECKSUM_LEN],
        ///The checksum calculated for the payload
        expected_checksum: [u8; CHECKSUM_LEN],
    },

    #[cfg(feature = "check")]
    #[cfg_attr(docsrs, doc(cfg(feature = "check")))]
    /// The version did not match the payload bytes
    InvalidVersion {
        ///The given version
        ver: u8,
        ///The expected version
        expected_ver: u8,
    },

    #[cfg(feature = "check")]
    #[cfg_attr(docsrs, doc(cfg(feature = "check")))]
    ///Not enough bytes to have both a checksum and a payload (less than to CHECKSUM_LEN)
    NoChecksum,
}

impl<'a, I: AsRef<[u8]>> DecodeBuilder<'a, I> {
    /// Setup decoder for the given string using the given alphabet.
    /// Preferably use [`bs58::decode`](crate::decode()) instead of this directly.
    pub fn new(input: I, alpha: &'a Alphabet) -> DecodeBuilder<'a, I> {
        DecodeBuilder {
            input,
            alpha,
            check: Check::Disabled,
        }
    }

    /// Setup decoder for the given string using default prepared alphabet.
    pub(crate) fn from_input(input: I) -> DecodeBuilder<'static, I> {
        DecodeBuilder {
            input,
            alpha: Alphabet::DEFAULT,
            check: Check::Disabled,
        }
    }

    /// Change the alphabet that will be used for decoding.
    ///
    /// # Examples
    ///
    /// ```rust
    /// assert_eq!(
    ///     vec![0x60, 0x65, 0xe7, 0x9b, 0xba, 0x2f, 0x78],
    ///     bs58::decode("he11owor1d")
    ///         .with_alphabet(bs58::Alphabet::RIPPLE)
    ///         .into_vec()?);
    /// # Ok::<(), bs58::decode::Error>(())
    /// ```
    pub fn with_alphabet(self, alpha: &'a Alphabet) -> DecodeBuilder<'a, I> {
        DecodeBuilder { alpha, ..self }
    }

    /// Expect and check checksum using the [Base58Check][] algorithm when
    /// decoding.
    ///
    /// Optional parameter for version byte. If provided, the version byte will
    /// be used in verification.
    ///
    /// [Base58Check]: https://en.bitcoin.it/wiki/Base58Check_encoding
    ///
    /// # Examples
    ///
    /// ```rust
    /// assert_eq!(
    ///     vec![0x2d, 0x31],
    ///     bs58::decode("PWEu9GGN")
    ///         .with_check(None)
    ///         .into_vec()?);
    /// # Ok::<(), bs58::decode::Error>(())
    /// ```
    #[cfg(feature = "check")]
    #[cfg_attr(docsrs, doc(cfg(feature = "check")))]
    pub fn with_check(self, expected_ver: Option<u8>) -> DecodeBuilder<'a, I> {
        let check = Check::Enabled(expected_ver);
        DecodeBuilder { check, ..self }
    }

    /// Decode into a new vector of bytes.
    ///
    /// See the documentation for [`bs58::decode`](crate::decode()) for an
    /// explanation of the errors that may occur.
    ///
    /// # Examples
    ///
    /// ```rust
    /// assert_eq!(
    ///     vec![0x04, 0x30, 0x5e, 0x2b, 0x24, 0x73, 0xf0, 0x58],
    ///     bs58::decode("he11owor1d").into_vec()?);
    /// # Ok::<(), bs58::decode::Error>(())
    /// ```
    ///
    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(any(feature = "alloc", feature = "std"))))]
    pub fn into_vec(self) -> Result<Vec<u8>> {
        let mut output = vec![0; self.input.as_ref().len()];
        self.into(&mut output).map(|len| {
            output.truncate(len);
            output
        })
    }

    /// Decode into the given buffer.
    ///
    /// Returns the length written into the buffer, the rest of the bytes in
    /// the buffer will be untouched.
    ///
    /// See the documentation for [`bs58::decode`](crate::decode()) for an
    /// explanation of the errors that may occur.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut output = [0xFF; 10];
    /// assert_eq!(8, bs58::decode("he11owor1d").into(&mut output)?);
    /// assert_eq!(
    ///     [0x04, 0x30, 0x5e, 0x2b, 0x24, 0x73, 0xf0, 0x58, 0xFF, 0xFF],
    ///     output);
    /// # Ok::<(), bs58::decode::Error>(())
    /// ```
    pub fn into<O: AsMut<[u8]>>(self, mut output: O) -> Result<usize> {
        match self.check {
            Check::Disabled => decode_into(self.input.as_ref(), output.as_mut(), &self.alpha),
            #[cfg(feature = "check")]
            Check::Enabled(expected_ver) => decode_check_into(
                self.input.as_ref(),
                output.as_mut(),
                &self.alpha,
                expected_ver,
            ),
        }
    }
}

fn decode_into(input: &[u8], output: &mut [u8], alpha: &Alphabet) -> Result<usize> {
    let mut index = 0;
    let zero = alpha.encode[0];

    for (i, c) in input.iter().enumerate() {
        if *c > 127 {
            return Err(Error::NonAsciiCharacter { index: i });
        }

        let mut val = alpha.decode[*c as usize] as usize;
        if val == 0xFF {
            return Err(Error::InvalidCharacter {
                character: *c as char,
                index: i,
            });
        }

        for byte in &mut output[..index] {
            val += (*byte as usize) * 58;
            *byte = (val & 0xFF) as u8;
            val >>= 8;
        }

        while val > 0 {
            let byte = output.get_mut(index).ok_or(Error::BufferTooSmall)?;
            *byte = (val & 0xFF) as u8;
            index += 1;
            val >>= 8
        }
    }

    for _ in input.iter().take_while(|c| **c == zero) {
        let byte = output.get_mut(index).ok_or(Error::BufferTooSmall)?;
        *byte = 0;
        index += 1;
    }

    output[..index].reverse();
    Ok(index)
}

#[cfg(feature = "check")]
fn decode_check_into(
    input: &[u8],
    output: &mut [u8],
    alpha: &Alphabet,
    expected_ver: Option<u8>,
) -> Result<usize> {
    use sha2::{Digest, Sha256};

    let decoded_len = decode_into(input, output, alpha)?;
    if decoded_len < CHECKSUM_LEN {
        return Err(Error::NoChecksum);
    }
    let checksum_index = decoded_len - CHECKSUM_LEN;

    let expected_checksum = &output[checksum_index..decoded_len];

    let first_hash = Sha256::digest(&output[0..checksum_index]);
    let second_hash = Sha256::digest(&first_hash);
    let (checksum, _) = second_hash.split_at(CHECKSUM_LEN);

    if checksum == expected_checksum {
        if let Some(ver) = expected_ver {
            if output[0] == ver {
                Ok(checksum_index)
            } else {
                Err(Error::InvalidVersion {
                    ver: output[0],
                    expected_ver: ver,
                })
            }
        } else {
            Ok(checksum_index)
        }
    } else {
        let mut a: [u8; CHECKSUM_LEN] = Default::default();
        a.copy_from_slice(&checksum[..]);
        let mut b: [u8; CHECKSUM_LEN] = Default::default();
        b.copy_from_slice(&expected_checksum[..]);
        Err(Error::InvalidChecksum {
            checksum: a,
            expected_checksum: b,
        })
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::BufferTooSmall => write!(
                f,
                "buffer provided to decode base58 encoded string into was too small"
            ),
            Error::InvalidCharacter { character, index } => write!(
                f,
                "provided string contained invalid character {:?} at byte {}",
                character, index
            ),
            Error::NonAsciiCharacter { index } => write!(
                f,
                "provided string contained non-ascii character starting at byte {}",
                index
            ),
            #[cfg(feature = "check")]
            Error::InvalidChecksum {
                checksum,
                expected_checksum,
            } => write!(
                f,
                "invalid checksum, calculated checksum: '{:?}', expected checksum: {:?}",
                checksum, expected_checksum
            ),
            #[cfg(feature = "check")]
            Error::InvalidVersion { ver, expected_ver } => write!(
                f,
                "invalid version, payload version: '{:?}', expected version: {:?}",
                ver, expected_ver
            ),
            #[cfg(feature = "check")]
            Error::NoChecksum => write!(f, "provided string is too small to contain a checksum"),
        }
    }
}
