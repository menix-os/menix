# Building
This document explains how to build `menix` on your machine from source code.

## Dependencies
You need to install the following packages and make sure they're available in `$PATH`.

```
cmake
clang
dtc
```

## Building steps
Create a build directory and configure CMake.

```sh
mkdir build
cd build
cmake ../ -D CMAKE_BUILD_TYPE=Release
# To cross-compile, add: -D MENIX_ARCH="<arch>"
```

This will generate `/kernel/config.cmake` for you with default values.

> **Note:**
>
> If you already have a `config.cmake` file, CMake won't override it.

Adjust its contents to your liking and save it.
Then, to build:
```sh
cmake --build .
```
