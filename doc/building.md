# Building from source
This guide assumes either a Linux or menix installation.

1. Make sure you have installed the following tools:
```
git make rustup cargo dtc
```

2. Clone the Git repository.
```sh
$ git clone https://github.com/menix-os/menix
$ cd menix
```

3. Build the kernel.
```sh
$ cargo build --release
```

> [!Note]
> If no target is specified, cargo will select x86_64 as the default target.
> To cross-compile, specify the target file by appending `--target toolchain/<arch>.json`.
