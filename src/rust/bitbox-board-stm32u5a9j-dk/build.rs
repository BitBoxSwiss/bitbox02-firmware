// SPDX-License-Identifier: Apache-2.0

use std::env;
use std::path::{Path, PathBuf};

#[allow(dead_code)]
#[path = "src/memory.rs"]
mod memory;

fn hex_addr(addr: usize) -> String {
    format!("0x{addr:08X}")
}

fn len_expr(len: usize) -> String {
    if len.is_multiple_of(1024) {
        format!("{}K", len / 1024)
    } else {
        len.to_string()
    }
}

fn region(name: &str, attrs: &str, origin: usize, len: usize) -> String {
    format!(
        "  {name:<22} ({attrs}) : ORIGIN = {}, LENGTH = {}\n",
        hex_addr(origin),
        len_expr(len)
    )
}

fn memory_x() -> String {
    let mut out = String::from("MEMORY\n{\n");
    out.push_str(&region(
        "BOOT_ARGS",
        "xrw",
        memory::BOOT_ARGS_ADDR,
        memory::BOOT_ARGS_LEN,
    ));
    out.push_str(&region("RAM", "xrw", memory::RAM_ADDR, memory::RAM_LEN));
    out.push_str(&region(
        "SRAM4",
        "xrw",
        memory::SRAM4_ADDR,
        memory::SRAM4_LEN,
    ));
    out.push_str(&region(
        "BOOT0_FLASH",
        "rx",
        memory::BOOT0_ADDR,
        memory::BOOT0_MAX_LEN,
    ));
    out.push_str(&region(
        "BOOT1_FLASH",
        "rx",
        memory::BOOT1_ADDR,
        memory::BOOT1_MAX_LEN,
    ));
    out.push_str(&region(
        "FW_FLASH",
        "rx",
        memory::FIRMWARE_ADDR,
        memory::FIRMWARE_MAX_LEN,
    ));
    out.push_str(&region(
        "VENDOR_DATA",
        "rx",
        memory::VENDOR_DATA_ADDR,
        memory::VENDOR_DATA_LEN,
    ));
    out.push_str(&region(
        "USER_DATA",
        "rx",
        memory::USER_DATA_ADDR,
        memory::USER_DATA_LEN,
    ));
    out.push_str(&region(
        "DFU_BOOT1",
        "rx",
        memory::DFU_BOOT1_ADDR,
        memory::DFU_BOOT1_MAX_LEN,
    ));
    out.push_str(&region("GRAM", "xrw", memory::GRAM_ADDR, memory::GRAM_LEN));
    out.push_str("}\n");
    out
}

fn generate_memory_x(out_dir: &Path, manifest_dir: &Path) {
    let source = manifest_dir.join("src/memory.rs");
    println!("cargo::rerun-if-changed={}", source.display());
    std::fs::write(out_dir.join("memory.x"), memory_x()).expect("write memory layout script");
}

fn main() {
    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));

    generate_memory_x(&out_dir, &manifest_dir);
    println!("cargo::rustc-link-search={}", out_dir.display());
}
