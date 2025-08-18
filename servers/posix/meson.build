executable(
    'posix',
    sources: [
        'main.c',
    ],
    c_args: [common_c_args],
    # It can't have a dynamic loader since it's the server providing the VFS.
    link_args: [common_link_args, '-static'],
    include_directories: common_includes,
    install: true,
    install_dir: server_install_dir,
)
