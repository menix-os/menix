project(
    'menix',
    'c',
    license: 'BSD-3',
    license_files: ['LICENSE'],
    default_options: ['c_std=gnu23'],
    version: '0.1.0',
)

keyval = import('keyval')
config_path = get_option('config')
config = keyval.load(config_path)

common_includes = include_directories('include')
subdir('vdso')
subdir('kernel')
subdir('drivers')

configure_file(configuration: config, output: 'config.h')
