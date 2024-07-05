#include <menix/common.h>
#include <menix/log.h>
#include <menix/module.h>

MODULE_FN int32_t init_fn()
{
	kmesg(LOG_INFO, "Loading example...\n");
	return 0;
}

MODULE_FN void exit_fn()
{
	kmesg(LOG_INFO, "Unloading example...\n");
}

MODULE = {
	.name = MODULE_NAME,
	.init = init_fn,
	.exit = exit_fn,
	.meta = MOULE_META(name, MODULE_NAME),
};
