project(
    'menix',
    'c',
    license: 'BSD-3',
    license_files: ['LICENSE'],
    default_options: ['c_std=gnu23'],
    version: '0.1.0',
    meson_version: '>=1.1.0',
)

# Install headers
if get_option('headers_only') == true
    install_subdir('uapi', install_dir: get_option('includedir'))
    subdir_done()
endif

objcopy = find_program('objcopy')
strip = find_program('strip')

common_c_args = [
    '-ffreestanding',
    '-fno-stack-protector',
    '-fno-omit-frame-pointer',
    '-funsigned-char',
    '-nostdinc',
    '-Wall',
    '-Werror',
    '-Wno-unused',
]
common_link_args = [
    '-nostdlib',
    '-znoexecstack',
    '-Wl,-nostdlib',
    '-Wl,--gc-sections',
    '-Wl,--build-id=none',
]
common_includes = include_directories('include')

subdir('kernel')
subdir('drivers')
subdir('vdso')

kernel = executable(
    'menix',
    include_directories: kernel_includes,
    sources: kernel_sources,
    c_args: kernel_c_args,
    link_args: kernel_link_args,
    install: true,
    install_dir: get_option('datadir') / 'menix',
    pie: false,
)
