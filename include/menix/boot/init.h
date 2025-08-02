#ifndef _MENIX_BOOT_INIT_H
#define _MENIX_BOOT_INIT_H

#ifndef __MODULE__
#define __init                  __used, __section(".init.text"), __cold
#define __initdata              __used, __section(".init.data")
#define __initdata_sorted(name) __used, __section(".init.data." name)
#endif

#endif
