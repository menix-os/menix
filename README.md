<img src="assets/menix.svg" width="10%"/>

![GitHub License](https://img.shields.io/github/license/menix-os/menix?style=flat&color=red)
![GitHub Repo stars](https://img.shields.io/github/stars/menix-os/menix?style=flat)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/menix-os/menix/ci.yml)
![GitHub Issues or Pull Requests](https://img.shields.io/github/issues/menix-os/menix?style=flat)

# Menix

Menix is a lightweight and modular Unix-like kernel targeting modern 64-bit devices.

> [!IMPORTANT]
> Please note that this project is a work in progress.
> Some parts may not work at all and/or will drastically change over time.

## Building

> [!NOTE]
> This repository contains only the kernel and drivers.
> If you want to get a bootable image, you might want to check out
> **https://github.com/menix-os/bootstrap** instead.

To build the kernel you need `meson` and either `gcc` or `clang`.
You will also need a tool that can parse `Kconfig`s, for example
[`kconfiglib`](https://github.com/ulfalizer/Kconfiglib).

First, run the `Kconfig` parser to generate a config file.
Make sure to provide the `ARCH` environment variable,
which is the architecture you're compiling for (see `kernel/arch/`).

For example with `kconfiglib`:
```sh
ARCH=x86_64 python3 -m alldefconfig
```

By default, the searched for file name is `.config`, but you can change that
by adding `-Dconfig=<path>` when configuring.

To configure, run:
```sh
meson setup $build_dir
```

And to build:
```sh
meson compile -C $build_dir
```

To cross-compile, you should follow the Meson cross-compilation guide.

## Debugging

Follow the debugging guide at **https://github.com/menix-os/bootstrap**
