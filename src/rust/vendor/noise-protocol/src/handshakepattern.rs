//! Handshake patterns.

use arrayvec::ArrayVec;

/// A token in noise message patterns.
#[allow(missing_docs)]
#[derive(Copy, Clone)]
pub enum Token {
    E,
    S,
    EE,
    ES,
    SE,
    SS,
    PSK,
}

use self::Token::*;

/// Noise handshake pattern.
#[derive(Clone)]
pub struct HandshakePattern {
    pre_i: ArrayVec<Token, 4>,
    pre_r: ArrayVec<Token, 4>,
    msg_patterns: ArrayVec<ArrayVec<Token, 8>, 8>,
    name: &'static str,
}

impl HandshakePattern {
    /// Construct a new HandshakePattern from pre-message patterns, message patterns and name.
    ///
    /// # Pattern validity
    ///
    /// It is the caller's responlity to ensure that the pattern is *valid*.
    ///
    /// # Panics
    ///
    /// If any of the patterns are too long (longer than 8 tokens).
    ///
    /// Or if the number of patterns are too large (larger than 8).
    pub fn new(
        pre_i: &[Token],
        pre_r: &[Token],
        msg_patterns: &[&[Token]],
        name: &'static str,
    ) -> Self {
        HandshakePattern {
            pre_i: pre_i.iter().cloned().collect(),
            pre_r: pre_r.iter().cloned().collect(),
            msg_patterns: msg_patterns
                .iter()
                .map(|p| p.iter().cloned().collect())
                .collect(),
            name,
        }
    }

    /// Get initiator pre-messages.
    pub fn get_pre_i(&self) -> &[Token] {
        &self.pre_i
    }

    /// Get responder pre-messages.
    pub fn get_pre_r(&self) -> &[Token] {
        &self.pre_r
    }

    /// Get message patterns.
    pub fn get_message_pattern(&self, i: usize) -> &[Token] {
        &self.msg_patterns[i]
    }

    /// Get number of message patterns.
    pub fn get_message_patterns_len(&self) -> usize {
        self.msg_patterns.len()
    }

    /// Get pattern name.
    pub fn get_name(&self) -> &'static str {
        self.name
    }

    /// Whether there are any psk tokens in this pattern.
    pub fn has_psk(&self) -> bool {
        self.msg_patterns.iter().any(|m| {
            m.iter().any(|m| match m {
                Token::PSK => true,
                _ => false,
            })
        })
    }

    /// Whether the pattern is a one-way pattern.
    pub fn is_one_way(&self) -> bool {
        self.msg_patterns.len() == 1
    }

    fn with_psks(&self, poses: &[usize], new_name: &'static str) -> HandshakePattern {
        let mut new_msg_patterns = self.msg_patterns.clone();
        for pos in poses {
            if *pos == 0usize {
                new_msg_patterns[0].insert(0, PSK);
            } else {
                new_msg_patterns[pos - 1].push(PSK);
            }
        }
        HandshakePattern {
            pre_i: self.pre_i.clone(),
            pre_r: self.pre_r.clone(),
            msg_patterns: new_msg_patterns,
            name: new_name,
        }
    }
}

