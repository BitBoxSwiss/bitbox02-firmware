// SPDX-License-Identifier: Apache-2.0

mod core;
mod graphical;
mod headless;
mod signals;

use clap::Parser;
use std::error::Error;
use tracing_subscriber::{EnvFilter, filter::LevelFilter, fmt, prelude::*};

// Explicitly link library for its C exports.
extern crate bitbox02_rust_c;

#[derive(Parser)]
#[command(about = "BitBox02 simulator", disable_version_flag = true)]
pub struct Args {
    /// Run without a window and automatically answer prompts.
    #[arg(long)]
    nogui: bool,
    /// Preseed with the documented simulator mnemonic and an empty password.
    #[arg(long)]
    preseed: bool,
    /// TCP port to listen on for the host connection.
    #[arg(long, default_value_t = 15423)]
    port: u16,
    /// Print the integration-test simulator version.
    #[arg(long)]
    version: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    if args.version {
        println!(
            "bitbox02-multi-{}-simulator1.0.0-linux-amd64",
            bitbox02_rust::version::FIRMWARE_VERSION_SHORT
        );
        return Ok(());
    }
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();
    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(std::io::stderr))
        .with(filter)
        .init();
    signals::install()?;
    if args.nogui {
        headless::run(&args)?;
    } else {
        graphical::run(&args)?;
    }
    Ok(())
}
