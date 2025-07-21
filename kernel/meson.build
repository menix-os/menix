kernel_c_args = [
    common_c_args,
    '-fno-PIC',
    '-fno-PIE',
    '-D__KERNEL__',
]

kernel_link_args = [
    common_link_args,
    '-static',
    '-T' + meson.current_source_dir() / 'kernel.ld',
]

kernel_includes = [
    common_includes,
    include_directories(
        'arch' / target_machine.cpu_family() / 'include',
    ),
]

kernel_sources = []

subdir('arch' / target_machine.cpu_family())
subdir('mm')
subdir('proc')
subdir('klibc')
subdir('sched')
subdir('sys')
subdir('util')
subdir('boot')
