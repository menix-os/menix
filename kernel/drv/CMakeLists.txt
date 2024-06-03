# ------------------
# Driver Sub-Systems
# ------------------

add_library(menix_drv INTERFACE)

include(${MENIX_UTIL_PATH})
include(${MENIX_CONFIG_SRC}) # Configuration

add_subdirectory(usb)
add_subdirectory(misc)

target_link_libraries(menix_drv INTERFACE
	menix_drv_usb
	menix_drv_misc
)
