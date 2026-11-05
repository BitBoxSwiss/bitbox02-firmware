# [Documentation](https://docs.rs/stm32-fmc)

# stm32-fmc

[![docs.rs](https://docs.rs/stm32-fmc/badge.svg)](https://docs.rs/stm32-fmc)
[![Crates.io](https://img.shields.io/crates/v/stm32-fmc.svg)](https://crates.io/crates/stm32-fmc)

Hardware Abstraction Layer for STM32 Memory Controllers (FMC/FSMC)

Currently only SDRAM functions are implemented.

**This crate is a work in progress! Contributions very welcome**

## Implementing

(If your HAL already implements FMC, you can skip this)

See the [docs](https://docs.rs/stm32-fmc)

# Usage

### SDRAM

The FMC peripheral supports up to 2 external SDRAM devices. This crate currently
only supports 1, although it may be on either bank 1 or 2.

External memories are defined by
[`SdramChip`](https://docs.rs/stm32-fmc/latest/stm32_fmc/trait.SdramChip.html)
implementations. There are several examples in the [`devices`](src/devices/)
folder, or you can make your own.

To pass pins to a constructor, create a tuple with the following ordering:

```rust
let pins = (
    // A0-A12
    pa0, ...
    // BA0-BA1
    // D0-D31
    // NBL0 - NBL3
    // SDCKE
    // SDCLK
    // SDNCAS
    // SDNE
    // SDRAS
    // SDNWE
);
```

You can leave out address/data pins not used by your memory.

#### Constructing

If you are using a HAL, see the HAL documentation.

Otherwise you can implement
[`FmcPeripheral`](https://docs.rs/stm32-fmc/latest/stm32_fmc/trait.FmcPeripheral.html)
yourself then use
[`Sdram::new`](https://docs.rs/stm32-fmc/latest/stm32_fmc/struct.Sdram.html#method.new)
/
[`Sdram::new_unchecked`](https://docs.rs/stm32-fmc/latest/stm32_fmc/struct.Sdram.html#method.new_unchecked)
directly.

#### Initialising

Once you have an
[`Sdram`](https://docs.rs/stm32-fmc/latest/stm32_fmc/struct.Sdram.html)
instance, you can:

* Initialise it by calling
  [`init`](https://docs.rs/stm32-fmc/latest/stm32_fmc/struct.Sdram.html#method.init). This
  returns a raw pointer
* Convert the raw pointer to a sized slice using `from_raw_parts_mut`

```rust
let ram = unsafe {
    // Initialise controller and SDRAM
    let ram_ptr: *mut u32 = sdram.init(&mut delay);

    // 32 MByte = 256Mbit SDRAM = 8M u32 words
    slice::from_raw_parts_mut(ram_ptr, 8 * 1024 * 1024)
};
```

### NAND Flash

The FMC peripheral supports once external parallel NAND flash device.

External memories are defined by
[`NandChip`](https://docs.rs/stm32-fmc/latest/stm32_fmc/trait.NandChip.html)
implementations. There are examples in the [`devices`](src/devices/) folder, or
you can make your own.

To pass pins to a constructor, create a tuple with the following ordering:

```rust
let pins = (
    // A17/ALE
    // A16/CLE
    pa0, ...
    // D0-D7
    // NCE/#CE
    // NOE/#RE
    // NWE/#WE
    // NWAIT/R/#B
);
```

#### Constructing

If you are using a HAL, see the HAL documentation.

Otherwise you can implement
[`FmcPeripheral`](https://docs.rs/stm32-fmc/latest/stm32_fmc/trait.FmcPeripheral.html)
yourself then use
[`Nand::new`](https://docs.rs/stm32-fmc/latest/stm32_fmc/struct.Nand.html#method.new)
/
[`Nand::new_unchecked`](https://docs.rs/stm32-fmc/latest/stm32_fmc/struct.Nand.html#method.new_unchecked)
directly.

#### Initialising

Once you have an
[`Nand`](https://docs.rs/stm32-fmc/latest/stm32_fmc/struct.Nand.html) instance
you should initialise it by calling
[`init`](https://docs.rs/stm32-fmc/latest/stm32_fmc/struct.Nand.html#method.init). This
returns a
[`NandDevice`](https://docs.rs/stm32-fmc/latest/stm32_fmc/nand_device/struct.NandDevice.html)
instance.

```rust
let mut nand_device = nand.init(&mut delay);

// Read device identifier
let id = nand_device.read_id();
```

### NOR Flash/PSRAM

TODO

### Troubleshooting
The library automatically does some trace-level logging either via `log` or via `defmt`.
To enable such logging, enable either the `log` or `defmt` feature in your `Cargo.toml`.

For debugging the SDRAM register contents, the library provides additional feature `trace-register-values`, which when enabled causes the init function to log the register contents to the trace level.
This is useful for example when you want to compare the register values between `stm32-fmc` and CubeMX code.
Note that one of the logging features (`log`/`defmt`) must be enabled for this to work.

### Implementing a new device

If you end up depending on a fork or a newer version of this crate than the
HAL crate for your device, you can override the version pulled in by the
external crate using a `[patch]` section in your `Cargo.toml`, as described
in the
[Cargo Book](https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html#the-patch-section).

## Releasing

* Update Cargo.toml
* Update CHANGELOG.md

```
git commit -am 'v0.2.0'
git push --set-upstream origin v0.2.0
```

Create a PR and check CI passes

```
git push --set-upstream origin v0.2.0:master
git tag -a 'v0.2.0' -m 'v0.2.0'
git push origin refs/tags/v0.2.0
cargo publish
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
