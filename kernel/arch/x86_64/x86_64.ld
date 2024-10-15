/* Linker script for x86 builds */

OUTPUT_FORMAT(elf64-x86-64)

PHDRS
{
    boot		PT_LOAD;
	text		PT_LOAD;
	rodata		PT_LOAD;
	data		PT_LOAD;
}

SECTIONS
{
	. = 0xffffffff80000000;
	__ld_kernel_start = .;

	.boot ALIGN(4K):
	{
		__ld_sect_boot_start = .;
		INCLUDE boot.ld
		__ld_sect_boot_end = .;
	} :boot

	.text ALIGN(4K):
	{
	__ld_seg_text_start = .;
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
		*(.comment*)
		*(.eh_frame*)
		*(.gcc_except_table*)
		*(.note*)
		*(.rel.eh_frame*)
		*(.interp)
	}
}
