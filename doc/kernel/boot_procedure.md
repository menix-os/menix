# Boot procedure

The kernel must by loaded by an external bootloader. It has no booting capabilities
on its own. You can build the kernel with different boot protocols.

Each protocol has its own directory in `kernel/boot/<protocol>`.

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
