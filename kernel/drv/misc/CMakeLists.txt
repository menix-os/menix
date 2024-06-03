# ------------
# Misc Drivers
# ------------

set(MENIX_PARENT_CAT menix_drv_misc)
add_library(${MENIX_PARENT_CAT} INTERFACE)
include(${MENIX_UTIL_PATH})

###########################

build_module(example)
