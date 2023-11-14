use crate::cipherstate::CipherState;
use crate::handshakepattern::{HandshakePattern, Token};
use crate::symmetricstate::SymmetricState;
use crate::traits::{Cipher, Hash, U8Array, DH};
use arrayvec::{ArrayString, ArrayVec};
use core::fmt::{Display, Error as FmtError, Formatter, Write};

#[cfg(feature = "use_alloc")]
use alloc::vec::Vec;

/// Noise handshake state.
pub struct HandshakeState<D: DH, C: Cipher, H: Hash> {
    symmetric: SymmetricState<C, H>,
    s: Option<D::Key>,
    e: Option<D::Key>,
    rs: Option<D::Pubkey>,
    re: Option<D::Pubkey>,
    is_initiator: bool,
    pattern: HandshakePattern,
    message_index: usize,
    pattern_has_psk: bool,
    psks: ArrayVec<[u8; 32], 4>,
}

impl<D, C, H> Clone for HandshakeState<D, C, H>
where
    D: DH,
    C: Cipher,
    H: Hash,
{
    fn clone(&self) -> Self {
        Self {
            symmetric: self.symmetric.clone(),
            s: self.s.as_ref().map(U8Array::clone),
            e: self.e.as_ref().map(U8Array::clone),
            rs: self.rs.as_ref().map(U8Array::clone),
            re: self.re.as_ref().map(U8Array::clone),
            is_initiator: self.is_initiator,
            pattern: self.pattern.clone(),
            message_index: self.message_index,
            pattern_has_psk: self.pattern_has_psk,
            psks: self.psks.clone(),
        }
    }
}

