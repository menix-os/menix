kernel_sources += files('init.c')

if cfg.get('CONFIG_SLAB', 'n') == 'y'
    kernel_sources += files('slab.c')
else
    error('No kmalloc allocator was enabled!')
endif
