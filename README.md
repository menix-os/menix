# Menix

Menix is a modern, lightweight, pragmatic, asynchronous 64-bit microkernel

## Contributing

Contributions are _always_ welcome!
Please read the **[contributing guide](docs/src/contributing.md)** first.
Then open an issue or pull request and submit your changes!

## Getting started

> [!Important]
> This repository contains only the kernel and servers.
> If you want to get a bootable image, you might want to check out
> **https://github.com/menix-os/bootstrap** instead.

### Building the kernel
The following commmand will build the kernel:
```sh
cargo +nightly build --release \
    --target toolchain/x86_64.json \
    --manifest-path kernel/Cargo.toml
```

### Building the servers
The following command will build all servers:
```sh
cargo +nightly build --release \
    --target toolchain/x86_64.json \
    --manifest-path servers/Cargo.toml
```

### Debugging

There's a CodeLLDB script you can use to debug the kernel with LLDB/VS Code.
It assumes you have built the kernel in-tree, in debug mode.
