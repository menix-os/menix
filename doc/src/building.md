# Building from source

This document explains how to build the kernel on your machine from source code.

## Dependencies

You need to install the following packages and make sure they're available in `$PATH`.

- `cmake`
- Either `clang` or a `gcc` for the target architecture
  >Make sure your compiler has gnu2x (C23) support and is recent enough!
- GNU Binutils for the target architecture

## Building steps

Create a `build` directory and configure with CMake.

```sh
cmake -D CMAKE_BUILD_TYPE=Release -B build
```

Then, to build:

```sh
cmake --build build
```

The final executable and dynamic modules are stored in `build/bin/` and `build/bin/modules/` respectively.
