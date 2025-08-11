kernel_c_args = [
    '-D__KERNEL__=1',
]

kernel_link_args = [
    '-static',
    '-T' + meson.current_source_dir() / 'kernel.ld',
]

kernel_includes = [
    include_directories(
        'arch' / target_machine.cpu_family() / 'include',
    ),
]

kernel_sources = []

subdir('klibc')
subdir('arch' / target_machine.cpu_family())
subdir('boot')
subdir('mem')
subdir('proc')
subdir('sys')
subdir('util')
