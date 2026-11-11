//! Slice reader.

use super::position::Position;
use crate::{BytesRef, Decode, EncodingRules, Error, ErrorKind, Length, Reader};

/// [`Reader`] which consumes an input byte slice.
#[derive(Clone, Debug)]
pub struct SliceReader<'a> {
    /// Byte slice being decoded.
    bytes: &'a BytesRef,

    /// Encoding rules to apply when decoding the input.
    encoding_rules: EncodingRules,

    /// Did the decoding operation fail?
    failed: bool,

    /// Position within the decoded slice.
    position: Position,
}

impl<'a> SliceReader<'a> {
    /// Create a new slice reader for the given byte slice.
    ///
    /// # Errors
    /// If `bytes` is too long.
    pub fn new(bytes: &'a [u8]) -> Result<Self, Error> {
        Self::new_with_encoding_rules(bytes, EncodingRules::default())
    }

    /// Create a new slice reader with the given encoding rules.
    ///
    /// # Errors
    /// If `bytes` is too long.
    pub fn new_with_encoding_rules(
        bytes: &'a [u8],
        encoding_rules: EncodingRules,
    ) -> Result<Self, Error> {
        Ok(Self {
            bytes: BytesRef::new(bytes)?,
            encoding_rules,
            failed: false,
            position: Position::new(bytes.len().try_into()?),
        })
    }

    /// Return an error with the given [`ErrorKind`], annotating it with context about where the
    /// error occurred.
    pub fn error(&mut self, kind: ErrorKind) -> Error {
        self.failed = true;
        self.position.error(kind)
    }

    /// Did the decoding operation fail due to an error?
    #[must_use]
    pub fn is_failed(&self) -> bool {
        self.failed
    }

    /// Obtain the remaining bytes in this slice reader from the current cursor position.
    pub(crate) fn remaining(&self) -> Result<&'a [u8], Error> {
        if self.is_failed() {
            Err(ErrorKind::Failed.at(self.position.current()))
        } else {
            self.bytes
                .as_slice()
                .get(self.position.current().try_into()?..)
                .ok_or_else(|| Error::incomplete(self.input_len()))
        }
    }
}

impl<'a> Reader<'a> for SliceReader<'a> {
    const CAN_READ_SLICE: bool = true;

    fn encoding_rules(&self) -> EncodingRules {
        self.encoding_rules
    }

    fn input_len(&self) -> Length {
        self.bytes.len()
    }

    fn position(&self) -> Length {
        self.position.current()
    }

    /// Read nested data of the given length.
    #[inline]
    fn read_nested<T, F, E>(&mut self, len: Length, f: F) -> Result<T, E>
    where
        F: FnOnce(&mut Self) -> Result<T, E>,
        E: From<Error>,
    {
        // Slice `self.bytes` as a secondary check we don't read past end-of-slice
        let bytes = self.bytes;
        let prefix_len = (self.position.current() + len)?;
        self.bytes = self.bytes.prefix(prefix_len)?;

        let resumption = self.position.split_nested(len)?;
        let ret = f(self);
        let finished = self.is_finished();
        let decoded = self.position.current();
        let remaining = self.remaining_len();

        self.bytes = bytes;
        self.position.resume_nested(resumption);

        if ret.is_ok() && !finished {
            self.failed = true;
            return Err(self
                .error(ErrorKind::TrailingData { decoded, remaining })
                .into());
        };

        ret
    }

    fn read_slice(&mut self, len: Length) -> Result<&'a [u8], Error> {
        if self.is_failed() {
            return Err(self.error(ErrorKind::Failed));
        }

        match self.remaining()?.get(..len.try_into()?) {
            Some(result) => {
                self.position.advance(len)?;
                Ok(result)
            }
            None => Err(self.error(ErrorKind::Incomplete {
                expected_len: (self.position.current() + len)?,
                actual_len: self.input_len(),
            })),
        }
    }

    fn decode<T: Decode<'a>>(&mut self) -> Result<T, T::Error> {
        if self.is_failed() {
            return Err(self.error(ErrorKind::Failed).into());
        }

        T::decode(self).inspect_err(|_| {
            self.failed = true;
        })
    }

    fn error(&mut self, kind: ErrorKind) -> Error {
        self.error(kind)
    }

    fn finish(mut self) -> Result<(), Error> {
        if self.is_failed() {
            Err(ErrorKind::Failed.at(self.position.current()))
        } else if !self.is_finished() {
            let decoded = self.position.current();
            let remaining = self.remaining_len();
            Err(self.error(ErrorKind::TrailingData { decoded, remaining }))
        } else {
            Ok(())
        }
    }

    fn remaining_len(&self) -> Length {
        self.position.remaining_len()
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic, reason = "tests")]
mod tests {
    use super::SliceReader;
    use crate::{Decode, Error, ErrorKind, Length, Reader};
    use hex_literal::hex;

    // INTEGER: 42
    const EXAMPLE_MSG: &[u8] = &hex!("02012A00");

    #[test]
    fn empty_message() {
        let mut reader = SliceReader::new(&[]).unwrap();
        let err = bool::decode(&mut reader).err().unwrap();
        assert_eq!(Some(Length::ZERO), err.position());

        match err.kind() {
            ErrorKind::Incomplete {
                expected_len,
                actual_len,
            } => {
                assert_eq!(actual_len, 0u8.into());
                assert_eq!(expected_len, 1u8.into());
            }
            other => panic!("unexpected error kind: {:?}", other),
        }
    }

    #[test]
    fn invalid_field_length() {
        const MSG_LEN: usize = 2;

        let mut reader = SliceReader::new(&EXAMPLE_MSG[..MSG_LEN]).unwrap();
        let err = i8::decode(&mut reader).err().unwrap();
        assert_eq!(Some(Length::from(2u8)), err.position());

        match err.kind() {
            ErrorKind::Incomplete {
                expected_len,
                actual_len,
            } => {
                assert_eq!(actual_len, MSG_LEN.try_into().unwrap());
                assert_eq!(expected_len, (MSG_LEN + 1).try_into().unwrap());
            }
            other => panic!("unexpected error kind: {:?}", other),
        }
    }

    #[test]
    fn trailing_data() {
        let mut reader = SliceReader::new(EXAMPLE_MSG).unwrap();
        let x = i8::decode(&mut reader).unwrap();
        assert_eq!(42i8, x);

        let err = reader.finish().err().unwrap();
        assert_eq!(Some(Length::from(3u8)), err.position());

        assert_eq!(
            ErrorKind::TrailingData {
                decoded: 3u8.into(),
                remaining: 1u8.into(),
            },
            err.kind()
        );
    }

    #[test]
    fn nested_trailing_data() {
        let der = hex!("0102");
        let mut reader = SliceReader::new(&der).unwrap();

        let err: Error = reader
            .read_nested(2u8.into(), |reader| {
                reader.read_slice(1u8.into())?;
                Ok(())
            })
            .expect_err("read_nested should return Err when the callback did not consume the complete contents of the nested value");

        assert_eq!(Length::ONE, err.position().unwrap());
        assert_eq!(
            ErrorKind::TrailingData {
                decoded: 1u8.into(),
                remaining: 1u8.into(),
            },
            err.kind()
        );
    }
}
