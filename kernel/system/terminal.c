// Terminal Output

#include <menix/common.h>
#include <menix/io/terminal.h>
#include <menix/util/cmd.h>
#include <menix/util/log.h>
#include <menix/util/spin.h>

Terminal terminal_global = {0};
static SpinLock terminal_lock = {0};

i32 fbcon_init();

void terminal_init()
{
	if (cmd_get_usize("fbcon", 1))
		fbcon_init();
}

void terminal_puts(const char* buf, usize len)
{
	spin_lock(&terminal_lock);
	if (terminal_global.write)
	{
		terminal_global.write(buf, len);
	}
	spin_unlock(&terminal_lock);
}
