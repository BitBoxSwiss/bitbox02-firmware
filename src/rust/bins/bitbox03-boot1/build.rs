use std::path::{Path, PathBuf};
use std::process::Command;

const IMAGE_HEADER_LEN: usize = 1024;

fn run_command(command: &mut Command, description: &str) {
    let status = command.status().unwrap_or_else(|err| {
        panic!("failed to execute {description}: {err}");
    });
    assert!(
        status.success(),
        "{description} failed with status {status}"
    );
}

fn generate_header_object(manifest_dir: &Path, out_dir: &Path) {
    let repo_root = manifest_dir.join("../../../..");
    let script = repo_root.join("scripts/bitbox03_image_header.py");
    let header_manifest = manifest_dir.join("image_header.json");
    let header_bin = out_dir.join("bitbox03-boot1-header.bin");
    let header_object = out_dir.join("bitbox03-boot1-header.o");

    println!("cargo::rerun-if-changed={}", script.display());
    println!("cargo::rerun-if-changed={}", header_manifest.display());

    run_command(
        Command::new("python3")
            .arg(&script)
            .arg("render-dev-header")
            .arg("--manifest")
            .arg(&header_manifest)
            .arg("--output")
            .arg(&header_bin),
        "render boot1 dev image header",
    );

    run_command(
        Command::new("arm-none-eabi-objcopy")
            .arg("-I")
            .arg("binary")
            .arg("-O")
            .arg("elf32-littlearm")
            .arg("-B")
            .arg("arm")
            .arg("--rename-section")
            .arg(".data=.image_header,alloc,load,readonly,data,contents")
            .arg(&header_bin)
            .arg(&header_object),
        "convert boot1 image header to object",
    );

    println!("cargo::rustc-link-arg={}", header_object.display());
}

fn main() {
    let target = std::env::var("TARGET").expect("TARGET not set");

    if target.starts_with("thumb") {
        if !cfg!(feature = "board-stm32u5a9j-dk") {
            panic!("select a BitBox03 board feature, e.g. `board-stm32u5a9j-dk`")
        }

        let manifest_dir =
            PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
        let out_dir = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR not set"));
        let lds_from = manifest_dir.join("bitbox03-boot1.ld");
        let lds_to = out_dir.join("bitbox03-boot1.ld");
        println!("cargo::rerun-if-changed={}", lds_from.display());
        std::fs::copy(lds_from, &lds_to).expect("copy linker script");

        generate_header_object(&manifest_dir, &out_dir);

        println!("cargo::rustc-link-search={}", out_dir.display());
        println!("cargo::rustc-link-arg=-Tbitbox03-boot1.ld");
        println!(
            "cargo::rustc-link-arg=-Map={}",
            out_dir.join("bitbox03-boot1.map").display()
        );
    }

    println!("cargo::rustc-env=BITBOX03_IMAGE_HEADER_LEN={IMAGE_HEADER_LEN}");
}