impl<D, C, H> HandshakeState<D, C, H>
where
    D: DH,
    C: Cipher,
    H: Hash,
{
    /// Get protocol name, e.g. Noise_IK_25519_ChaChaPoly_BLAKE2s.
    fn get_name(pattern_name: &str) -> ArrayString<256> {
        let mut ret = ArrayString::new();
        write!(
            &mut ret,
            "Noise_{}_{}_{}_{}",
            pattern_name,
            D::name(),
            C::name(),
            H::name()
        )
        .unwrap();
        ret
    }

    /// Initialize a handshake state.
    ///
    /// If `e` is [`None`], a new ephemeral key will be generated if necessary
    /// when [`write_message`](HandshakeState::write_message).
    ///
    /// # Setting Explicit Ephemeral Key
    ///
    /// An explicit `e` should only be specified for testing purposes, or in
    /// fallback patterns. If you do pass in an explicit `e`, [`HandshakeState`]
    /// will use it as is and will not generate new ephemeral keys in
    /// [`write_message`](HandshakeState::write_message).
    pub fn new<P>(
        pattern: HandshakePattern,
        is_initiator: bool,
        prologue: P,
        s: Option<D::Key>,
        e: Option<D::Key>,
        rs: Option<D::Pubkey>,
        re: Option<D::Pubkey>,
    ) -> Self
    where
        P: AsRef<[u8]>,
    {
        let mut symmetric = SymmetricState::new(Self::get_name(pattern.get_name()).as_bytes());
        let pattern_has_psk = pattern.has_psk();

        // Mix in prologue.
        symmetric.mix_hash(prologue.as_ref());

        // Mix in static keys known ahead of time.
        for t in pattern.get_pre_i() {
            match *t {
                Token::S => {
                    if is_initiator {
                        symmetric.mix_hash(D::pubkey(s.as_ref().unwrap()).as_slice());
                    } else {
                        symmetric.mix_hash(rs.as_ref().unwrap().as_slice());
                    }
                }
                _ => panic!("Unexpected token in pre message"),
            }
        }
        for t in pattern.get_pre_r() {
            match *t {
                Token::S => {
                    if is_initiator {
                        symmetric.mix_hash(rs.as_ref().unwrap().as_slice());
                    } else {
                        symmetric.mix_hash(D::pubkey(s.as_ref().unwrap()).as_slice());
                    }
                }
                Token::E => {
                    if is_initiator {
                        let re = re.as_ref().unwrap().as_slice();
                        symmetric.mix_hash(re);
                        if pattern_has_psk {
                            symmetric.mix_key(re);
                        }
                    } else {
                        let e = D::pubkey(e.as_ref().unwrap());
                        symmetric.mix_hash(e.as_slice());
                        if pattern_has_psk {
                            symmetric.mix_key(e.as_slice());
                        }
                    }
                }
                _ => panic!("Unexpected token in pre message"),
            }
        }

        HandshakeState {
            symmetric,
            s,
            e,
            rs,
            re,
            is_initiator,
            pattern,
            message_index: 0,
            pattern_has_psk,
            psks: ArrayVec::new(),
        }
    }

    /// Calculate the size overhead of the next message.
    ///
    /// # Panics
    ///
    /// If these is no more message to read/write, i.e., if the handshake is
    /// already completed.
    pub fn get_next_message_overhead(&self) -> usize {
        let m = self.pattern.get_message_pattern(self.message_index);

        let mut overhead = 0;

        let mut has_key = self.symmetric.has_key();

        for &t in m {
            match t {
                Token::E => {
                    overhead += D::Pubkey::len();
                    if self.pattern_has_psk {
                        has_key = true;
                    }
                }
                Token::S => {
                    overhead += D::Pubkey::len();
                    if has_key {
                        overhead += 16;
                    }
                }
                _ => {
                    has_key = true;
                }
            }
        }

        if has_key {
            overhead += 16
        }

        overhead
    }

    /// Like [`write_message`](HandshakeState::write_message), but returns a [`Vec`].
    #[cfg(any(feature = "use_std", feature = "use_alloc"))]
    pub fn write_message_vec(&mut self, payload: &[u8]) -> Result<Vec<u8>, Error> {
        let mut out = vec![0u8; payload.len() + self.get_next_message_overhead()];
        self.write_message(payload, &mut out)?;
        Ok(out)
    }

    /// Takes a payload and write the generated handshake message to
    /// `out`.
    ///
    /// # Error Kinds
    ///
    /// - [DH](ErrorKind::DH): DH operation failed.
    /// - [NeedPSK](ErrorKind::NeedPSK): A PSK token is encountered but none is available.
    ///
    /// # Panics
    ///
    /// * If a required static key is not set.
    ///
    /// * If `out.len() != payload.len() + self.get_next_message_overhead()`.
    ///
    /// * If it is not our turn to write.
    ///
    /// * If the handshake has already completed.
    pub fn write_message(&mut self, payload: &[u8], out: &mut [u8]) -> Result<(), Error> {
        debug_assert_eq!(out.len(), payload.len() + self.get_next_message_overhead());

        // Check that it is our turn to send.
        assert!(self.is_write_turn());

        // Get the message pattern.
        let m = self.pattern.get_message_pattern(self.message_index);
        self.message_index += 1;

        let mut cur: usize = 0;
        // Process tokens.
        for t in m {
            match *t {
                Token::E => {
                    if self.e.is_none() {
                        self.e = Some(D::genkey());
                    }
                    let e_pk = D::pubkey(self.e.as_ref().unwrap());
                    self.symmetric.mix_hash(e_pk.as_slice());
                    if self.pattern_has_psk {
                        self.symmetric.mix_key(e_pk.as_slice());
                    }
                    out[cur..cur + D::Pubkey::len()].copy_from_slice(e_pk.as_slice());
                    cur += D::Pubkey::len();
                }
                Token::S => {
                    let len = if self.symmetric.has_key() {
                        D::Pubkey::len() + 16
                    } else {
                        D::Pubkey::len()
                    };

                    let encrypted_s_out = &mut out[cur..cur + len];
                    self.symmetric.encrypt_and_hash(
                        D::pubkey(self.s.as_ref().unwrap()).as_slice(),
                        encrypted_s_out,
                    );
                    cur += len;
                }
                Token::PSK => {
                    if let Some(psk) = self.psks.pop_at(0) {
                        self.symmetric.mix_key_and_hash(&psk);
                    } else {
                        return Err(Error::need_psk());
                    }
                }
                t => {
                    let dh_result = self.perform_dh(t).map_err(|_| Error::dh())?;
                    self.symmetric.mix_key(dh_result.as_slice());
                }
            }
        }

        self.symmetric.encrypt_and_hash(payload, &mut out[cur..]);
        Ok(())
    }

    /// Takes a handshake message, process it and update our internal
    /// state, and write the encapsulated payload to `out`.
    ///
    /// # Error Kinds
    ///
    /// - [DH](ErrorKind::DH): DH operation failed.
    /// - [NeedPSK](ErrorKind::NeedPSK): A PSK token is encountered but none is
    ///   available.
    /// - [Decryption](ErrorKind::Decryption): Decryption failed.
    ///
    /// # Error Recovery
    ///
    /// If [`read_message`](HandshakeState::read_message) fails, the whole
    /// [`HandshakeState`] may be in invalid state and should not be used to
    /// read or write any further messages. (But
    /// [`get_re()`](HandshakeState::get_re) and
    /// [`get_rs()`](HandshakeState::get_rs) is allowed.) In case error recovery
    /// is desirable, [`clone`](Clone::clone) the [`HandshakeState`] before
    /// calling [`read_message`](HandshakeState::read_message).
    ///
    /// # Panics
    ///
    /// * If `out.len() + self.get_next_message_overhead() != data.len()`.
    ///
    ///   (Notes that this implies `data.len() >= overhead`.)
    ///
    /// * If a required static key is not set.
    ///
    /// * If it is not our turn to read.
    ///
    /// * If the handshake has already completed.
    pub fn read_message(&mut self, data: &[u8], out: &mut [u8]) -> Result<(), Error> {
        debug_assert_eq!(out.len() + self.get_next_message_overhead(), data.len());

        assert!(!self.is_write_turn());

        // Get the message pattern.
        let m = self.pattern.get_message_pattern(self.message_index);
        self.message_index += 1;

        let mut data = data;
        // Consume the next `n` bytes of data.
        let mut get = |n| {
            let ret = &data[..n];
            data = &data[n..];
            ret
        };

        // Process tokens.
        for t in m {
            match *t {
                Token::E => {
                    let re = D::Pubkey::from_slice(get(D::Pubkey::len()));
                    self.symmetric.mix_hash(re.as_slice());
                    if self.pattern_has_psk {
                        self.symmetric.mix_key(re.as_slice());
                    }
                    self.re = Some(re);
                }
                Token::S => {
                    let temp = get(if self.symmetric.has_key() {
                        D::Pubkey::len() + 16
                    } else {
                        D::Pubkey::len()
                    });
                    let mut rs = D::Pubkey::new();
                    self.symmetric
                        .decrypt_and_hash(temp, rs.as_mut())
                        .map_err(|_| Error::decryption())?;
                    self.rs = Some(rs);
                }
                Token::PSK => {
                    if let Some(psk) = self.psks.pop_at(0) {
                        self.symmetric.mix_key_and_hash(&psk);
                    } else {
                        return Err(Error::need_psk());
                    }
                }
                t => {
                    let dh_result = self.perform_dh(t).map_err(|_| Error::dh())?;
                    self.symmetric.mix_key(dh_result.as_slice());
                }
            }
        }

        self.symmetric
            .decrypt_and_hash(data, out)
            .map_err(|_| Error::decryption())
    }

    /// Similar to [`read_message`](HandshakeState::read_message), but returns
    /// result as a [`Vec`].
    ///
    /// In addition to possible errors from
    /// [`read_message`](HandshakeState::read_message),
    /// [TooShort](ErrorKind::TooShort) may be returned.
    #[cfg(any(feature = "use_std", feature = "use_alloc"))]
    pub fn read_message_vec(&mut self, data: &[u8]) -> Result<Vec<u8>, Error> {
        let overhead = self.get_next_message_overhead();
        if data.len() < overhead {
            Err(Error::too_short())
        } else {
            let mut out = vec![0u8; data.len() - overhead];
            self.read_message(data, &mut out)?;
            Ok(out)
        }
    }

    /// Push a PSK to the PSK-queue.
    ///
    /// # Panics
    ///
    /// If the PSK-queue becomes longer than 4.
    pub fn push_psk(&mut self, psk: &[u8]) {
        self.psks.push(U8Array::from_slice(psk));
    }

    /// Whether handshake has completed.
    pub fn completed(&self) -> bool {
        self.message_index == self.pattern.get_message_patterns_len()
    }

    /// Get handshake hash. Useful for e.g., channel binding.
    pub fn get_hash(&self) -> &[u8] {
        self.symmetric.get_hash()
    }

    /// Get ciphers that can be used to encrypt/decrypt further messages. The
    /// first [`CipherState`] is for initiator to responder, and the second for
    /// responder to initiator.
    ///
    /// Should be called after handshake is
    /// [`completed`](HandshakeState::completed).
    pub fn get_ciphers(&self) -> (CipherState<C>, CipherState<C>) {
        self.symmetric.split()
    }

    /// Get remote static pubkey, if available.
    pub fn get_rs(&self) -> Option<D::Pubkey> {
        self.rs.as_ref().map(U8Array::clone)
    }

    /// Get remote semi-ephemeral pubkey.
    ///
    /// Returns [`None`](None) if we do not know.
    ///
    /// Useful for noise-pipes.
    pub fn get_re(&self) -> Option<D::Pubkey> {
        self.re.as_ref().map(U8Array::clone)
    }

    /// Get whether this [`HandshakeState`] is created as initiator.
    pub fn get_is_initiator(&self) -> bool {
        self.is_initiator
    }

    /// Get handshake pattern this [`HandshakeState`] uses.
    pub fn get_pattern(&self) -> &HandshakePattern {
        &self.pattern
    }

    /// Check whether it is our turn to send in the handshake state.
    pub fn is_write_turn(&self) -> bool {
        self.message_index % 2 == if self.is_initiator { 0 } else { 1 }
    }

    fn perform_dh(&self, t: Token) -> Result<D::Output, ()> {
        let dh = |a: Option<&D::Key>, b: Option<&D::Pubkey>| D::dh(a.unwrap(), b.unwrap());

        match t {
            Token::EE => dh(self.e.as_ref(), self.re.as_ref()),
            Token::ES => {
                if self.is_initiator {
                    dh(self.e.as_ref(), self.rs.as_ref())
                } else {
                    dh(self.s.as_ref(), self.re.as_ref())
                }
            }
            Token::SE => {
                if self.is_initiator {
                    dh(self.s.as_ref(), self.re.as_ref())
                } else {
                    dh(self.e.as_ref(), self.rs.as_ref())
                }
            }
            Token::SS => dh(self.s.as_ref(), self.rs.as_ref()),
            _ => unreachable!(),
        }
    }
}

