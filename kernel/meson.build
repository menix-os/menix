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

subdir('arch' / target_machine.cpu_family())
subdir('boot')
subdir('mem')
subdir('sched')
subdir('sys')
subdir('util')

executable(
    'menix',
    include_directories: [common_includes, kernel_includes],
    c_args: [common_c_args, kernel_c_args],
    link_args: [common_link_args, kernel_link_args],
    sources: kernel_sources,
    install: true,
    install_dir: get_option('datadir') / 'menix',
)
