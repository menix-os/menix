# ------------------
# Driver Sub-Systems
# ------------------

add_library(menix_drv INTERFACE)

include(${MENIX_UTIL_PATH})
include(${MENIX_CONFIG_SRC}) # Configuration

add_subdirectory(acpi)
add_subdirectory(gpu)
add_subdirectory(net)
add_subdirectory(pci)
add_subdirectory(spi)
add_subdirectory(usb)
add_subdirectory(misc)

target_link_libraries(menix_drv INTERFACE
	menix_drv_acpi
	menix_drv_gpu
	menix_drv_net
	menix_drv_pci
	menix_drv_spi
	menix_drv_usb
	menix_drv_misc
)
