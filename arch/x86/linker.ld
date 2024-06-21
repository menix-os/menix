/*? Linker script for x86 builds */

ENTRY(_start);

FLAGS_READ	= 1 << 2;
FLAGS_WRITE	= 1 << 1;
FLAGS_EXEC	= 1 << 0;

KERNEL_OFFSET = 1M;

PHDRS
{
	text	PT_LOAD FLAGS(FLAGS_READ | FLAGS_EXEC);
	data	PT_LOAD FLAGS(FLAGS_READ | FLAGS_WRITE);
	rodata	PT_LOAD FLAGS(FLAGS_READ);
}

SECTIONS
{
	. = KERNEL_OFFSET;
	__ld_kernel_start = .;

	/* General kernel code */
	.text ALIGN(4K):
	{
  		KEEP(*(.multiboot))
  		KEEP(*(.entry))
		*(.text)
	} :text

	/* Built-in module data goes here. */
	.mod ALIGN(4K):
	{
		__ld_sect_mod_start = .;
		KEEP(*(.mod*));
		__ld_sect_mod_end = .;
	} :data

	.data ALIGN(4K):
	{
		*(.data)
	} :data

	.rodata ALIGN(4K):
	{
		*(.rodata)
	} :rodata

	.bss ALIGN(4K):
	{
		*(COMMON)
		*(.bss)
		*(.bootstrap_stack)
	}

	__ld_kernel_end = .;

	/* Remove all unwanted sections as they can break code layout. */
	/DISCARD/ :
	{
		*(.comment*)
    	*(.eh_frame*)
    	*(.gcc_except_table*)
    	*(.note*)
    	*(.rel.eh_frame*)
	}
}
