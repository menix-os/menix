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
    '-funsigned-char',
    '-Wall',
    '-Werror',
    '-Wno-unused',
]
common_link_args = [
    '-znoexecstack',
    '-Wl,--gc-sections',
    '-Wl,--build-id=none',
]

common_includes = include_directories('include')
common_cfg = configuration_data()

common_cfg.set_quoted('MENIX_VERSION', meson.project_version())
c_compiler = meson.get_compiler('c')
common_cfg.set_quoted('MENIX_COMPILER_ID', c_compiler.get_id() + ' ' + c_compiler.version())
common_cfg.set_quoted('MENIX_LINKER_ID', c_compiler.get_linker_id())

subdir('kernel')
subdir('servers')
