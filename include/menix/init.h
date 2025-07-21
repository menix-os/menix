#ifndef _MENIX_INIT_H
#define _MENIX_INIT_H

#include <menix/types.h>

// Code reclaimed after boot.
#define __init				[[gnu::used, gnu::section(".init.text"), gnu::cold]]
// Data reclaimed after boot.
#define __initdata			[[gnu::used, gnu::section(".init.data")]]
#define __initdata_named(p) [[gnu::used, gnu::section(".init.data." #p)]]

struct boot_file {
	u8* address;
	usize length;
	const char* path;
};

extern struct boot_file boot_files[32];
extern usize boot_files_count;

[[noreturn]]
void kmain();

#endif
