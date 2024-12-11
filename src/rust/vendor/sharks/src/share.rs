use alloc::vec::Vec;

use super::field::GF256;

#[cfg(feature = "fuzzing")]
use arbitrary::Arbitrary;

#[cfg(feature = "zeroize_memory")]
use zeroize::Zeroize;

/// A share used to reconstruct the secret. Can be serialized to and from a byte array.
///
/// Usage example:
/// ```
/// use sharks::{Sharks, Share};
/// use core::convert::TryFrom;
/// # use rand_chacha::rand_core::SeedableRng;
/// # fn send_to_printer(_: Vec<u8>) {}
/// # fn ask_shares() -> Vec<Vec<u8>> {vec![vec![1, 2], vec![2, 3], vec![3, 4]]}
///
/// // Transmit the share bytes to a printer
/// let sharks = Sharks(3);
/// let mut rng = rand_chacha::ChaCha8Rng::from_seed([0x90; 32]);
/// let dealer = sharks.dealer_rng(&[1, 2, 3], &mut rng);
///
/// // Get 5 shares and print paper keys
/// for s in dealer.take(5) {
///     send_to_printer(Vec::from(&s));
/// };
///
/// // Get share bytes from an external source and recover secret
/// let shares_bytes: Vec<Vec<u8>> = ask_shares();
/// let shares: Vec<Share> = shares_bytes.iter().map(|s| Share::try_from(s.as_slice()).unwrap()).collect();
/// let secret = sharks.recover(&shares).unwrap();
#[derive(Clone)]
#[cfg_attr(feature = "fuzzing", derive(Arbitrary, Debug))]
#[cfg_attr(feature = "zeroize_memory", derive(Zeroize))]
#[cfg_attr(feature = "zeroize_memory", zeroize(drop))]
pub struct Share {
    pub x: GF256,
    pub y: Vec<GF256>,
}

/// Obtains a byte vector from a `Share` instance
impl From<&Share> for Vec<u8> {
    fn from(s: &Share) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(s.y.len() + 1);
        bytes.push(s.x.0);
        bytes.extend(s.y.iter().map(|p| p.0));
        bytes
    }
}

/// Obtains a `Share` instance from a byte slice
impl core::convert::TryFrom<&[u8]> for Share {
    type Error = &'static str;

    fn try_from(s: &[u8]) -> Result<Share, Self::Error> {
        if s.len() < 2 {
            Err("A Share must be at least 2 bytes long")
        } else {
            let x = GF256(s[0]);
            let y = s[1..].iter().map(|p| GF256(*p)).collect();
            Ok(Share { x, y })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Share, GF256};
    use alloc::{vec, vec::Vec};
    use core::convert::TryFrom;

    #[test]
    fn vec_from_share_works() {
        let share = Share {
            x: GF256(1),
            y: vec![GF256(2), GF256(3)],
        };
        let bytes = Vec::from(&share);
        assert_eq!(bytes, vec![1, 2, 3]);
    }

    #[test]
    fn share_from_u8_slice_works() {
        let bytes = [1, 2, 3];
        let share = Share::try_from(&bytes[..]).unwrap();
        assert_eq!(share.x, GF256(1));
        assert_eq!(share.y, vec![GF256(2), GF256(3)]);
    }
}
