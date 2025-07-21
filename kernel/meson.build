kernel_sources = []

kernel_c_args = [
    '-ffreestanding',
    '-fno-stack-protector',
    '-fno-omit-frame-pointer',
    '-fno-PIC',
    '-fno-PIE',
    '-funsigned-char',
    '-nostdinc',
    '-D__KERNEL__',
    '-Wall',
    '-Werror',
]

kernel_link_args = [
    '-static',
    '-nostdlib',
    '-znoexecstack',
    '-Wl,-nostdlib',
    '-Wl,--gc-sections',
    '-Wl,--build-id=none',
]

subdir('arch' / target_machine.cpu_family())
subdir('mem')
subdir('proc')
subdir('klibc')
subdir('sched')
subdir('system')
subdir('boot')

executable(
    'menix',
    include_directories: [
        common_includes,
        'arch' / target_machine.cpu_family() / 'include',
    ],
    sources: kernel_sources,
    c_args: kernel_c_args,
    link_args: kernel_link_args,
    install: true,
    pie: false,
)
