# Building
This document explains how to build `menix` on your machine from source code.

## Dependencies
You need to install the following packages and make sure they're available in `$PATH`.

```
cmake
clang
```

You also need to install the `gnu-efi` package as a build dependency.

## Building steps
Create a build directory and configure CMake.

```sh
cmake . -B build/
# To cross-compile, add: -DMENIX_ARCH="<arch>"
```

This will generate `config.cmake` for you with default values.

> **Note:**
>
> If you already have a `config.cmake` file, CMake won't override it.

Adjust its contents to your liking and save it.
Then, to build:
```sh
cmake --build build/
```
