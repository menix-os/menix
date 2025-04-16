/* Linker script for kernel modules */

ENTRY(_start)

PHDRS
{
    null        PT_NULL;
    text        PT_LOAD     FLAGS(5);
    rodata      PT_LOAD     FLAGS(4);
    data        PT_LOAD     FLAGS(6);
    dynamic     PT_DYNAMIC  FLAGS(6);
    modversion  0x60000001;
    modauthor   0x60000002;
    moddesc     0x60000003;
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

    .mod.version : {
        KEEP(*(.mod.version))
    } :rodata :modversion

    .mod.author : {
        KEEP(*(.mod.author))
    } :rodata :modauthor

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
