//! Generic CBOR tokenization.

use core::ops::{Deref, DerefMut};

use crate::data::Token;
use crate::decode::Error;

/// An [`Iterator`] over CBOR tokens.
///
/// The `Iterator` implementation calls [`Tokenizer::token`] until end of input has been reached.
///
/// *Requires feature* `"half"`.
#[derive(Debug, Clone)]
pub struct Tokenizer<'a, 'b> {
    decoder: Decoder<'a, 'b>
}

impl<'a, 'b> Iterator for Tokenizer<'a, 'b> {
    type Item = Result<Token<'b>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.token() {
            Ok(t) => Some(Ok(t)),
            Err(e) if e.is_end_of_input() => None,
            Err(e) => Some(Err(e))
        }
    }
}

impl<'b> From<crate::Decoder<'b>> for Tokenizer<'_, 'b> {
    fn from(d: crate::Decoder<'b>) -> Self {
        Tokenizer { decoder: Decoder::Owned(d) }
    }
}

impl<'a, 'b> From<&'a mut crate::Decoder<'b>> for Tokenizer<'a, 'b> {
    fn from(d: &'a mut crate::Decoder<'b>) -> Self {
        Tokenizer { decoder: Decoder::Borrowed(d) }
    }
}

impl<'a, 'b> Tokenizer<'a, 'b> {
    /// Create a new Tokenizer for the given input bytes.
    pub fn new(bytes: &'b[u8]) -> Self {
        Tokenizer { decoder: Decoder::Owned(crate::Decoder::new(bytes)) }
    }

    /// Decode the next token.
    ///
    /// Note that a sequence of tokens may not necessarily represent
    /// well-formed CBOR items.
    pub fn token(&mut self) -> Result<Token<'b>, Error> {
        match self.decoder.decode() {
            Ok(tk) => Ok(tk),
            Err(e) => {
                let end = self.decoder.input().len();
                self.decoder.set_position(end); // drain decoder
                Err(e)
            }
        }
    }
}

