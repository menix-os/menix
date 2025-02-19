# Limine bootloader support.
# Limine is the default bootloader for menix.

target_sources(menix PUBLIC
	limine_entry.c
)

target_link_options(menix PUBLIC -T ${CMAKE_CURRENT_SOURCE_DIR}/limine.ld)
