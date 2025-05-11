/* Linker script for kernel builds */

PHDRS
{
    text    PT_LOAD	    FLAGS(5);
    rodata  PT_LOAD	    FLAGS(4);
    data    PT_LOAD	    FLAGS(6);
    dynamic PT_DYNAMIC  FLAGS(6);
    percpu  PT_LOAD     FLAGS(6);
}

SECTIONS
{
    /* Default to -2G if KASLR is off */
    . = 0xffffffff80000000;
    LD_KERNEL_START = .;

    . = ALIGN(CONSTANT(MAXPAGESIZE));

    /* Code */
    .text : {
        LD_TEXT_START = .;
        KEEP(*(.boot.init))
        KEEP(*(.boot))
        KEEP(*(.boot.fini))
        *(.init)
        *(.text .text.*)
        *(.fini)
        *(.plt .plt.*)
    } :text

    LD_TEXT_END = .;

    . = ALIGN(CONSTANT(MAXPAGESIZE));

    /* Read-only data */
    .rodata : {
        LD_RODATA_START = .;
        *(.rodata .rodata.*)
        *(.got .got.*)
        LD_RODATA_END = .;
    } :rodata

    . = ALIGN(CONSTANT(MAXPAGESIZE));

    /* Read-Write data */
    .data : {
        LD_DATA_START = .;

        LD_EARLY_ARRAY_START = .;
        *(.early_array)
        LD_EARLY_ARRAY_END = .;

        LD_INIT_ARRAY_START = .;
        *(.init_array)
        LD_INIT_ARRAY_END = .;

        *(.data .data.*)
    } :data

    /* Dynamic symbols */
    .dynsym : {
        LD_DYNSYM_START = .;
        *(.dynsym)
        LD_DYNSYM_END = .;
    } :data

    /* Dynamic symbol names */
    .dynstr : {
        LD_DYNSTR_START = .;
        *(.dynstr)
        LD_DYNSTR_END = .;
    } :data

    /* Dynamic tags */
    .dynamic : {
        *(.dynamic)
    } :data :dynamic

    /* Zero-initialized memory */
    .bss ALIGN(CONSTANT(MAXPAGESIZE)) : {
        *(.*bss .*bss.*)
        *(COMMON)
    } :data

    /* Per-CPU data block. MUST always come last. */
    .percpu ALIGN(CONSTANT(MAXPAGESIZE)) : {
        LD_PERCPU_START = .;
        *(.percpu.init)
        *(.percpu)
        . = ALIGN(CONSTANT(MAXPAGESIZE));
        LD_PERCPU_END = .;
        LD_DATA_END = .;
    } :percpu

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
