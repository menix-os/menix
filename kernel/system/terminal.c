// Terminal Output

#include <menix/common.h>
#include <menix/fs/devtmpfs.h>
#include <menix/io/terminal.h>
#include <menix/util/cmd.h>
#include <menix/util/spin.h>

#include <stdlib.h>
#include <string.h>

static Terminal terminal_list[TERMINAL_MAX] = {0};
static usize terminal_active = 0;
static SpinLock terminal_lock = {0};

static Handle null_driver = {
	.read = NULL,
	.write = NULL,
	.ioctl = NULL,
};

i32 fbcon_init();

void terminal_init()
{
	for (usize i = 0; i < TERMINAL_MAX; i++)
	{
		if (terminal_list[i].driver == NULL)
		{
			terminal_list[i].driver = &null_driver;
		}

		char name[32] = "terminal";
		u32toa(i, name + 8, 10);
		devtmpfs_add_device(terminal_list[i].driver, name);
	}

	if (cmd_get_usize("fbcon", 1))
		fbcon_init();
}

void terminal_set_active(usize terminal)
{
	if (terminal > TERMINAL_MAX)
		return;

	spin_lock(&terminal_lock);
	terminal_active = terminal;
	spin_unlock(&terminal_lock);
}

usize terminal_get_active()
{
	return terminal_active;
}

VfsNode* terminal_get_active_node()
{
	char name[32] = "/dev/terminal";
	u32toa(terminal_active, name + 13, 10);
	return vfs_get_node(vfs_get_root(), name, false);
}

// Set the active terminal to display.
void terminal_set_driver(usize terminal, Handle* driver)
{
	if (terminal > TERMINAL_MAX)
		return;

	spin_lock(&terminal_lock);
	terminal_list[terminal].driver = driver;
	if (driver != NULL)
	{
		char name[32] = "terminal";
		u32toa(terminal, name + 8, 10);
		devtmpfs_add_device(terminal_list[terminal].driver, name);
	}
	spin_unlock(&terminal_lock);
}

void terminal_puts(usize terminal, const char* buf, usize len)
{
	if (terminal > TERMINAL_MAX)
		return;

	// Write each character to the buffer.
	Handle* handle = terminal_list[terminal].driver;
	if (handle != NULL && handle->write != NULL)
		handle->write(handle, NULL, buf, len, 0);
}
