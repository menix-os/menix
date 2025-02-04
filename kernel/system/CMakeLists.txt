# System hardware management

target_sources(menix PUBLIC
	elf.c
	interrupts.c
	terminal.c
)

add_subdirectory(sch)
add_subdirectory(time)
add_subdirectory(video)
