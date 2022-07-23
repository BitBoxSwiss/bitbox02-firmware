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

use clap::Clap;

#[derive(Clap)]
struct Opts {
    #[clap(long)]
    messages_dir: String,
    #[clap(long)]
    out_dir: String,
}

fn main() {
    let opts: Opts = Opts::parse();
    let mut config = prost_build::Config::new();
    config.out_dir(opts.out_dir);
    config
        .compile_protos(&["hww.proto", "backup.proto"], &[&opts.messages_dir])
        .unwrap();
}
