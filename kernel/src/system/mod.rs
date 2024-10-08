#[cfg(all(
    feature = "fw_acpi",
    any(
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "riscv64",
        target_arch = "loongarch64"
    )
))]
mod acpi;

#[cfg(all(
    feature = "fw_open_firmware",
    any(target_arch = "aarch64", target_arch = "riscv64")
))]
mod of;

#[cfg(feature = "fw_pci")]
mod pci;
