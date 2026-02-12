use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
    let repo_root = manifest_dir.join("../../..");
    let lv_conf = repo_root.join("src/lvgl/lv_conf.h");
    println!("cargo::rustc-env=LV_CONF_PATH={}", lv_conf.display());

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));

    let lds_from = manifest_dir.join("bitbox03-firmware.ld");
    let lds_to = out_dir.join("bitbox03-firmware.ld");
    std::fs::copy(lds_from, lds_to).expect("copy linker script");

    let mem_from = manifest_dir.join("memory.x");
    let mem_to = out_dir.join("memory.x");
    std::fs::copy(mem_from, mem_to).expect("copy memory layout script");

    // Search paths to linker scripts
    println!("cargo::rustc-link-search={}", out_dir.display());

    println!("cargo::rustc-link-arg=-Wl,-Map={}", out_dir.join("bitbox03-firmware.map").display())
}
