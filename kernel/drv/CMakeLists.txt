# Driver Sub-Systems

include(${MENIX_UTIL_PATH})
include(${MENIX_CONFIG_SRC})

add_library(menix_drv INTERFACE)

add_subdirectory(pci)
add_subdirectory(spi)
add_subdirectory(usb)
add_subdirectory(misc)

target_link_libraries(menix_drv INTERFACE
	menix_drv_pci
	menix_drv_spi
	menix_drv_usb
	menix_drv_misc
)
