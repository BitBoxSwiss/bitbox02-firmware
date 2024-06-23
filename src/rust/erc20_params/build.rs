// Copyright 2024 Shift Crypto AG
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

#![allow(clippy::format_collect)]

use std::collections::BTreeMap;
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

    // Group tokens by decimals
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

    for ((decimals, unit_len), tokens) in &mut grouped_tokens {
        // Sort by contract address so we can look up by contract
        // address more efficiently.
        tokens.sort_by_key(|token| token.contract_address);
        writeln!(
            output_file,
            "const PARAMS_D{}_U{}: &[P] = &[",
            decimals, unit_len
        )
        .unwrap();
        for token in tokens {
            writeln!(
                output_file,
                "    P {{ unit: b\"{}\".as_ptr(), contract_address: *b\"{}\" }},",
                token.unit,
                token
                    .contract_address
                    .iter()
                    .map(|byte| format!("\\x{:02x}", byte))
                    .collect::<String>(),
            )
            .unwrap();
        }
        writeln!(output_file, "];").unwrap();
    }

    writeln!(
        output_file,
        "const ALL: &[(u8, u8, &[P])] = &[{}];",
        grouped_tokens
            .keys()
            .map(|(decimal, unit_len)| format!(
                "({}, {}, PARAMS_D{}_U{})",
                decimal, unit_len, decimal, unit_len
            ))
            .collect::<Vec<String>>()
            .join(", ")
    )
    .unwrap();
}
