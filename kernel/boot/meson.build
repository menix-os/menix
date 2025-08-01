kernel_sources += files(
    'cmdline.c',
    'main.c',
)

boot_protocol = cfg.get('CONFIG_BOOT_PROTOCOL', 'limine')

if boot_protocol == 'limine'
    subdir('limine')
elif boot_protocol == 'uefi'
    subdir('uefi')
endif
