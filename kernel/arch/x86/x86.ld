/* Linker script for x86 builds */


INCLUDE common.ld

SECTIONS
{
	. = 0xffffffff80000000 + SIZEOF_HEADERS;
	__ld_kernel_start = .;

	__ld_seg_text_start = .;
	.boot ALIGN(4K):
	{
		INCLUDE boot.ld
	} :text

	.text ALIGN(4K):
	{
		*(.init)
		*(.text)
		*(.text.*)
		*(.fini)
	} :text
	__ld_seg_text_end = .;

	__ld_seg_rodata_start = .;
	.rodata ALIGN(4K):
	{
		*(.rodata)
		*(.rodata.*)
	} :rodata
	__ld_seg_rodata_end = .;

	__ld_seg_data_start = .;
	.mod ALIGN(4K):
	{
		__ld_sect_mod_start = .;
		KEEP(*(.mod))
		__ld_sect_mod_end = .;
	} :data

	.data ALIGN(4K):
	{
		*(.data)
		*(.data.*)
	} :data

	.bss ALIGN(4K):
	{
		*(.bss)
		*(.bss.*)
		*(COMMON)
	}
	__ld_seg_data_end = .;

	__ld_kernel_end = .;

	/DISCARD/ :
	{
		INCLUDE discard.ld
	}
}
