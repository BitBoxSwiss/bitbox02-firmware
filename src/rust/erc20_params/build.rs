// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::format_collect)]

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

    // Group tokens by decimals, unit length and ambiguity.
    let mut grouped_tokens: BTreeMap<(u8, u8, bool), Vec<&Token>> = BTreeMap::new();
    for token in &tokens {
        grouped_tokens
            .entry((
                token.decimals,
                token.unit.len().try_into().unwrap(),
                contracts_by_unit[token.unit.as_str()].len() > 1,
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
    let units: String = grouped_tokens
        .values()
        .flatten()
        .map(|token| token.unit.as_str())
        .collect();
    let alphabet: String = units.chars().collect::<BTreeSet<_>>().into_iter().collect();
    assert!(
        alphabet.len() <= 64,
        "ERC-20 unit alphabet exceeds 64 characters"
    );
    let mut packed_units = vec![0u8; (units.len() * 6).div_ceil(8)];
    for (index, byte) in units.bytes().enumerate() {
        let code = alphabet.as_bytes().binary_search(&byte).unwrap() as u8;
        let bit_offset = index * 6;
        let shift = bit_offset % 8;
        packed_units[bit_offset / 8] |= code << shift;
        if shift > 2 {
            packed_units[bit_offset / 8 + 1] |= code >> (8 - shift);
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
            .map(|((decimals, unit_len, unit_is_ambiguous), tokens)| {
                let count: u16 = tokens.len().try_into().unwrap();
                format!("Group {{ count: {count}, decimals: {decimals}, unit_len: {unit_len}, unit_is_ambiguous: {unit_is_ambiguous} }}")
            })
            .collect::<Vec<String>>()
            .join(", ")
    )
    .unwrap();
}
