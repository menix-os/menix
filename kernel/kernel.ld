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
	__ld_kernel_start = .;

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

	.text : {
		__ld_text_start = .;
		*(.text .text.*)
		*(.plt .plt.*)
		__ld_text_end = .;
	} :text

	. = ALIGN(CONSTANT(MAXPAGESIZE));

	.rodata : {
		__ld_rodata_start = .;
		*(.rodata .rodata.*)
		*(.got .got.*)
		__ld_rodata_end = .;
	} :rodata

	. = ALIGN(CONSTANT(MAXPAGESIZE));

	__ld_data_start = .;
	.data : {
		*(.data .data.*)
	} :data

	.cmdline : {
		__ld_cmdline_start = .;
		KEEP(*(SORT_BY_NAME(.cmdline.*)))
		__ld_cmdline_end = .;
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
		__ld_percpu_start = .;
		*(.percpu.init)
		*(.percpu)
		. = ALIGN(CONSTANT(MAXPAGESIZE));
		__ld_percpu_end = .;
	} :percpu
	__ld_data_end = .;

	__ld_kernel_end = .;

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
