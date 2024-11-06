# menix
A minimal and expandable Unix-like kernel.

menix uses a monolithic architecture for many parts of the system, but can
load drivers as processes at runtime via dynamic linking, similar to an exokernel design.

> [!Important]
> This repository contains only the kernel and its driver daemons.
> You could run them on their own in an emulator like QEMU,
> but don't expect much to happen without a full system build and/or bootloader.
> If you just want to get a bootable image, you might want to check out
> **https://github.com/menix-os/bootstrap** instead.

## Getting started
Follow the **[build instructions](doc/src/building.md)** for building the kernel and drivers.

## Contributing
Contributions are _always_ welcome!
First, please read the **[contributing guide](doc/src/contributing.md)** to make sure
your changes fit the rest of the project.
Open an issue or pull request with the appropriate template and submit your changes.
