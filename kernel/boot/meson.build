kernel_sources += files(
    'cmdline.c',
    'main.c',
)

if cfg.get('CONFIG_BOOT_LIMINE', 'n') == 'y'
    subdir('limine')
elif cfg.get('CONFIG_BOOT_UEFI', 'n') == 'y'
    subdir('uefi')
else
    error('Need one of BOOT_LIMINE or BOOT_UEFI')
endif
