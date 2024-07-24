#include <menix/common.h>
#include <menix/log.h>
#include <menix/module.h>

MODULE_FN i32 init_fn()
{
	module_log("Hello, world!\n");
	return 0;
}

MODULE_FN void exit_fn()
{
	module_log("Goodbye, world!\n");
}

MODULE = {
	.name = MODULE_NAME,
	.init = init_fn,
	.exit = exit_fn,
	.meta = MOULE_META_COMMON,
};
