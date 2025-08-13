#ifndef _KERNEL_BOOT_INIT_H
#define _KERNEL_BOOT_INIT_H

#include <kernel/util/attributes.h>
#include <kernel/util/common.h>

#define __init                  __used, __section(".init.text"), __cold
#define __initdata              __used, __section(".init.data")
#define __initdata_sorted(name) __used, __section(".init.data." name)

#endif
