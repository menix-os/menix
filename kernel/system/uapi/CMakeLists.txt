# uAPI implementation

target_sources(menix PUBLIC
	pci.c
	log.c
	malloc.c
)

target_include_directories(menix PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
