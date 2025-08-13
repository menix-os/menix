common_c_args += [
    '-mgeneral-regs-only',
    '-mno-red-zone',
    '-masm=intel',
]

kernel_c_args += [
    '-mcmodel=kernel',
]

kernel_sources += files(
    'gdt.c',
    'serial.c',
    'trampoline.S',
)
