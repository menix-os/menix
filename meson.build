project(
    'menix',
    'c',
    license: 'BSD-3',
    license_files: ['LICENSE'],
    default_options: [
        'c_std=gnu23',
    ],
    version: '0.1.0',
    meson_version: '>=1.1.0',
)

# Install headers
if get_option('headers_only') == true
    install_subdir('uapi', install_dir: get_option('includedir'))
    subdir_done()
endif

keyval = import('keyval')
cfg = keyval.load(get_option('config'))

# Generate a header from the config file.
output_config = {}
foreach c, v : cfg
    if v == 'y'
        output_config += {c: 1}
    else
        output_config += {c: v}
    endif
endforeach
configure_file(output: 'config.h', configuration: output_config)

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

modules = []

subdir('kernel')
subdir('drivers')

kernel = shared_library(
    'menix',
    include_directories: [common_includes, kernel_includes],
    c_args: [common_c_args, kernel_c_args],
    link_args: [common_link_args, kernel_link_args],
    sources: kernel_sources,
    install: true,
    install_dir: get_option('datadir') / 'menix',
    name_prefix: '',
    name_suffix: 'elf',
)

foreach m : modules
    shared_library(
        m[0],
        sources: m[1],
        c_args: [common_c_args, '-D__MODULE__="' + m[0] + '"'],
        include_directories: common_includes,
        link_args: [
            common_link_args,
            '-T' + meson.current_source_dir() / 'drivers' / 'module.ld',
        ],
        name_prefix: '',
        name_suffix: 'kso',
        install: true,
        install_dir: get_option('datadir') / 'menix' / 'modules',
        link_with: [kernel],
    )
endforeach

subdir('vdso')
# TODO: Copy VDSO into the kernel.
# objcopy = find_program('objcopy')
# strip = find_program('strip')
