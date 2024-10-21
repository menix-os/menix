# Boot procedure

The kernel must by loaded by an external bootloader. It has no booting capabilities
on its own. You can build the kernel with different boot protocols.

Each protocol has its own directory in `kernel/boot/<protocol>`.

The kernel assumes an UEFI environment at all times. It supports being booted
by real UEFI implementations (e.g. EDK2, coreboot) or pseudo-implementations
(e.g. U-Boot). That way there is less burden on the kernel to support a wide
range of hardware, especially for aarch64 and riscv64.

Note that this doesn't automatically mean that the kernel has to rely on
firmware features like ACPI. You could, for example, boot on a x86_64 machine
with OpenFirmware and a provided device tree.

Boot order:

- Bootloader loads the kernel into memory.
- `kernel_boot` is invoked.
- `kernel_boot` calls `arch_early_init`.
- `kernel_boot` fills the kernel structure `BootInfo`.
- `kernel_boot` calls `arch_init`.
- `kernel_boot` is now done and passes control to the kernel entry point.
- `kernel_main` is invoked.
- ...
- `kernel_main` returns back to `kernel_boot` if the init program terminated.
- `kernel_boot` invokes `arch_stop` to request a regular system shutdown.
- If the shutdown was not successful, `kernel_boot` hands back control to the bootloader.
