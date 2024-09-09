#include <menix/common.h>

#ifndef CONFIG_arch_x86
#error This driver is only compatible with x86!
#endif

#include <menix/log.h>
#include <menix/module.h>

#include <io.h>

#define KB_KEY_DOWN_LSHIFT	 0x2A
#define KB_KEY_UP_LSHIFT	 0xAA
#define KB_KEY_DOWN_CAPSLOCK 0x3A

unsigned char keyboard_map[128] = {
	'\0', '\e', '1',  '2', '3',	 '4',  '5',	 '6',  '7',	 '8',  '9',	 '0',  '-',	 '=',  '\b', '\t', 'q',	 'w',
	'e',  'r',	't',  'y', 'u',	 'i',  'o',	 'p',  '[',	 ']',  '\n', '\0', 'a',	 's',  'd',	 'f',  'g',	 'h',
	'j',  'k',	'l',  ';', '\'', '`',  '\0', '\\', 'z',	 'x',  'c',	 'v',  'b',	 'n',  'm',	 ',',  '.',	 '/',
	'\0', '*',	'\0', ' ', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '7',
	'8',  '9',	'-',  '4', '5',	 '6',  '+',	 '1',  '2',	 '3',  '0',	 '.',  '\0', '\0', '\0', '\0', '\0'};

unsigned char keyboard_shift_map[128] = {
	'\0', '\e', '!',  '@', '#',	 '$',  '%',	 '^',  '&',	 '*',  '(',	 ')',  '_',	 '+',  '\b', '\t', 'Q',	 'W',
	'E',  'R',	'T',  'Y', 'U',	 'I',  'O',	 'P',  '{',	 '}',  '\n', '\0', 'A',	 'S',  'D',	 'F',  'G',	 'H',
	'J',  'K',	'L',  ':', '"',	 '`',  '\0', '|',  'Z',	 'X',  'C',	 'V',  'B',	 'N',  'M',	 '<',  '>',	 '?',
	'\0', '*',	'\0', ' ', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '7',
	'8',  '9',	'-',  '4', '5',	 '6',  '+',	 '1',  '2',	 '3',  '0',	 '.',  '\0', '\0', '\0', '\0', '\0'};

static u8 ps2_read()
{
	while (!!arch_x86_read8(0x64) == 0)
		asm_pause();
	return arch_x86_read8(0x60);
}

MODULE_FN i32 init_fn()
{
	// TODO
	module_log("Registered PS/2 keyboard input.\n");
	return 0;
}

MODULE_FN void exit_fn()
{
}

MODULE = {
	.name = MODULE_NAME,
	.init = init_fn,
	.exit = exit_fn,
	MODULE_META,
};
