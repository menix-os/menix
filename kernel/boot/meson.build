kernel_sources += files('main.c')

if config.get('CONFIG_BOOT_LIMINE', 'y') == 'y'
    subdir('limine')
elif config.get('CONFIG_BOOT_UEFI', 'n') == 'y'
    subdir('uefi')
else
    error('Exactly one of the CONFIG_BOOT_* options must be enabled!')
endif
