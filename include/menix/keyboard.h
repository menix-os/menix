//? Basic PS/2 Keyboard IO

#pragma once

/// \brief	Initializes the PS/2 keyboard.
void kb_init();

#define KB_KEY_DOWN_LSHIFT	 0x2A
#define KB_KEY_UP_LSHIFT	 0xAA
#define KB_KEY_DOWN_CAPSLOCK 0x3A

unsigned char keyboard_map[128] = {
	// Nothing, Escape
	'\0', '\0',
	// Numbers
	'1', '2', '3', '4', '5', '6', '7', '8', '9', '0',
	// Symbols
	'-', '=', '\b', '\t',
	// R1
	'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', '[', ']', '\n',
	// R2
	'\0', 'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';', '\'', '`',
	// R3
	'\0', '\\', 'z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/', '\0', '*',
	// LAlt, Space, Caps
	'\0', ' ', '\0',
	// Fn
	'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
	// NumLock, ScrollLock
	'\0', '\0',
	// Numpad
	'7', '8', '9', '-', '4', '5', '6', '+', '1', '2', '3', '0', '.',
	// Pad
	'\0', '\0', '\0',
	// F11, F12
	'\0', '\0'
};

unsigned char keyboard_shift_map[128] = {
	// Nothing, Escape
	'\0', '\0',
	// Numbers
	'!', '@', '#', '$', '%', '^', '&', '*', '(', ')',
	// Symbols
	'_', '+', '\b', '\t',
	// R1
	'Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P', '{', '}', '\n',
	// R2
	'\0', 'A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L', ':', '"', '`',
	// R3
	'\0', '|', 'Z', 'X', 'C', 'V', 'B', 'N', 'M', '<', '>', '?', '\0', '*',
	// LAlt, Space, Caps
	'\0', ' ', '\0',
	// Fn
	'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
	// NumLock, ScrollLock
	'\0', '\0',
	// Numpad
	'7', '8', '9', '-', '4', '5', '6', '+', '1', '2', '3', '0', '.',
	// Pad
	'\0', '\0', '\0',
	// F11, F12
	'\0', '\0'
};
