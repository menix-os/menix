build_example = cfg.get('CONFIG_EXAMPLE', 'n')
example_sources = files('main.c')

if build_example == 'y'
    kernel_sources += example_sources
elif build_example == 'm'
    modules += [['example', example_sources]]
endif
