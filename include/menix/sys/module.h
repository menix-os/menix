#ifndef _MENIX_SYS_MODULE_H
#define _MENIX_SYS_MODULE_H

#include <menix/posix/errno.h>
#include <menix/util/attributes.h>

#define MODULE \
    [[__section(".mod")]] \
    struct module_info __this_module

#ifdef __MODULE__
extern struct module_info __this_module;
#define THIS_MODULE (&__this_module)
#else
#define THIS_MODULE ((struct module_info*)0)
#endif

struct module_info {
    const char name[64];
    errno_t (*init)();
    errno_t (*exit)();
};

#endif
