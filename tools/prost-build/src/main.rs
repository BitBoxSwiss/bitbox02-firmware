// Copyright 2020 Shift Cryptos AG
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

struct Opts {
    messages_dir: String,
    out_dir: String,
}

fn parse_args() -> Result<Opts, lexopt::Error> {
    use lexopt::prelude::*;

    let mut messages_dir = String::new();
    let mut out_dir = String::new();

    let mut parser = lexopt::Parser::from_env();
    while let Some(arg) = parser.next()? {
        match arg {
            Long("messages-dir") => messages_dir = parser.value()?.into_string()?,
            Long("out-dir") => out_dir = parser.value()?.into_string()?,
            Short('h') | Long("help") => {
                println!("Usage: prost-build --messages-dir <messages-dir> --out-dir <out-dir>");
                std::process::exit(0);
            },
            _ => return Err(arg.unexpected())
        }
    }
    Ok(Opts {messages_dir, out_dir})
}

fn main() {
    let opts = parse_args().unwrap();
    let mut config = prost_build::Config::new();
    config.out_dir(opts.out_dir);
    config
        .compile_protos(&["hww.proto"], &[&opts.messages_dir])
        .unwrap();
}
