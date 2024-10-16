# Firmware management and initialization

target_sources(menix PUBLIC
	fw.c
	module.c
)

add_subdirectory(acpi)
add_subdirectory(pci)
