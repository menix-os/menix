/* Linker script for kernel modules */

ENTRY(_start)

PHDRS
{
	null	PT_NULL;
	text	PT_LOAD		FLAGS(5);
	rodata	PT_LOAD		FLAGS(4);
	data	PT_LOAD		FLAGS(6);
	dynamic PT_DYNAMIC	FLAGS(6);
	modvers 0x60000001;
	modauth 0x60000002;
	moddesc 0x60000003;
}

SECTIONS
{
	.debug_gdb_scripts : {
		*(.debug_gdb_scripts)
	} :null

	/* Code */
	.text : {
		*(.init)
		*(.text)
		*(.text.*)
		*(.fini)
		*(.plt)
		*(.plt.*)
	} :text

	/* Read-only data */
	.rodata : {
		*(.rodata)
		*(.rodata.*)
		*(.got)
		*(.got.*)
	} :rodata

	.mod.vers : {
		KEEP(*(.mod.vers))
	} :rodata :modvers

	.mod.auth : {
		KEEP(*(.mod.auth))
	} :rodata :modauth

	.mod.desc : {
		KEEP(*(.mod.desc))
	} :rodata :moddesc

	/* Read-Write data */
	.data : {
		*(.data)
		*(.data.*)
	} :data

	/* Zero-initialized memory */
	.bss : {
		*(.*bss)
		*(.*bss.*)
		*(COMMON)
	} :data

	/* Dynamic tags */
	.dynamic : {
		*(.dynamic)
	} :data :dynamic

	.dynsym : {
		*(.dynsym)
	} :dynamic

	.dynstr : {
		*(.dynstr)
	} :dynamic

	.rela.dyn : {
		*(.rela.dyn)
	} :dynamic

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
