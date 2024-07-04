/*? Linker script for x86 builds */

ENTRY(kernel_boot);

FLAGS_READ	= 1 << 2;
FLAGS_WRITE	= 1 << 1;
FLAGS_EXEC	= 1 << 0;

PHDRS
{
	text	PT_LOAD		FLAGS(FLAGS_READ | FLAGS_EXEC);
	rodata	PT_LOAD		FLAGS(FLAGS_READ);
	data	PT_LOAD		FLAGS(FLAGS_READ | FLAGS_WRITE);
	dynamic PT_DYNAMIC	FLAGS(FLAGS_READ | FLAGS_WRITE);
}

SECTIONS
{
	. = 0xffffffff80000000;
	__ld_kernel_start = .;

	/* General kernel code */
	.text ALIGN(4K):
	{
  		KEEP(*(.multiboot))
  		KEEP(*(.entry))
		*(.text)
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

	/* Built-in module data goes here. */
	.mod ALIGN(4K):
	{
		__ld_sect_mod_start = .;
		KEEP(*(.mod*));
		__ld_sect_mod_end = .;
	} :data

	.dynamic ALIGN(4K):
	{
        *(.dynamic)
    } :data :dynamic

	.bss ALIGN(4K):
	{
		*(COMMON)
		*(.bss)
	} :data

	__ld_kernel_end = .;

	INCLUDE toolchain/common.ld
}
