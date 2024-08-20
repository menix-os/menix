# Modules

The modules/ directory contains all code that is loaded and activated at runtime,
after the main kernel has been initialized.

That encompasses:
- Kernel extensions
- Device drivers
- File systems

These modules can choose to be loaded into memory at runtime, or be compiled
directly into the kernel binary.

## Device drivers
Device drivers are grouped by function. For example, all USB drivers are in `/modules/drv/usb/`.