/// Handshake error.
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

/// Handshake error kind.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ErrorKind {
    /// A DH operation has failed.
    DH,
    /// A PSK is needed, but none is available.
    NeedPSK,
    /// Decryption failed.
    Decryption,
    /// The message is too short, and impossible to read.
    TooShort,
}

impl Error {
    fn dh() -> Error {
        Error {
            kind: ErrorKind::DH,
        }
    }

    fn need_psk() -> Error {
        Error {
            kind: ErrorKind::NeedPSK,
        }
    }

    fn decryption() -> Error {
        Error {
            kind: ErrorKind::Decryption,
        }
    }

    fn too_short() -> Error {
        Error {
            kind: ErrorKind::TooShort,
        }
    }

    /// Error kind.
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }
}

impl Display for Error {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), FmtError> {
        write!(fmt, "{:?}", self)
    }
}

#[cfg(feature = "use_std")]
impl ::std::error::Error for Error {
    fn description(&self) -> &'static str {
        match self.kind {
            ErrorKind::DH => "DH error",
            ErrorKind::NeedPSK => "Need PSK",
            ErrorKind::Decryption => "Decryption failed",
            ErrorKind::TooShort => "Message is too short",
        }
    }
}

/// Builder for `HandshakeState`.
pub struct HandshakeStateBuilder<'a, D: DH> {
    pattern: Option<HandshakePattern>,
    is_initiator: Option<bool>,
    prologue: Option<&'a [u8]>,
    s: Option<D::Key>,
    e: Option<D::Key>,
    rs: Option<D::Pubkey>,
    re: Option<D::Pubkey>,
}

