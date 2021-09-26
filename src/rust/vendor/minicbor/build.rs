/// Targets with max. atomic width = 32:
const ATOMIC32: &[&str] = &[
    "arm",
    "mips",
    "powerpc",
    "riscv32imac-unknown-none-elf",
    "riscv32imc-unknown-none-elf",
    "thumbv7em-none-eabi",
    "thumbv7m-none-eabi",
    "thumbv8m.base-none-eabi",
    "thumbv8m.main-none-eabi"
];

/// Targets with max. atomic width = 64:
const ATOMIC64: &[&str] = &[
    "x86_64",
    "aarch64",
    "arm-unknown-linux-gnueabi",
    "arm-unknown-linux-musleabi",
    "armv7-linux-androideabi",
    "armv7-unknown-linux-gnueabi",
    "armv7-unknown-linux-musleabi",
    "armv7a-none-eabi",
    "armv7r-none-eabi",
    "asmjs-unknown-emscripten",
    "i586",
    "i686",
    "mips64",
    "mips64el",
    "nvptx64-nvidia-cuda",
    "powerpc64",
    "powerpc64le",
    "riscv64",
    "s390x",
    "sparc64",
    "thumbv7neon-linux-androideabi",
    "thumbv7neon-unknown-linux-gnueabihf",
    "wasm32"
];

fn main() {
    let target = std::env::var("TARGET").unwrap();

    if ATOMIC64.iter().any(|a| target.starts_with(a)) {
        println!("cargo:rustc-cfg=atomic64");
    } else if ATOMIC32.iter().any(|a| target.starts_with(a)) {
        println!("cargo:rustc-cfg=atomic32");
    }
}

