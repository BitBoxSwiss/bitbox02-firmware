// SPDX-License-Identifier: Apache-2.0

pub const fn placeholder_count(value: &str) -> usize {
    let mut bytes = value.as_bytes();
    let mut count = 0;
    while let [_, rest @ ..] = bytes {
        count += matches!(bytes, [b'{', b'}', ..] | [b'{', b':', b'?', b'}', ..]) as usize;
        bytes = rest;
    }
    count
}
