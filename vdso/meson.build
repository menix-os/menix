vdso = shared_library(
    'menix-vdso',
    sources: ['main.c'],
    c_args: ['-ffreestanding', '-static', '-nostdinc'],
    link_args: ['-nostdlib'],
    include_directories: common_includes,
    name_prefix: '',
)
