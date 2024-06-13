#include <menix/common.h>
#include <menix/module.h>
#include <menix/stdint.h>
#include <menix/stdio.h>

static int32_t init_fn()
{
	printf("Loading example...\n");
	printf("-> Hello, world!\n");
	return 0;
}

static void exit_fn()
{
	printf("Unloading example...\n");
	return;
}

MENIX_MODULE = {
	.init = init_fn,
	.exit = exit_fn,
	MENIX_MODULE_META,
};