macro_rules! vec {
    () => {
        ArrayVec::new()
    };
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = ArrayVec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

/// The `Noise_N` pattern.
pub fn noise_n() -> HandshakePattern {
    HandshakePattern {
        pre_i: vec![],
        pre_r: vec![S],
        msg_patterns: vec![vec![E, ES]],
        name: "N",
    }
}

/// The `Noise_K` pattern.
pub fn noise_k() -> HandshakePattern {
    HandshakePattern {
        pre_i: vec![S],
        pre_r: vec![S],
        msg_patterns: vec![vec![E, ES, SS]],
        name: "K",
    }
}

/// The `Noise_X` pattern.
pub fn noise_x() -> HandshakePattern {
    HandshakePattern {
        pre_i: vec![],
        pre_r: vec![S],
        msg_patterns: vec![vec![E, ES, S, SS]],
        name: "X",
    }
}

/// The `Noise_NN` pattern.
pub fn noise_nn() -> HandshakePattern {
    HandshakePattern {
        pre_i: vec![],
        pre_r: vec![],
        msg_patterns: vec![vec![E], vec![E, EE]],
        name: "NN",
    }
}

/// The `Noise_NK` pattern.
pub fn noise_nk() -> HandshakePattern {
    HandshakePattern {
        pre_i: vec![],
        pre_r: vec![S],
        msg_patterns: vec![vec![E, ES], vec![E, EE]],
        name: "NK",
    }
}

/// The `Noise_NX` pattern.
pub fn noise_nx() -> HandshakePattern {
    HandshakePattern {
        pre_i: vec![],
        pre_r: vec![],
        msg_patterns: vec![vec![E], vec![E, EE, S, ES]],
        name: "NX",
    }
}

/// The `Noise_XN` pattern.
pub fn noise_xn() -> HandshakePattern {
    HandshakePattern {
        pre_i: vec![],
        pre_r: vec![],
        msg_patterns: vec![vec![E], vec![E, EE], vec![S, SE]],
        name: "XN",
    }
}

/// The `Noise_XK` pattern.
pub fn noise_xk() -> HandshakePattern {
    HandshakePattern {
        pre_i: vec![],
        pre_r: vec![S],
        msg_patterns: vec![vec![E, ES], vec![E, EE], vec![S, SE]],
        name: "XK",
    }
}

/// The `Noise_XX` pattern.
pub fn noise_xx() -> HandshakePattern {
    HandshakePattern {
        pre_i: vec![],
        pre_r: vec![],
        msg_patterns: vec![vec![E], vec![E, EE, S, ES], vec![S, SE]],
        name: "XX",
    }
}

/// The `Noise_KN` pattern.
pub fn noise_kn() -> HandshakePattern {
    HandshakePattern {
        pre_i: vec![S],
        pre_r: vec![],
        msg_patterns: vec![vec![E], vec![E, EE, SE]],
        name: "KN",
    }
}

/// The `Noise_KK` pattern.
pub fn noise_kk() -> HandshakePattern {
    HandshakePattern {
        pre_i: vec![S],
        pre_r: vec![S],
        msg_patterns: vec![vec![E, ES, SS], vec![E, EE, SE]],
        name: "KK",
    }
}

/// The `Noise_KX` pattern.
pub fn noise_kx() -> HandshakePattern {
    HandshakePattern {
        pre_i: vec![S],
        pre_r: vec![],
        msg_patterns: vec![vec![E], vec![E, EE, SE, S, ES]],
        name: "KX",
    }
}

/// The `Noise_IN` pattern.
pub fn noise_in() -> HandshakePattern {
    HandshakePattern {
        pre_i: vec![],
        pre_r: vec![],
        msg_patterns: vec![vec![E, S], vec![E, EE, SE]],
        name: "IN",
    }
}

/// The `Noise_IK` pattern.
pub fn noise_ik() -> HandshakePattern {
    HandshakePattern {
        pre_i: vec![],
        pre_r: vec![S],
        msg_patterns: vec![vec![E, ES, S, SS], vec![E, EE, SE]],
        name: "IK",
    }
}

/// The `Noise_IX` pattern.
pub fn noise_ix() -> HandshakePattern {
    HandshakePattern {
        pre_i: vec![],
        pre_r: vec![],
        msg_patterns: vec![vec![E, S], vec![E, EE, SE, S, ES]],
        name: "IX",
    }
}

/// The `Noise_XXfallback` pattern.
///
/// Something that is used in noise pipes.
pub fn noise_xx_fallback() -> HandshakePattern {
    HandshakePattern {
        pre_i: vec![],
        pre_r: vec![E],
        msg_patterns: vec![vec![E, EE, S, SE], vec![S, ES]],
        name: "XXfallback",
    }
}

// PSK Patterns.

/// The `Noise_Npsk0` pattern.
pub fn noise_n_psk0() -> HandshakePattern {
    noise_n().with_psks(&[0], "Npsk0")
}

/// The `Noise_Kpsk0` pattern.
pub fn noise_k_psk0() -> HandshakePattern {
    noise_k().with_psks(&[0], "Kpsk0")
}

/// The `Noise_Xpsk1` pattern.
pub fn noise_x_psk1() -> HandshakePattern {
    noise_x().with_psks(&[1], "Xpsk1")
}

/// The `Noise_NNpsk0` pattern.
pub fn noise_nn_psk0() -> HandshakePattern {
    noise_nn().with_psks(&[0], "NNpsk0")
}

/// The `Noise_NNpsk2` pattern.
pub fn noise_nn_psk2() -> HandshakePattern {
    noise_nn().with_psks(&[2], "NNpsk2")
}

/// The `Noise_NKpsk0` pattern.
pub fn noise_nk_psk0() -> HandshakePattern {
    noise_nk().with_psks(&[0], "NKpsk0")
}

/// The `Noise_NKpsk2` pattern.
pub fn noise_nk_psk2() -> HandshakePattern {
    noise_nk().with_psks(&[2], "NKpsk2")
}

/// The `Noise_NXpsk2` pattern.
pub fn noise_nx_psk2() -> HandshakePattern {
    noise_nx().with_psks(&[2], "NXpsk2")
}

/// The `Noise_XNpsk3` pattern.
pub fn noise_xn_psk3() -> HandshakePattern {
    noise_xn().with_psks(&[3], "XNpsk3")
}

/// The `Noise_XKpsk3` pattern.
pub fn noise_xk_psk3() -> HandshakePattern {
    noise_xk().with_psks(&[3], "XKpsk3")
}

/// The `Noise_XXpsk3` pattern.
pub fn noise_xx_psk3() -> HandshakePattern {
    noise_xx().with_psks(&[3], "XXpsk3")
}

/// The `Noise_KNpsk0` pattern.
pub fn noise_kn_psk0() -> HandshakePattern {
    noise_kn().with_psks(&[0], "KNpsk0")
}

/// The `Noise_KNpsk2` pattern.
pub fn noise_kn_psk2() -> HandshakePattern {
    noise_kn().with_psks(&[2], "KNpsk2")
}

/// The `Noise_KKpsk0` pattern.
pub fn noise_kk_psk0() -> HandshakePattern {
    noise_kk().with_psks(&[0], "KKpsk0")
}

/// The `Noise_KKpsk2` pattern.
pub fn noise_kk_psk2() -> HandshakePattern {
    noise_kk().with_psks(&[2], "KKpsk2")
}

/// The `Noise_KXpsk2` pattern.
pub fn noise_kx_psk2() -> HandshakePattern {
    noise_kx().with_psks(&[2], "KXpsk2")
}

/// The `Noise_INpsk1` pattern.
pub fn noise_in_psk1() -> HandshakePattern {
    noise_in().with_psks(&[1], "INpsk1")
}

/// The `Noise_INpsk2` pattern.
pub fn noise_in_psk2() -> HandshakePattern {
    noise_in().with_psks(&[2], "INpsk2")
}

/// The `Noise_IKpsk1` pattern.
pub fn noise_ik_psk1() -> HandshakePattern {
    noise_ik().with_psks(&[1], "IKpsk1")
}

/// The `Noise_IKpsk2` pattern.
pub fn noise_ik_psk2() -> HandshakePattern {
    noise_ik().with_psks(&[2], "IKpsk2")
}

/// The `Noise_IXpsk2` pattern.
pub fn noise_ix_psk2() -> HandshakePattern {
    noise_ix().with_psks(&[2], "IXpsk2")
}

/// The `Noise_NNpsk0+psk2` pattern.
pub fn noise_nn_psk0_psk2() -> HandshakePattern {
    noise_nn().with_psks(&[0, 2], "NNpsk0+psk2")
}

/// The `Noise_NXpsk0+psk1+psk2` pattern.
pub fn noise_nx_psk0_psk1_psk2() -> HandshakePattern {
    noise_nx().with_psks(&[0, 1, 2], "NXpsk0+psk1+psk2")
}

/// The `Noise_XNpsk1+psk3` pattern.
pub fn noise_xn_psk1_psk3() -> HandshakePattern {
    noise_xn().with_psks(&[1, 3], "XNpsk1+psk3")
}

/// The `Noise_XKpsk0+psk3` pattern.
pub fn noise_xk_psk0_psk3() -> HandshakePattern {
    noise_xk().with_psks(&[0, 3], "XKpsk0+psk3")
}

/// The `Noise_KNpsk1+psk2` pattern.
pub fn noise_kn_psk1_psk2() -> HandshakePattern {
    noise_kn().with_psks(&[1, 2], "KNpsk1+psk2")
}

/// The `Noise_KKpsk0+psk2` pattern
pub fn noise_kk_psk0_psk2() -> HandshakePattern {
    noise_kk().with_psks(&[0, 2], "KKpsk0+psk2")
}

/// The `Noise_INpsk1+psk2` pattern.
pub fn noise_in_psk1_psk2() -> HandshakePattern {
    noise_in().with_psks(&[1, 2], "INpsk1+psk2")
}

/// The `Noise_IKpsk0+psk2` pattern.
pub fn noise_ik_psk0_psk2() -> HandshakePattern {
    noise_ik().with_psks(&[0, 2], "IKpsk0+psk2")
}

/// The `Noise_IXpsk0+psk2` pattern.
pub fn noise_ix_psk0_psk2() -> HandshakePattern {
    noise_ix().with_psks(&[0, 2], "IXpsk0+psk2")
}

/// The `Noise_XXpsk0+psk1` pattern.
pub fn noise_xx_psk0_psk1() -> HandshakePattern {
    noise_xx().with_psks(&[0, 1], "XXpsk0+psk1")
}

/// The `Noise_XXpsk0+psk2` pattern.
pub fn noise_xx_psk0_psk2() -> HandshakePattern {
    noise_xx().with_psks(&[0, 2], "XXpsk0+psk2")
}

/// The `Noise_XXpsk0+psk3` pattern.
pub fn noise_xx_psk0_psk3() -> HandshakePattern {
    noise_xx().with_psks(&[0, 3], "XXpsk0+psk3")
}

/// The `Noise_XXpsk0+psk1+psk2+psk3` pattern.
pub fn noise_xx_psk0_psk1_psk2_psk3() -> HandshakePattern {
    noise_xx().with_psks(&[0, 1, 2, 3], "XXpsk0+psk1+psk2+psk3")
}
