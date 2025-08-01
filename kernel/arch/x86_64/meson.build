kernel_c_args += [
    '-mgeneral-regs-only',
    '-mno-red-zone',
    '-mcmodel=kernel',
    '-masm=intel',
]

kernel_sources += files(
    'gdt.c',
    'trampoline.S',
)
