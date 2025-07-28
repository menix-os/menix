#ifndef _MENIX_INIT_FILE_H
#define _MENIX_INIT_FILE_H

#include <menix/types.h>
#include <menix/init.h>

struct boot_file {
	u8* data;
	usize length;
	const char* path;
};

extern struct boot_file boot_files[32];
extern usize boot_files_count;

#endif
