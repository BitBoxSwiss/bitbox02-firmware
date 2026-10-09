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

    // Group tokens by decimals and unit length.
    let mut grouped_tokens: BTreeMap<(u8, u8), Vec<&Token>> = BTreeMap::new();
    for token in &tokens {
        grouped_tokens
            .entry((token.decimals, token.unit.len().try_into().unwrap()))
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

    // BTreeMap iteration keeps this list sorted for binary search at runtime.
    writeln!(output_file, "const AMBIGUOUS_UNITS: &[&str] = &[").unwrap();
    for (unit, contracts) in &contracts_by_unit {
        if contracts.len() > 1 {
            writeln!(output_file, "    \"{}\",", unit.escape_default()).unwrap();
        }
    }
    writeln!(output_file, "];\n").unwrap();

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
    writeln!(
        output_file,
        "const UNITS: &str = \"{}\";",
        units.escape_default(),
    )
    .unwrap();

    writeln!(
        output_file,
        "const ALL: &[Group] = &[{}];",
        grouped_tokens
            .iter()
            .map(|((decimals, unit_len), tokens)| {
                let count: u16 = tokens.len().try_into().unwrap();
                format!("Group {{ count: {count}, decimals: {decimals}, unit_len: {unit_len} }}")
            })
            .collect::<Vec<String>>()
            .join(", ")
    )
    .unwrap();
}
