kernel_c_args = [
    '-ffreestanding',
    '-fno-stack-protector',
    '-fno-omit-frame-pointer',
    '-fno-pie',
    '-nostdinc',
    '-D__KERNEL__=1',
]

kernel_link_args = [
    '-nostdlib',
    '-Wl,-nostdlib',
    '-static',
    '-T' + meson.current_source_dir() / 'kernel.ld',
]

kernel_includes = [include_directories('arch' / target_machine.cpu_family())]

kernel_sources = []

subdir('arch' / target_machine.cpu_family())
subdir('boot')
subdir('klibc')
subdir('mem')
subdir('core')
subdir('syscalls')

configure_file(configuration: common_cfg, output: 'config.h')

executable(
    'menix',
    include_directories: [common_includes, kernel_includes],
    c_args: [common_c_args, kernel_c_args],
    link_args: [common_link_args, kernel_link_args],
    sources: kernel_sources,
    install: true,
    install_dir: get_option('datadir') / 'menix',
)
