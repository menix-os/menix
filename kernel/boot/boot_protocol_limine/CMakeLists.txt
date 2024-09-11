# Limine bootloader support.
# Limine is the default bootloader for menix.

target_sources(menix PUBLIC
	limine_entry.c
)
add_linker_dir()
