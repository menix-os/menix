# Modules

set(MENIX_PARENT_CAT menix_builtin)
add_library(${MENIX_PARENT_CAT} INTERFACE)
include(${MENIX_UTIL_PATH})

# Prepare module build folder so we can create an archive, even if there's no modules to be packed.
file(MAKE_DIRECTORY "${CMAKE_BINARY_DIR}/bin/mod")

add_subdirectory(drv)
add_subdirectory(fs)
