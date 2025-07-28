#ifndef _MENIX_INIT_H
#define _MENIX_INIT_H

#include <menix/types.h>
#include <menix/util/attributes.h>

#define __init			   __used __section(".init.text") __cold
#define __initconst		   __used __section(".init.rodata")
#define __initdata		   __used __section(".init.data")
#define __initdata_prio(p) __used __section(".init.data." #p)

// The kernel's main init function.
void __noreturn kmain();

#endif
