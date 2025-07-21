#ifndef _MENIX_INIT_H
#define _MENIX_INIT_H

#include <menix/types.h>

// Code reclaimed after boot.
#define __init			   __attribute__((used, section(".init.text"), cold))
// Data reclaimed after boot.
#define __initconst		   __attribute__((used, section(".init.rodata")))
#define __initdata		   __attribute__((used, section(".init.data")))
#define __initdata_prio(p) __attribute__((used, section(".init.data." #p)))
#define __noreturn		   __attribute__((noreturn))

void __noreturn kmain();

#endif
