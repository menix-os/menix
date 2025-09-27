#[cfg(all(
    any(
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "riscv64",
        target_arch = "loongarch64"
    ),
    feature = "acpi"
))]
pub mod acpi;
pub mod dt;
pub mod pci;
