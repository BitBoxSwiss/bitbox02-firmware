// SPDX-License-Identifier: Apache-2.0

use std::env;

fn main() -> Result<(), i32> {
    let args: Vec<String> = env::args().collect();
    let messages_dir = &args[1];
    let out_dir = &args[2];
    let mut config = prost_build::Config::new();
    config.out_dir(out_dir);
    if let Err(e) = config.compile_protos(&["hww.proto", "backup.proto"], &[messages_dir]) {
        eprintln!("{e}");
        return Err(1);
    }
    Ok(())
}
