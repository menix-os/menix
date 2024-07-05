/*? Linker script for x86 builds */

. = 0xffffffff80000000 + SIZEOF_HEADERS;

INCLUDE common.ld

SECTIONS
{
	__ld_kernel_start = .;

	.text ALIGN(4K):
	{
		INCLUDE boot.ld
		*(.init)
		*(.text)
		*(.text.*)
		*(.fini)
	} :text

	.data ALIGN(4K):
	{
		*(.data)
	} :data

	.rodata ALIGN(4K):
	{
		*(.rodata)
		*(.rodata.*)
	} :rodata

	.mod ALIGN(4K):
	{
		__ld_sect_mod_start = .;
		KEEP(*(.mod))
		__ld_sect_mod_end = .;
	}

	.dynamic ALIGN(4K):
	{
		*(.dynamic)
	}

	.bss ALIGN(4K):
	{
		*(COMMON)
		*(.bss)
	}

	__ld_kernel_end = .;

	/DISCARD/ :
	{
		INCLUDE discard.ld
	}
}
