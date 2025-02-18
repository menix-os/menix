# ACPI firmware configuration

target_sources(menix PUBLIC
	mcfg.c
	uacpi_interface.c
)

# uACPI
include(uacpi/uacpi.cmake)
add_library(uacpi ${UACPI_SOURCES})
target_link_libraries(uacpi PRIVATE common common_kernel)
target_compile_definitions(uacpi PRIVATE UACPI_OVERRIDE_LIBC)
target_include_directories(uacpi PRIVATE ${UACPI_INCLUDES} ${CMAKE_CURRENT_SOURCE_DIR})

target_include_directories(menix PRIVATE ${UACPI_INCLUDES})
target_link_libraries(menix PRIVATE uacpi)
