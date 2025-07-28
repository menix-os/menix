#ifndef _MENIX_INIT_CMDLINE_H
#define _MENIX_INIT_CMDLINE_H

struct cmdline_option {
	const char* name;
	int foo;
};

void cmdline_setup(const char* cmdline);

#endif
