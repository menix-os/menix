// Console/Terminal IO

#pragma once
#include <menix/common.h>
#include <menix/fs/handle.h>
#include <menix/fs/vfs.h>

#define TERMINAL_MAX 8

typedef struct
{
	Handle* driver;
} Terminal;

// Initializes all terminals.
void terminal_init();

// Set the active terminal to display.
void terminal_set_active(usize terminal);

// Gets the active terminal.
usize terminal_get_active();

// Gets the VFS node of the active terminal
VfsNode* terminal_get_active_node();

// Set the active terminal to display.
void terminal_set_driver(usize terminal, Handle* driver);

// Writes a string to the active terminal.
void terminal_puts(usize terminal, const char* buf, usize len);
