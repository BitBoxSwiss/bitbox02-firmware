# rtt-target

[![crates.io](https://img.shields.io/crates/v/rtt-target.svg)](https://crates.io/crates/rtt-target) [![documentation](https://docs.rs/rtt-target/badge.svg)](https://docs.rs/rtt-target)

Target side implementation of the RTT (Real-Time Transfer) I/O protocol. RTT implements input and output via a debug probe using in-memory ring buffers and polling. This enables debug logging from the microcontroller with minimal delays and no blocking, making it usable even in real-time applications where e.g. semihosting delays cannot be tolerated.

## [Documentation](https://docs.rs/rtt-target)

## Platform support

To use the global `rprintln!` macro, a platform-specific [`critical-section`](https://github.com/rust-embedded/critical-section) implementation is needed for locking.

Output directly to a channel object with `write!` or the binary `write` method does not require locking and therefore does not need any platform-specific critical section.

## Usage

With a platform-specific critical section in use, printing is as simple as:

```rust
use rtt_target::{rtt_init_print, rprintln};

fn main() {
    rtt_init_print!();
    loop {
        rprintln!("Hello, world!");
    }
}
```

## Development

The examples-cortex-m and panic-test crates come with build files for the venerable STM32F103C8xx by default, but can be easily adapted for any chip as they contain only minimal platform-specific runtime code to get `fn main` to run.
