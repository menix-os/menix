ENTRY(kernel_start)

PHDRS
{
    idata	PT_LOAD		FLAGS(6);
    itext	PT_LOAD		FLAGS(5);
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

    .init.data : {
        KEEP(*(SORT_BY_NAME(.init.data.*)))
        KEEP(*(.init.data))
    } :idata

    . = ALIGN(CONSTANT(MAXPAGESIZE));

    .init.text : {
        KEEP(*(.init.text))
    } :itext

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

    /* Command line options */
    .cmdline : {
        __ld_cmdline_start = .;
        KEEP(*(.cmdline))
        __ld_cmdline_end = .;
    } :rodata

    /* Early consoles */
    .earlycon : {
        __ld_earlycon_start = .;
        KEEP(*(.earlycon))
        __ld_earlycon_end = .;
    } :rodata

    . = ALIGN(CONSTANT(MAXPAGESIZE));

    __ld_data_start = .;
    .data : {
        *(.data .data.*)
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

    .bss : {
        *(.*bss .*bss.*)
        *(COMMON)
    } :data

    /* Per-CPU data block. MUST always come last. */
    .percpu ALIGN(CONSTANT(MAXPAGESIZE)) : {
        __ld_percpu_start = .;
        KEEP(*(.percpu))
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
