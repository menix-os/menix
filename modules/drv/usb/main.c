#include <menix/common.h>
#include <menix/drv/module.h>
#include <menix/util/log.h>

MODULE_FN i32 init_fn()
{
	// TODO
	return 0;
}

MODULE_FN void exit_fn()
{
}

MODULE = {
	.name = MODULE_NAME,
	.init = init_fn,
	.exit = exit_fn,
	MODULE_META,
};
