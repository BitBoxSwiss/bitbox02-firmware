// SPDX-License-Identifier: Apache-2.0

use alloc::boxed::Box;
use alloc::collections::VecDeque;

use bitcoin::hashes::{Hash, sha256};

pub struct TestingRandom {
    mock_next_values: VecDeque<[u8; 32]>,
    counter: u32,
}

impl TestingRandom {
    pub fn new() -> Self {
        Self {
            mock_next_values: VecDeque::new(),
            counter: 0,
        }
    }

    pub fn mock_next(&mut self, value: [u8; 32]) {
        self.mock_next_values.push_back(value)
    }

    fn next_value(&mut self) -> [u8; 32] {
        self.counter += 1;
        if let Some(value) = self.mock_next_values.pop_front() {
            value
        } else {
            let hash = sha256::Hash::hash(&self.counter.to_be_bytes());
            hash.to_byte_array()
        }
    }
}

impl crate::hal::Random for TestingRandom {
    fn random_32_bytes(&mut self) -> Box<zeroize::Zeroizing<[u8; 32]>> {
        Box::new(zeroize::Zeroizing::new(self.next_value()))
    }

    fn mcu_32_bytes(&mut self, out: &mut [u8; 32]) {
        *out = self.next_value();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hal::Random;
    use hex_lit::hex;

    #[test]
    fn test_random() {
        let mut random = TestingRandom::new();
        let first = random.random_32_bytes();
        let second = random.random_32_bytes();
        assert_eq!(
            first.as_slice(),
            &hex!("b40711a88c7039756fb8a73827eabe2c0fe5a0346ca7e0a104adc0fc764f528d"),
        );
        assert_eq!(
            second.as_slice(),
            &hex!("433ebf5bc03dffa38536673207a21281612cef5faa9bc7a4d5b9be2fdb12cf1a"),
        );
    }

    #[test]
    fn test_mcu_32_bytes() {
        let mut random = TestingRandom::new();
        let mut first = [0u8; 32];
        let mut second = [0u8; 32];
        random.mcu_32_bytes(&mut first);
        random.mcu_32_bytes(&mut second);
        assert_eq!(
            first,
            hex!("b40711a88c7039756fb8a73827eabe2c0fe5a0346ca7e0a104adc0fc764f528d"),
        );
        assert_eq!(
            second,
            hex!("433ebf5bc03dffa38536673207a21281612cef5faa9bc7a4d5b9be2fdb12cf1a"),
        );
    }
}
