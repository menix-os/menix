# Firmware management and initialization

target_sources(menix PUBLIC
	elf.c
	fw.c
	module.c
)

add_subdirectory(acpi)
add_subdirectory(pci)
add_subdirectory(net)
add_subdirectory(sch)
add_subdirectory(video)
