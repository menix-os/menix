# ? Self contained EFI bootloader.

add_module(boot_efi "marv7000" "Use EFI for booting" MAIN FALSE OFF
	main.c
)
conflicts_option(boot_limine)

include_directories(BEFORE /usr/include/efi/)

add_compile_options(-fshort-wchar)
add_compile_options(-mno-red-zone)

add_link_options(-Wl,-entry:efi_main)
add_link_options(-Wl,-subsystem:efi_application)
add_link_options(-fuse-ld=lld-link)