impl<'a, D: DH> Default for HandshakeStateBuilder<'a, D> {
    fn default() -> Self {
        HandshakeStateBuilder::new()
    }
}

impl<'a, D> HandshakeStateBuilder<'a, D>
where
    D: DH,
{
    /// Create a new [`HandshakeStateBuilder`].
    pub fn new() -> Self {
        HandshakeStateBuilder {
            pattern: None,
            is_initiator: None,
            prologue: None,
            s: None,
            e: None,
            rs: None,
            re: None,
        }
    }

    /// Set handshake pattern.
    pub fn set_pattern(&mut self, p: HandshakePattern) -> &mut Self {
        self.pattern = Some(p);
        self
    }

    /// Set whether the [`HandshakeState`] is initiator.
    pub fn set_is_initiator(&mut self, is: bool) -> &mut Self {
        self.is_initiator = Some(is);
        self
    }

    /// Set prologue.
    pub fn set_prologue(&mut self, prologue: &'a [u8]) -> &mut Self {
        self.prologue = Some(prologue);
        self
    }

    /// Set ephemeral key.
    ///
    /// This is not encouraged and usually not necessary. Cf.
    /// [`HandshakeState::new()`].
    pub fn set_e(&mut self, e: D::Key) -> &mut Self {
        self.e = Some(e);
        self
    }

    /// Set static key.
    pub fn set_s(&mut self, s: D::Key) -> &mut Self {
        self.s = Some(s);
        self
    }

    /// Set peer semi-ephemeral public key.
    ///
    /// Usually used in fallback patterns.
    pub fn set_re(&mut self, re: D::Pubkey) -> &mut Self {
        self.re = Some(re);
        self
    }

    /// Set peer static public key.
    pub fn set_rs(&mut self, rs: D::Pubkey) -> &mut Self {
        self.rs = Some(rs);
        self
    }

    /// Build [`HandshakeState`].
    ///
    /// # Panics
    ///
    /// If any of [`set_pattern`](HandshakeStateBuilder::set_pattern),
    /// [`set_prologue`](HandshakeStateBuilder::set_prologue) or
    /// [`set_is_initiator`](HandshakeStateBuilder::set_is_initiator) has not
    /// been called yet.
    pub fn build_handshake_state<C, H>(self) -> HandshakeState<D, C, H>
    where
        C: Cipher,
        H: Hash,
    {
        HandshakeState::new(
            self.pattern.unwrap(),
            self.is_initiator.unwrap(),
            self.prologue.unwrap(),
            self.s,
            self.e,
            self.rs,
            self.re,
        )
    }
}
