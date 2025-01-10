# Building kernel

This document explains how to build the `menix` kernel on your machine from source code.

## Dependencies

You need to install the following packages and make sure they're available in `$PATH`.

- `cmake`
- Either `clang` or a `gcc` for the target architecture
- Binutils for your toolchain

## Building steps

Create a `build` directory and configure with CMake.

```sh
cmake -D CMAKE_BUILD_TYPE=Release -B build
# To cross-compile with clang:
cmake -D CMAKE_BUILD_TYPE=Release -B build -D MENIX_ARCH="<arch>" -D CMAKE_C_COMPILER="clang"
```

Then, to build:

```sh
cmake --build build
```

The final executable and dynamic modules are stored in `build/bin/`.

## Building using Docker

```sh
cd tools
docker compose build
docker compose up
```

### Cross-compilation with Docker

```sh
cd tools
docker compose build
docker compose run -e ARCH="<arch>" builder
```
