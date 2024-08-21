# Driver subsystems (Not drivers themselves, see `/modules/`!)

add_library(menix_drv module.c)

add_subdirectory(acpi)
add_subdirectory(pci)

# TODO: Linker errors otherwise.
target_link_libraries(menix_drv_acpi menix_drv_pci)

target_link_libraries(menix_drv
	menix_drv_pci
	menix_drv_acpi
)