#[cfg(feature = "alloc")]
impl core::fmt::Display for Tokenizer<'_, '_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        /// Control stack element.
        enum E {
            N,               // get next token
            T,               // tag
            A(Option<u64>),  // array
            M(Option<u64>),  // map
            B,               // indefinite bytes
            D,               // indefinite text
            S(&'static str), // display string
            X(&'static str)  // display string (unless next token is BREAK)
        }

        let mut iter  = self.clone().peekable();
        let mut stack = alloc::vec::Vec::new();

        while iter.peek().is_some() {
            stack.push(E::N);
            while let Some(elt) = stack.pop() {
                match elt {
                    E::N => match iter.next() {
                        Some(Ok(Token::Array(n))) => {
                            stack.push(E::A(Some(n)));
                            f.write_str("[")?
                        }
                        Some(Ok(Token::Map(n))) => {
                            stack.push(E::M(Some(n)));
                            f.write_str("{")?
                        }
                        Some(Ok(Token::BeginArray)) => {
                            stack.push(E::A(None));
                            f.write_str("[_ ")?
                        }
                        Some(Ok(Token::BeginMap)) => {
                            stack.push(E::M(None));
                            f.write_str("{_ ")?
                        }
                        Some(Ok(Token::BeginBytes)) => if let Some(Ok(Token::Break)) = iter.peek() {
                            iter.next();
                            f.write_str("''_")?
                        } else {
                            stack.push(E::B);
                            f.write_str("(_ ")?
                        }
                        Some(Ok(Token::BeginString)) => if let Some(Ok(Token::Break)) = iter.peek() {
                            iter.next();
                            f.write_str("\"\"_")?
                        } else {
                            stack.push(E::D);
                            f.write_str("(_ ")?
                        }
                        Some(Ok(Token::Tag(t))) => {
                            stack.push(E::T);
                            write!(f, "{}(", u64::from(t))?
                        }
                        Some(Ok(t))  => t.fmt(f)?,
                        Some(Err(e)) => {
                            write!(f, " !!! decoding error: {}", e)?;
                            return Ok(())
                        }
                        None => continue
                    }
                    E::S(s) => f.write_str(s)?,
                    E::X(s) => match iter.peek() {
                        Some(Ok(Token::Break)) | None => continue,
                        Some(Ok(_))  => f.write_str(s)?,
                        Some(Err(e)) => {
                            write!(f, " !!! decoding error: {}", e)?;
                            return Ok(())
                        }
                    }
                    E::T => {
                        stack.push(E::S(")"));
                        stack.push(E::N)
                    }
                    E::A(Some(0)) => f.write_str("]")?,
                    E::A(Some(1)) => {
                        stack.push(E::A(Some(0)));
                        stack.push(E::N)
                    }
                    E::A(Some(n)) => {
                        stack.push(E::A(Some(n - 1)));
                        stack.push(E::S(", "));
                        stack.push(E::N)
                    }
                    E::A(None) => match iter.peek() {
                        None => {
                            write!(f, " !!! indefinite array not closed")?;
                            return Ok(())
                        }
                        Some(Ok(Token::Break)) => {
                            iter.next();
                            f.write_str("]")?
                        }
                        _ => {
                            stack.push(E::A(None));
                            stack.push(E::X(", "));
                            stack.push(E::N)
                        }
                    }
                    E::M(Some(0)) => f.write_str("}")?,
                    E::M(Some(1)) => {
                        stack.push(E::M(Some(0)));
                        stack.push(E::N);
                        stack.push(E::S(": "));
                        stack.push(E::N)
                    }
                    E::M(Some(n)) => {
                        stack.push(E::M(Some(n - 1)));
                        stack.push(E::S(", "));
                        stack.push(E::N);
                        stack.push(E::S(": "));
                        stack.push(E::N)
                    }
                    E::M(None) => match iter.peek() {
                        None => {
                            write!(f, " !!! indefinite map not closed")?;
                            return Ok(())
                        }
                        Some(Ok(Token::Break)) => {
                            iter.next();
                            f.write_str("}")?
                        }
                        _ => {
                            stack.push(E::M(None));
                            stack.push(E::X(", "));
                            stack.push(E::N);
                            stack.push(E::S(": "));
                            stack.push(E::N)
                        }
                    }
                    E::B => match iter.peek() {
                        None => {
                            write!(f, " !!! indefinite byte string not closed")?;
                            return Ok(())
                        }
                        Some(Ok(Token::Break)) => {
                            iter.next();
                            f.write_str(")")?
                        }
                        _ => {
                            stack.push(E::B);
                            stack.push(E::X(", "));
                            stack.push(E::N)
                        }
                    }
                    E::D => match iter.peek() {
                        None => {
                            write!(f, " !!! indefinite string not closed")?;
                            return Ok(())
                        }
                        Some(Ok(Token::Break)) => {
                            iter.next();
                            f.write_str(")")?
                        }
                        _ => {
                            stack.push(E::D);
                            stack.push(E::X(", "));
                            stack.push(E::N)
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

/// Either own or borrow a decoder (similar to `alloc::borrow::Cow`).
#[derive(Debug)]
enum Decoder<'a, 'b> {
    Owned(crate::Decoder<'b>),
    Borrowed(&'a mut crate::Decoder<'b>)
}

impl<'b> Deref for Decoder<'_, 'b> {
    type Target = crate::Decoder<'b>;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Owned(d)    => d,
            Self::Borrowed(d) => d
        }
    }
}

impl<'b> DerefMut for Decoder<'_, 'b> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Owned(d)    => d,
            Self::Borrowed(d) => d
        }
    }
}

impl Clone for Decoder<'_, '_> {
    fn clone(&self) -> Self {
        match self {
            Self::Owned(d)    => Self::Owned(d.clone()),
            Self::Borrowed(d) => Self::Owned((*d).clone())
        }
    }
}

