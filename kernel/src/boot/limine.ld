/* Linker script for kernel builds */

PHDRS
{
    text    PT_LOAD	    FLAGS(5);
    rodata  PT_LOAD	    FLAGS(4);
    data    PT_LOAD	    FLAGS(6);
    dynamic PT_DYNAMIC  FLAGS(6);
}

SECTIONS
{
    /* Default to -2G if KASLR is off */
    . = 0xffffffff80000000;
    LD_KERNEL_START = .;

    /* Code */
    .text ALIGN(CONSTANT(MAXPAGESIZE)) : {
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
    } :text

    .dynsym : {
        LD_DYNSYM_START = .;
        *(.dynsym)
        LD_DYNSYM_END = .;
    } :text

    .dynstr : {
        LD_DYNSTR_START = .;
        *(.dynstr)
        LD_DYNSTR_END = .;
    } :text

    LD_TEXT_END = .;

    /* Read-only data */
    .rodata ALIGN(CONSTANT(MAXPAGESIZE)) : {
    LD_RODATA_START = .;
        *(.rodata)
        *(.rodata.*)
        *(.got)
        *(.got.*)
    LD_RODATA_END = .;
    } :rodata

    /* Read-Write data */
    .data ALIGN(CONSTANT(MAXPAGESIZE)) : {
    LD_DATA_START = .;
        *(.data)
        *(.data.*)
    } :data

    /* Zero-initialized memory */
    .bss ALIGN(CONSTANT(MAXPAGESIZE)) : {
        *(.*bss)
        *(.*bss.*)
        *(COMMON)
    } :data

    /* Dynamic tags */
    .dynamic : {
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
