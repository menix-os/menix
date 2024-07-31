# (Multi-)Threading and processes

include(${MENIX_UTIL_PATH})
add_library(menix_thread
	spin.c
	process.c
)
