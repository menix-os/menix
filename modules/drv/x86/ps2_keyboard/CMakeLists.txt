add_module(ps2_keyboard "Marvin Friedrich" "PS/2 Keyboard driver" MAIN ON MOD
	main.c
)
require_option(arch_x86)
