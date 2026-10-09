// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::format_collect)]

use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Write};
use std::path::Path;

struct Token {
    unit: String,
    contract_address: [u8; 20],
    decimals: u8,
}

fn main() {
    let file = File::open("src/tokens.txt").unwrap();
    let reader = io::BufReader::new(file);
    let mut tokens = Vec::<Token>::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if line.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = line.split(';').collect();
        if parts.len() != 3 {
            panic!("token line has more than three fields");
        }
        let (unit, contract_address) = (parts[0], parts[1]);
        if !unit.bytes().all(|byte| (32..=126).contains(&byte)) {
            panic!("token unit must be printable ASCII");
        }
        let decimals: u8 = parts[2].parse().unwrap();

        tokens.push(Token {
            unit: unit.into(),
            contract_address: hex::decode(contract_address.strip_prefix("0x").unwrap())
                .unwrap()
                .try_into()
                .unwrap(),
            decimals,
        });
    }

    // A symbol is ambiguous if multiple contracts use it, even with different decimals:
    // changing the raw transfer value can still produce the same displayed amount.
    let mut contracts_by_unit: BTreeMap<&str, BTreeSet<[u8; 20]>> = BTreeMap::new();
    for token in &tokens {
        contracts_by_unit
            .entry(&token.unit)
            .or_default()
            .insert(token.contract_address);
    }

    // The 32 most frequent characters fit in five bits.
    let mut frequencies = BTreeMap::<u8, usize>::new();
    for byte in tokens.iter().flat_map(|token| token.unit.bytes()) {
        *frequencies.entry(byte).or_default() += 1;
    }
    let mut alphabet: Vec<u8> = frequencies.keys().copied().collect();
    alphabet.sort_by_key(|byte| (Reverse(frequencies[byte]), *byte));
    assert!(
        alphabet.len() <= 64,
        "ERC-20 unit alphabet exceeds 64 characters"
    );
    let alphabet: String = alphabet.into_iter().map(char::from).collect();
    let encode_char = |byte| alphabet.bytes().position(|ch| ch == byte).unwrap() as u8;

    // Group tokens by decimals, unit length, ambiguity and encoding width.
    let mut grouped_tokens: BTreeMap<(u8, u8, bool, u8), Vec<&Token>> = BTreeMap::new();
    for token in &tokens {
        let bits_per_char = if token.unit.bytes().all(|byte| encode_char(byte) < 32) {
            5
        } else {
            6
        };
        grouped_tokens
            .entry((
                token.decimals,
                token.unit.len().try_into().unwrap(),
                contracts_by_unit[token.unit.as_str()].len() > 1,
                bits_per_char,
            ))
            .or_default()
            .push(token);
    }

    let out_filename = Path::new(&std::env::var("OUT_DIR").unwrap()).join("tokens.rs");
    let mut output_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(out_filename)
        .unwrap();

    writeln!(output_file, "const CONTRACT_ADDRESSES: &[[u8; 20]] = &[").unwrap();
    for tokens in grouped_tokens.values_mut() {
        // Sort by contract address so we can look up by contract
        // address more efficiently.
        tokens.sort_by_key(|token| token.contract_address);
        for token in tokens.iter() {
            writeln!(
                output_file,
                "    *b\"{}\",",
                token
                    .contract_address
                    .iter()
                    .map(|byte| format!("\\x{:02x}", byte))
                    .collect::<String>(),
            )
            .unwrap();
        }
    }
    writeln!(output_file, "];").unwrap();
    let mut packed_units = Vec::<u8>::new();
    let mut bit_offset = 0usize;
    // Emit one bit at a time; this runs only at build time.
    for ((_, _, _, bits_per_char), tokens) in &grouped_tokens {
        for byte in tokens.iter().flat_map(|token| token.unit.bytes()) {
            let code = encode_char(byte);
            for bit in 0..*bits_per_char {
                if bit_offset.is_multiple_of(8) {
                    packed_units.push(0);
                }
                packed_units[bit_offset / 8] |= ((code >> bit) & 1) << (bit_offset % 8);
                bit_offset += 1;
            }
        }
    }
    writeln!(
        output_file,
        "const UNIT_ALPHABET: &[u8] = b\"{}\";",
        alphabet.escape_default(),
    )
    .unwrap();
    writeln!(
        output_file,
        "const UNITS: &[u8] = b\"{}\";",
        packed_units
            .iter()
            .map(|byte| format!("\\x{byte:02x}"))
            .collect::<String>(),
    )
    .unwrap();

    writeln!(
        output_file,
        "const ALL: &[Group] = &[{}];",
        grouped_tokens
            .iter()
            .map(|((decimals, unit_len, unit_is_ambiguous, bits_per_char), tokens)| {
                let count: u16 = tokens.len().try_into().unwrap();
                format!("Group {{ count: {count}, decimals: {decimals}, unit_len: {unit_len}, unit_is_ambiguous: {unit_is_ambiguous}, bits_per_char: {bits_per_char} }}")
            })
            .collect::<Vec<String>>()
            .join(", ")
    )
    .unwrap();
}
