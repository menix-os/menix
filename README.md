# Menix

Menix is a modern, lightweight, pragmatic, asynchronous, modular 64-bit kernel.

## Getting started

> [!Important]
> This repository contains only the kernel and drivers.
> If you want to get a bootable image, you might want to check out
> **https://github.com/menix-os/bootstrap** instead.

### Building the kernel
To build the kernel you will need:
- cargo
- rustc
- clang (Used for bindgen)
- binutils

The following commmand will build the kernel and all drivers for x86_64:
```sh
cargo +nightly build --release --target toolchain/x86_64-kernel.json
```

### Debugging

There's a CodeLLDB script you can use to debug the kernel with LLDB/VS Code.
It assumes you have built the kernel in-tree, in debug mode.

## Contributing

Contributions are _always_ welcome!
Please read the **[contributing guide](docs/src/contributing.md)** first.
Then open an issue or pull request and submit your changes!
