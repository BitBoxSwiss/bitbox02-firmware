#[cfg(target_arch = "x86_64")]
mod x86_64;
#[cfg(target_arch = "x86_64")]
pub use x86_64::*;

#[cfg(target_arch = "x86")]
mod x86;
#[cfg(target_arch = "x86")]
pub use x86::*;

#[cfg(target_arch = "riscv64")]
mod riscv64;
#[cfg(target_arch = "riscv64")]
pub use riscv64::*;

#[cfg(target_arch = "riscv32")]
mod riscv32;
#[cfg(target_arch = "riscv32")]
pub use riscv32::*;

#[cfg(target_arch = "aarch64")]
mod aarch64;
#[cfg(target_arch = "aarch64")]
pub use aarch64::*;

#[cfg(not(any(
    target_arch = "x86_64",
    target_arch = "x86",
    target_arch = "riscv64",
    target_arch = "riscv32",
    target_arch = "aarch64"
)))]
compile_error!("Current architecture is not supported");
