build_example = get_option('example')
example_sources = files('main.c')

if build_example == 'y'
    kernel_sources += example_sources
elif build_example == 'm'
    modules += [['example', example_sources]]
endif
