kernel_sources += files('main.c')

if get_option('boot_protocol') == 'limine'
    subdir('limine')
elif get_option('boot_protocol') == 'uefi'
    subdir('uefi')
endif
