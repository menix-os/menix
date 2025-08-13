executable(
    'posix',
    sources: [
        'main.c',
    ],
    c_args: [common_c_args, server_c_args],
    link_args: [common_link_args, server_link_args],
    include_directories: common_includes,
    install: true,
    install_dir: server_install_dir,
    link_with: libmenix,
)
