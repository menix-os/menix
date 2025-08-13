/* Linker script for servers */

ENTRY(_start)

PHDRS
{
    null    PT_NULL;
    text    PT_LOAD     FLAGS(5);
    rodata  PT_LOAD     FLAGS(4);
    data    PT_LOAD     FLAGS(6);
    dynamic PT_DYNAMIC  FLAGS(6);
    menix   0x60000001  FLAGS(4); /* Menix-specific data */
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

    .menix : {
        KEEP(*(.menix.*))
        KEEP(*(.menix))
    } :rodata :menix

    . = ALIGN(CONSTANT(MAXPAGESIZE));

    /* Read-Write data */
    .data : {
        *(.data)
        *(.data.*)
    } :data

    .dynsym : {
        __ld_dynsym_start = .;
        *(.dynsym)
        __ld_dynsym_end = .;
    } :data

    .dynstr : {
        __ld_dynstr_start = .;
        *(.dynstr)
        __ld_dynstr_end = .;
    } :data

    .dynamic : {
        *(.dynamic)
    } :data :dynamic

    /* Zero-initialized memory */
    .bss : {
        *(.*bss)
        *(.*bss.*)
        *(COMMON)
    } :data

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
