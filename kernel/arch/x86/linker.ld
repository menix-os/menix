/*--------------------------
Linker script for x86 builds
--------------------------*/

ENTRY(_start);

FLAGS_READ	= 1 << 2;
FLAGS_WRITE = 1 << 1;
FLAGS_EXEC	= 1 << 0;

PHDRS
{
	text	PT_LOAD FLAGS(FLAGS_READ | FLAGS_EXEC);
	data	PT_LOAD FLAGS(FLAGS_READ | FLAGS_WRITE);
	rodata	PT_LOAD FLAGS(FLAGS_READ);
}

SECTIONS
{
	. = 1M;
	_kernel_start = .;

	.text ALIGN(4K):
	{
  		KEEP(*(.multiboot))
  		KEEP(*(.entry))
		*(.text)
	} :text

	.rodata ALIGN(4K):
	{
		*(.rodata)
	} :rodata

	.data ALIGN(4K):
	{
		*(.data)
	} :data

	.bss ALIGN(4K):
	{
		*(COMMON)
		*(.bss)
		*(.bootstrap_stack)
	}
	_kernel_end = .;

	/* Remove all unwanted notes as they can break code layout. */
	/DISCARD/ :
	{
		*(.note*)
	}
}
