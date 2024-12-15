# System hardware management

target_sources(menix PUBLIC
	device.c
	elf.c
	isr.c
	module.c
	terminal.c
)

add_subdirectory(acpi)
add_subdirectory(bus)
add_subdirectory(dst)
add_subdirectory(net)
add_subdirectory(sch)
add_subdirectory(time)
add_subdirectory(video)
