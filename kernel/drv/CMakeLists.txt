# Driver subsystems (Not drivers themselves, see `/mod/`!)

add_library(menix_drv INTERFACE)

add_subdirectory(acpi)
add_subdirectory(pci)

target_link_libraries(menix_drv INTERFACE
	menix_drv_pci
	menix_drv_acpi
)
