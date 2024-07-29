# Boot protocols

set(MENIX_PARENT_CAT menix_boot)
add_library(${MENIX_PARENT_CAT} INTERFACE)
include(${MENIX_UTIL_PATH})

build_module(boot_limine)
build_module(boot_mb2)
