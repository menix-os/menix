# ? Self contained EFI bootloader.

add_module(boot_efi "marv7000" "Boot using the EFI loader" MAIN FALSE OFF
	efi_entry.c
)
conflicts_option(boot_*)

include_directories(BEFORE /usr/include/efi/)

add_compile_options(-fshort-wchar)
add_compile_options(-mno-red-zone)

add_link_options(-Wl,-subsystem:efi_application)
add_link_options(-fuse-ld=lld-link)
