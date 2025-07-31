/* Linker script for kernel modules */

PHDRS
{
    null    PT_NULL;
    text    PT_LOAD     FLAGS(5);
    rodata  PT_LOAD     FLAGS(4);
    data    PT_LOAD     FLAGS(6);
    dynamic PT_DYNAMIC  FLAGS(6);
    mod     0x60000001  FLAGS(4);
}

SECTIONS
{
    .debug_gdb_scripts : {
        *(.debug_gdb_scripts)
    } :null

    . = ALIGN(CONSTANT(MAXPAGESIZE));

    /* Code */
    .text : {
        *(.init)
        *(.text)
        *(.text.*)
        *(.fini)
        *(.plt)
        *(.plt.*)
    } :text

    . = ALIGN(CONSTANT(MAXPAGESIZE));

    /* Read-only data */
    .rodata : {
        *(.rodata)
        *(.rodata.*)
        *(.got)
        *(.got.*)
    } :rodata

    .mod : {
        KEEP(*(.mod))
    } :rodata :mod

    . = ALIGN(CONSTANT(MAXPAGESIZE));

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

    .dynsym : {
        *(.dynsym)
    } :data

    .dynstr : {
        *(.dynstr)
    } :data

    .rela.dyn : {
        *(.rela.dyn)
    } :data

    /* Dynamic tags */
    .dynamic : {
        *(.dynamic)
    } :data :dynamic

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
