# Modules

set(MENIX_PARENT_CAT menix_modules)
add_library(menix_modules INTERFACE)
include(${MENIX_UTIL_PATH})

# Example module (not built by default)
add_subdirectory(example)

add_library(menix_mod module.c)
target_link_libraries(menix_mod menix_modules)
