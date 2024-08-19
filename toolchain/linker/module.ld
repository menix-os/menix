INCLUDE common.ld

SECTIONS
{
	.text ALIGN(4K):
	{
		*(.init)
		*(.text)
		*(.text.*)
		*(.fini)
	} :text

	.rodata ALIGN(4K):
	{
		*(.rodata)
		*(.rodata.*)
	} :rodata

	.mod ALIGN(4K):
	{
		KEEP(*(.mod))
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

	/DISCARD/ :
	{
		INCLUDE discard.ld
	}

}
