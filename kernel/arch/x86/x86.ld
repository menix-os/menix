/* Linker script for x86 builds */

INCLUDE common.ld

SECTIONS
{
	. = 0xffffffff80000000 + SIZEOF_HEADERS;
	__ld_kernel_start = .;

	.boot ALIGN(4K):
	{
	__ld_seg_text_start = .;

		__ld_sect_boot_start = .;
		INCLUDE boot.ld
		__ld_sect_boot_end = .;
	} :text

	.text ALIGN(4K):
	{
		*(.*init)
		__ld_sect_text_start = .;
		*(.*text)
		*(.*text.*)
		__ld_sect_text_end = .;
		*(.*fini)
	__ld_seg_text_end = .;
	} :text

	.rodata ALIGN(4K):
	{
	__ld_seg_rodata_start = .;
		__ld_sect_rodata_start = .;
		*(.*rodata)
		*(.*rodata.*)
		__ld_sect_rodata_end = .;
	__ld_seg_rodata_end = .;
	} :rodata


	.data ALIGN(4K):
	{
	__ld_seg_data_start = .;
		__ld_sect_data_start = .;
		*(.*data)
		*(.*data.*)
		__ld_sect_data_end = .;
	} :data

	.mod :
	{
		__ld_sect_mod_start = .;
		KEEP(*(.mod))
		__ld_sect_mod_end = .;
	} :data

	.bss ALIGN(4K):
	{
		*(.*bss)
		*(.*bss.*)
		*(COMMON)
	__ld_seg_data_end = .;
	}

	__ld_kernel_end = .;

	/DISCARD/ :
	{
		INCLUDE discard.ld
	}
}
