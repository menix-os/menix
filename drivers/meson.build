modules = []

subdir('example')

foreach m : modules
    shared_library(
        m[0],
        sources: m[1],
        c_args: [common_c_args, '-D__MODULE__'],
        include_directories: common_includes,
        link_args: [common_link_args, '-T' + meson.current_source_dir() / 'module.ld'],
        name_prefix: '',
        name_suffix: 'kso',
        install: true,
        install_dir: get_option('datadir') / 'menix' / 'modules',
    )
endforeach
