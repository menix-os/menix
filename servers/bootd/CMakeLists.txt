add_executable(bootd
	main.c
)

target_link_libraries(bootd PRIVATE common_server)
