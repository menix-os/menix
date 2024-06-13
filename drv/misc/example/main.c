#include <menix/common.h>
#include <menix/module.h>
#include <menix/stdio.h>

static int32_t init_fn()
{
	printf("Loading example...\n");
	return 0;
}

static void exit_fn()
{
	printf("Unloading example...\n");
}

MENIX_MODULE(.init = init_fn, .exit = exit_fn);
