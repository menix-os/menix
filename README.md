# Menix

Menix is a modular 64-bit kernel written in Rust.

## Getting started

> [!NOTE]
> This repository contains only the kernel and drivers.
> If you want to get a bootable image, you might want to check out
> **https://github.com/menix-os/bootstrap** instead.

### Cloning the repository

Menix has external submodules as dependencies, to initialize them either run:

```sh
git clone https://github.com/menix-os/menix --recurse-submodules
```

or if you've already cloned the repository:

```sh
git submodule update --init --recursive
```

### Building the kernel

To build the kernel you will need:
- cargo
- rustc
- clang (Used for bindgen)
- lld

Make sure you have a full nightly toolchain installed,
including the `rust-src` component.

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
