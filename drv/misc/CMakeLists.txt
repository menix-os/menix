# ? Misc Drivers and utility

set(MENIX_PARENT_CAT menix_drv_misc)
add_library(${MENIX_PARENT_CAT} INTERFACE)
include(${MENIX_UTIL_PATH})

# Bootloader selection
build_module(boot_efi)
build_module(boot_limine)

build_module(example)
