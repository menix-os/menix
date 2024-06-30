#include <menix/common.h>
#include <menix/log.h>
#include <menix/module.h>

static int32_t init_fn()
{
	kmesg(LOG_INFO, "Loading example...\n");
	return 0;
}

static void exit_fn()
{
	kmesg(LOG_INFO, "Unloading example...\n");
}

MENIX_MODULE = {
	.name = MODULE_NAME,
	.init = init_fn,
	.exit = exit_fn,
};
