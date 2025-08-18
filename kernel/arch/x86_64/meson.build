common_c_args += [
    '-masm=intel',
]

kernel_c_args += [
    '-mgeneral-regs-only',
    '-mno-red-zone',
    '-mcmodel=kernel',
]

kernel_sources += files(
    'gdt.c',
    'serial.c',
    'sys.c',
    'trampoline.S',
)
