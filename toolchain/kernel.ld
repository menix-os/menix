/* Linker script for kernel builds */

ENTRY(kernel_boot)

PHDRS
{
	headers	PT_LOAD		FILEHDR PHDRS;
	text	PT_LOAD		FLAGS(5);
	rodata	PT_LOAD		FLAGS(4);
	data	PT_LOAD		FLAGS(6);
	dynamic	PT_DYNAMIC	FLAGS(6);
}

SECTIONS
{
	. = SIZEOF_HEADERS;
	LD_KERNEL_START = .;

	/* Code */
	.text ALIGN(4K):
	{
	LD_TEXT_START = .;
		KEEP(*(.boot.init))
		KEEP(*(.boot))
		KEEP(*(.boot.fini))
		*(.init)
		*(.text)
		*(.text.*)
		*(.fini)
		*(.plt)
		*(.plt.*)
	LD_TEXT_END = .;
	} :text

	/* Read-only data */
	.rodata ALIGN(4K):
	{
	LD_RODATA_START = .;
		*(.rodata)
		*(.rodata.*)
		*(.got)
	LD_RODATA_END = .;
	} :rodata

	/* Read-Write data */
	.data ALIGN(4K):
	{
	LD_DATA_START = .;
		*(.data)
		*(.data.*)
	} :data

	/* Zero-initialized memory */
	.bss ALIGN(4K):
	{
		*(.*bss)
		*(.*bss.*)
		*(COMMON)
	} :data

	/* Dynamic tags */
	.dynamic :
	{
		*(.dynamic)
	LD_DATA_END = .;
	} :data :dynamic

	LD_KERNEL_END = .;

	/DISCARD/ :
	{
		*(.comment*)
		*(.eh_frame*)
		*(.rel.eh_frame*)
		*(.gcc_except_table*)
		*(.note*)
		*(.interp)
	}
}
