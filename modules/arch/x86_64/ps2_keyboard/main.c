#include <menix/common.h>
#include <menix/fs/devtmpfs.h>
#include <menix/fs/fd.h>
#include <menix/fs/handle.h>
#include <menix/io/terminal.h>

#include <interrupts.h>

#ifndef CONFIG_arch_x86_64
#error This driver is only compatible with x86!
#endif

#include <menix/system/module.h>
#include <menix/util/log.h>

#include <io.h>
#include <pic.h>

#define KB_KEY_DOWN_LSHIFT	 0x2A
#define KB_KEY_UP_LSHIFT	 0xAA
#define KB_KEY_DOWN_CAPSLOCK 0x3A

#define KEYBOARD_DATA_PORT	 0x60
#define KEYBOARD_STATUS_PORT 0x64

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

static u8 shift = 1;

static u8 ps2_read()
{
	u8 status;
	do
	{
		status = arch_x86_read8(KEYBOARD_STATUS_PORT);
	} while ((status & 0x01) == 0);
	return arch_x86_read8(KEYBOARD_DATA_PORT);
}

static char read_keycode()
{
	u8 keycode = ps2_read();

	// Determine shift.
	if (keycode == KB_KEY_DOWN_LSHIFT || keycode == KB_KEY_UP_LSHIFT || keycode == KB_KEY_DOWN_CAPSLOCK)
	{
		shift = !shift;
		return -1;
	}

	// Only get press events.
	if (keycode > 128)
		return -1;

	char ch = shift ? keyboard_map[keycode] : keyboard_shift_map[keycode];

	Handle* h = terminal_get_active_node()->handle;
	h->write(h, NULL, &ch, 1, 0);

	return ch;
}

static isize ps2_keyboard_read(Handle* handle, FileDescriptor* fd, void* data, usize size, off_t offset)
{
	char ch;
	do
	{
		ch = read_keycode();
		asm_pause();
	} while (ch == -1);
	((char*)data)[0] = ch;

	return 1;
}

MODULE_FN i32 nvme_init()
{
	// Add this keyboard as a new input method.

	arch_x86_write8(KEYBOARD_STATUS_PORT, 0xFF);	// Reset PS/2 controller.
	arch_x86_write8(KEYBOARD_STATUS_PORT, 0xAE);	// Enable PS/2 keyboard.

	arch_x86_write8(KEYBOARD_DATA_PORT, 0xFF);	  // Reset the keyboard.

	if (ps2_read() != 0xFA)
	{
		module_log("Failed to register PS/2 keyboard because it didn't send an ACK response!\n");
		return 1;
	}

	arch_x86_write8(KEYBOARD_DATA_PORT, 0xF0);	  // Send "Set Scan Code Set" command.
	arch_x86_write8(KEYBOARD_DATA_PORT, 0x02);	  // Set scan code set to 2.

	// Register input from PS/2 keyboard.
	Handle* h = terminal_get_active_node()->handle;
	if (h != NULL)
		h->read = ps2_keyboard_read;

	return 0;
}

MODULE_DEFAULT(nvme_init, NULL);
