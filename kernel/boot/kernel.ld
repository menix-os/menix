PHDRS
{
	itext	PT_LOAD		FLAGS(5);
	idata	PT_LOAD		FLAGS(6);
	text	PT_LOAD		FLAGS(5);
	rodata	PT_LOAD		FLAGS(4);
	data	PT_LOAD		FLAGS(6);
	dynamic	PT_DYNAMIC	FLAGS(6);
	percpu	PT_LOAD		FLAGS(6);
}

SECTIONS
{
	. = 0xffffffff80000000;
	_LD_KERNEL_START = .;

	. = ALIGN(CONSTANT(MAXPAGESIZE));

	.init.text : {
		KEEP(*(.init.text))
	} :itext

	. = ALIGN(CONSTANT(MAXPAGESIZE));

	.init.data : {
		KEEP(*(SORT_BY_NAME(.init.data.*)))
		KEEP(*(.init.data))
	} :idata

	. = ALIGN(CONSTANT(MAXPAGESIZE));

	_LD_TEXT_START = .;
	.text : {
		*(.text .text.*)
		*(.plt .plt.*)
	} :text
	_LD_TEXT_END = .;

	. = ALIGN(CONSTANT(MAXPAGESIZE));

	_LD_RODATA_START = .;
	.rodata : {
		*(.rodata .rodata.*)
		*(.got .got.*)
	} :rodata
	_LD_RODATA_END = .;

	. = ALIGN(CONSTANT(MAXPAGESIZE));

	_LD_DATA_START = .;
	.data : {
		*(.data .data.*)
	} :data

	.dynamic : {
		*(.dynamic)
	} :data :dynamic

	.bss ALIGN(CONSTANT(MAXPAGESIZE)) : {
		*(.*bss .*bss.*)
		*(COMMON)
	} :data

	/* Per-CPU data block. MUST always come last. */
	.percpu ALIGN(CONSTANT(MAXPAGESIZE)) : {
		_LD_PERCPU_START = .;
		*(.percpu.init)
		*(.percpu)
		. = ALIGN(CONSTANT(MAXPAGESIZE));
		_LD_PERCPU_END = .;
	} :percpu
	_LD_DATA_END = .;

	_LD_KERNEL_END = .;

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
