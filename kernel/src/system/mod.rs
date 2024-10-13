#[cfg(all(
    feature = "sys_acpi",
    any(
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "riscv64",
        target_arch = "loongarch64"
    )
))]
pub mod acpi;

#[cfg(all(
    feature = "sys_open_firmware",
    any(target_arch = "aarch64", target_arch = "riscv64")
))]
pub mod of;

#[cfg(feature = "sys_pci")]
pub mod pci;

#[cfg(feature = "sys_video")]
pub mod video;

pub mod error;
