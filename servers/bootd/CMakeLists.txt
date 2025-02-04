add_executable(testd
	main.c
)

target_link_libraries(testd PRIVATE common_server)
