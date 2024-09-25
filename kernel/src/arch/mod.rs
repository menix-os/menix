#[cfg(target_arch = "x86_64")]
pub mod amd64;

#[cfg(target_arch = "aarch64")]
pub mod arm64;

#[cfg(target_arch = "riscv64")]
pub mod riscv64;

#[cfg(target_arch = "loongarch64")]
pub mod loong64;
