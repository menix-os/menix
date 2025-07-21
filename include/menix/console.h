#ifndef _MENIX_CONSOLE_H
#define _MENIX_CONSOLE_H

#include <menix/types.h>
#include <menix/hint.h>

struct console {
	char name[16];
	void (*write)(struct console* con, const char* buf, usize count);
	usize (*read)(struct console* con, char* buf, usize count);
	int (*init)(struct console* con, char* options);
};

#endif
