project(
    'menix',
    'c',
    license: 'BSD-3',
    license_files: ['LICENSE'],
    version: '0.0.1',
    meson_version: '>=1.1.0',
)

common_c_args = [
    '-Wall',
    '-Werror',
    '-Wno-unused',
    '-std=gnu23',
]

common_link_args = [
    '-znoexecstack',
    '-Wl,--gc-sections',
    '-Wl,--build-id=none',
]

common_includes = include_directories('include')

# Config file
common_cfg = configuration_data()
common_cfg.set_quoted('MENIX_VERSION', meson.project_version())
common_cfg.set_quoted('MENIX_ARCH', target_machine.cpu_family())
c_compiler = meson.get_compiler('c')
common_cfg.set_quoted('MENIX_COMPILER_ID', c_compiler.get_id() + ' ' + c_compiler.version())
common_cfg.set_quoted('MENIX_LINKER_ID', c_compiler.get_linker_id())


if get_option('build_kernel')
    subdir('kernel')
    subdir('drivers')
    subdir('vdso')
endif

if get_option('build_utils')
    subdir('utils')
endif
