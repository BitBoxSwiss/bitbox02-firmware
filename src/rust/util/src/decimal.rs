// Copyright 2020 Shift Crypto AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate alloc;
use alloc::string::String;

/// Formats integer `value` as `value / 10^decimals`, with up to `decimals` decimal places.
/// E.g. "123450" wit decimals=3: "123.45".
/// Value must consists only if '0'-'9' digits.
pub fn format(value: &str, decimals: usize) -> String {
    let mut v: String = format!("{:0>width$}", value, width = decimals + 1);
    v.insert(v.len() - decimals, '.');
    v.trim_end_matches('0').trim_end_matches('.').into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_int() {
        assert_eq!(format("0", 0), "0");
        assert_eq!(format("0", 2), "0");
        assert_eq!(format("0", 6), "0");

        assert_eq!(format("1", 0), "1");
        assert_eq!(format("1", 1), "0.1");
        assert_eq!(format("1", 2), "0.01");
        assert_eq!(format("1", 6), "0.000001");

        assert_eq!(format("12345", 0), "12345");
        assert_eq!(format("12345", 1), "1234.5");
        assert_eq!(format("12345", 2), "123.45");
        assert_eq!(format("12345", 3), "12.345");
        assert_eq!(format("12345", 4), "1.2345");
        assert_eq!(format("12345", 5), "0.12345");
        assert_eq!(format("12345", 6), "0.012345");
        assert_eq!(format("12345", 7), "0.0012345");
    }
}
