# tmpfs file system

add_module(fs_tmpfs "Marvin Friedrich" "tmpfs file system" MAIN OFF ON
	main.c
)

# tmpfs is never optional, it's just built as a module.
require_option(fs_tmpfs)
