# Limine bootloader support.
# Limine is the default bootloader for menix.

add_module(boot_protocol_limine "marv7000" "Boot using the Limine protocol" MAIN OFF ON
	limine_entry.c
)
conflicts_option(boot_protocol_*)
add_linker_dir()
