FROM archlinux/archlinux

RUN pacman -Syuu cmake ninja clang gcc --noconfirm

COPY include /app/include
COPY kernel /app/kernel
COPY modules /app/modules
COPY CMakeLists.txt /app/CMakeLists.txt
COPY toolchain /app/toolchain

WORKDIR /app
CMD cmake -D CMAKE_BUILD_TYPE=Release -B build -G "Ninja" -D CMAKE_C_COMPILER=$CC -D MENIX_ARCH=$ARCH; cmake --build build
