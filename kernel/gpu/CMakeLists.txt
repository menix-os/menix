# Graphics Drivers

include(${MENIX_UTIL_PATH})
add_library(menix_gpu STATIC
	fb.c
)
add_option(gpu ON)
