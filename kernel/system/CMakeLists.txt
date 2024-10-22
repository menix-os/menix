# Firmware management and initialization

target_sources(menix PUBLIC
	fw.c
	module.c
)

add_subdirectory(acpi)
add_subdirectory(pci)
add_subdirectory(net)
add_subdirectory(video)
