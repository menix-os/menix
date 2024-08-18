# Driver subsystems (Not drivers themselves, see `/modules/`!)

add_library(menix_drv STATIC module.c)

add_subdirectory(acpi)
add_subdirectory(pci)

target_link_libraries(menix_drv
	menix_drv_pci
	menix_drv_acpi
)
