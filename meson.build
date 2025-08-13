project(
    'menix',
    'c',
    license: 'BSD-3',
    license_files: ['LICENSE'],
    default_options: [
        'c_std=gnu23',
    ],
    version: '0.0.1',
    meson_version: '>=1.1.0',
)

# Install headers
if get_option('headers_only') == true
    install_subdir('include' / 'menix', install_dir: get_option('includedir'))
    subdir_done()
endif

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
subdir('lib')
subdir('servers')
