# Building
This document explains how to build `menix` on your machine from source code.

## Dependencies
You need to install the following packages and make sure they're available in `$PATH`.

- `cmake`
- `ninja`
- Either `clang` or `gcc`

## Building steps
Create a build directory and configure with CMake.

```sh
cmake -D CMAKE_BUILD_TYPE=Release -B build -G "Ninja"
# To cross-compile with clang:
cmake -D CMAKE_BUILD_TYPE=Release -B build -G "Ninja" -D MENIX_ARCH="<arch>" -D CMAKE_C_COMPILER="clang"
```

This will generate `config.cmake` for you with default values. That file is
used to generate the C headers used by the kernel and components.

> [!Note]
> If you already have a `config.cmake` file, CMake won't overwrite it.
> If you have made changes to the build system, you need to manually delete it.

Adjust the config to your liking and save it.
Then, to build:
```sh
cmake --build build
```

The final executable and dynamic modules are stored in `build/bin/`.
