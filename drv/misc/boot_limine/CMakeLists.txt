# ? Limine bootloader support.
# ? Limine is the default bootloader for menix.

add_module(boot_limine "marv7000" "Boot using the Limine protocol" MAIN FALSE ON
	limine_entry.c
)
conflicts_option(boot_*)

add_linker_dir()