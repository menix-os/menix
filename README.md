<p align="center">
<img src="menix.svg" width="20%"/>
</p>

![GitHub License](https://img.shields.io/github/license/menix-os/menix?style=flat&color=red)
![GitHub Repo stars](https://img.shields.io/github/stars/menix-os/menix?style=flat)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/menix-os/menix/ci.yml)
![GitHub Issues or Pull Requests](https://img.shields.io/github/issues/menix-os/menix?style=flat)

# About Menix
Menix is a modular 64-bit kernel written in Rust.

Its goal is to provide a familiar POSIX/Linux-like user interface,
with an easy to follow code structure and an emphasis on stable operation.

# Getting started

> [!IMPORTANT]
> Please note that this project is a work in progress
> and some parts may not work yet at all.

> [!NOTE]
> This repository contains only the kernel and drivers.
> If you want to get a bootable image, you might want to check out
> **https://github.com/menix-os/bootstrap** instead.

## Cloning the repository
Menix has external submodules as dependencies, to initialize them either run:

```sh
git clone https://github.com/menix-os/menix --recurse-submodules
```

or if you've already cloned the repository:

```sh
git submodule update --init --recursive
```

## Building the kernel

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

## Debugging

Follow the debugging setup from **https://github.com/menix-os/bootstrap**

# Contributing

Contributions are _always_ welcome!
Please read the **[contributing guide](docs/src/contributing.md)** first.
Then open an issue or pull request and submit your changes!
