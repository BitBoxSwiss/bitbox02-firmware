use std::path::{Path, PathBuf};

#[cfg(feature = "board-stm32u5a9j-dk")]
fn generate_memory_x(out_dir: &Path) {
    use bitbox_board_stm32u5a9j_dk::memory;

    let contents = format!(
        concat!(
            "MEMORY\n",
            "{{\n",
            "  BOOT_ARGS (xrw) : ORIGIN = 0x{:08X}, LENGTH = {}\n",
            "  RAM (xrw)       : ORIGIN = 0x{:08X}, LENGTH = {}\n",
            "  RAM_CODE (xrw)  : ORIGIN = 0x{:08X}, LENGTH = {}\n",
            "}}\n",
        ),
        memory::BOOT_ARGS_ADDR,
        memory::BOOT_ARGS_LEN,
        memory::FACTORYSETUP_RAM_ADDR,
        memory::FACTORYSETUP_RAM_LEN,
        memory::FACTORYSETUP_RAM_CODE_ADDR,
        memory::FACTORYSETUP_RAM_CODE_LEN,
    );
    std::fs::write(out_dir.join("memory.x"), contents).expect("write memory layout script");
}

fn build_board_hal_overrides_object(repo_root: &Path, out_dir: &Path) {
    #[cfg(feature = "board-stm32u5a9j-dk")]
    bitbox_board_stm32u5a9j_dk_build::build_hal_overrides_object(repo_root, out_dir);
    #[cfg(not(feature = "board-stm32u5a9j-dk"))]
    let _ = (repo_root, out_dir);
}

fn main() {
    if !cfg!(feature = "board-stm32u5a9j-dk") {
        panic!("select a BitBox03 board feature, e.g. `board-stm32u5a9j-dk`")
    }

    let target = std::env::var("TARGET").expect("TARGET not set");
    if target.starts_with("thumb") {
        let manifest_dir =
            PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
        let out_dir = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR not set"));
        let repo_root = manifest_dir.join("../../../..");

        let lds_from = manifest_dir.join("bitbox03-factorysetup.ld");
        let lds_to = out_dir.join("bitbox03-factorysetup.ld");
        println!("cargo::rerun-if-changed={}", lds_from.display());
        std::fs::copy(lds_from, &lds_to).expect("copy linker script");

        generate_memory_x(&out_dir);

        println!("cargo::rustc-link-search={}", out_dir.display());
        println!(
            "cargo::rustc-link-arg=-Map={}",
            out_dir.join("bitbox03-factorysetup.map").display()
        );

        println!("cargo::rustc-link-arg=-Tbitbox03-factorysetup.ld");

        build_board_hal_overrides_object(&repo_root, &out_dir);
    }
}
