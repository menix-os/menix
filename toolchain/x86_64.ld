/* Linker script for x86 builds */

ENTRY(kernel_boot)

PHDRS
{
	headers	PT_LOAD		FILEHDR PHDRS;
	text	PT_LOAD		FLAGS(5);
	rodata	PT_LOAD		FLAGS(4);
	data	PT_LOAD		FLAGS(6);
}

SECTIONS
{
	. = 0xffffffff80000000 + SIZEOF_HEADERS;
	LD_KERNEL_START = .;

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
	LD_TEXT_END = .;
	} :text

	.rodata ALIGN(4K):
	{
	LD_RODATA_START = .;
		*(.rodata)
		*(.rodata.*)
	LD_RODATA_END = .;
	} :rodata

	.data ALIGN(4K):
	{
	LD_DATA_START = .;
		*(.data)
		*(.data.*)
	} :data

	.bss ALIGN(4K):
	{
		*(.*bss)
		*(.*bss.*)
		*(COMMON)
	LD_DATA_END = .;
	}

	LD_KERNEL_END = .;

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
