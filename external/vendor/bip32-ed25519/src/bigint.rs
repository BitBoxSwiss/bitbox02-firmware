/// Computes `(x + 8*y[:28]) % 2^256`. where `x` and `y` are little-endian encoded ints.
pub fn add_28_mul8(x: &[u8], y: &[u8], out: &mut [u8; 32]) {
    let mut carry: u16 = 0;
    for i in 0..28 {
        let r = x[i] as u16 + ((y[i] as u16) << 3) + carry;
        out[i] = (r & 0xff) as u8;
        carry = r >> 8;
    }
    for i in 28..32 {
        let r = x[i] as u16 + carry;
        out[i] = (r & 0xff) as u8;
        carry = r >> 8;
    }
}

/// Computes `(x + y) % 2^256` where `x` and `y` are little-endian encoded ints.
pub fn add_256bits(x: &[u8], y: &[u8], out: &mut [u8; 32]) {
    let mut carry: u16 = 0;
    for i in 0..32 {
        let r = (x[i] as u16) + (y[i] as u16) + carry;
        out[i] = r as u8;
        carry = r >> 8;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arbitrary::Arbitrary32;
    use core::ops::{Add, Mul};
    use num_bigint::BigUint;

    fn add_256bits_bigint(x: &[u8], y: &[u8]) -> [u8; 32] {
        let left = BigUint::from_bytes_le(x);
        let right = BigUint::from_bytes_le(y);
        let s = &left.add(right).to_bytes_le();

        let mut out = [0u8; 32];
        for (i, e) in (0..32).zip(s) {
            out[i] = *e;
        }
        out
    }

    #[quickcheck]
    fn test_add_256bits_quickcheck(left: Arbitrary32, right: Arbitrary32) {
        let mut out = [0u8; 32];
        add_256bits(&left.0, &right.0, &mut out);

        assert_eq!(out, add_256bits_bigint(&left.0, &right.0));
    }

    fn add_28_mul8_bigint(x: &[u8], y: &[u8]) -> [u8; 32] {
        let left = BigUint::from_bytes_le(x);
        let right = BigUint::from_bytes_le(&y[..28]);
        let s = &left.add(right.mul(BigUint::from(8u32))).to_bytes_le();

        let mut out = [0u8; 32];
        for (i, e) in (0..32).zip(s) {
            out[i] = *e;
        }
        out
    }

    #[quickcheck]
    fn test_add_28_mul8_quickcheck(left: Arbitrary32, right: Arbitrary32) {
        let mut out = [0u8; 32];
        add_28_mul8(&left.0, &right.0, &mut out);
        assert_eq!(out, add_28_mul8_bigint(&left.0, &right.0));
    }
}
