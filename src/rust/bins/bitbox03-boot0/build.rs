use std::path::PathBuf;

fn main() {
    let target = std::env::var("TARGET").expect("TARGET not set");
    if target.starts_with("thumb") {
        if cfg!(feature = "board-stm32u5a9j-dk") == cfg!(feature = "board-testboard") {
            panic!(
                "select exactly one BitBox03 board feature: `board-stm32u5a9j-dk` or `board-testboard`"
            )
        }

        let manifest_dir =
            PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
        let out_dir = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR not set"));

        let lds_from = manifest_dir.join("bitbox03-boot0.ld");
        let lds_to = out_dir.join("bitbox03-boot0.ld");
        println!("cargo::rerun-if-changed={}", lds_from.display());
        std::fs::copy(lds_from, &lds_to).expect("copy linker script");

        println!("cargo::rustc-link-search={}", out_dir.display());
        println!(
            "cargo::rustc-link-arg=-Map={}",
            out_dir.join("bitbox03-boot0.map").display()
        );
        println!("cargo::rustc-link-arg=-Tbitbox03-boot0.ld");
        if std::env::var("PROFILE").expect("PROFILE not set") == "release" {
            println!("cargo::rustc-link-arg=--defsym=__bitbox03_production=1");
        }
    }
}
