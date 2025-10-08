kernel_sources += files(
    'cmdline.c',
    'main.c',
)

if get_option('boot_protocol') == 'limine'
    subdir('limine')
elif get_option('boot_protocol') == 'uefi'
    subdir('uefi')
else
    error('Need one of limine or uefi')
endif
