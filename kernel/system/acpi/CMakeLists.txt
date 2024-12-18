# ACPI driver subsystem

target_sources(menix PUBLIC
	rsdt.c
	mcfg.c
	madt.c
	uacpi_interface.c
)

# uACPI
include(uacpi/uacpi.cmake)
target_include_directories(menix PUBLIC
	${UACPI_INCLUDES}
)
target_sources(menix PUBLIC
	${UACPI_SOURCES}
)