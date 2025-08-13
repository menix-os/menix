server_c_args = []

server_link_args = [
    '-T' + meson.current_source_dir() / 'server.ld',
]

server_install_dir = get_option('datadir') / 'menix' / 'servers'

subdir('posix')
