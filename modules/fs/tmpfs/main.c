// tmpfs file system

#include <menix/common.h>
#include <menix/module.h>

MODULE_FN i32 tmpfs_init()
{
	return 0;
}

MODULE = {
	.name = MODULE_NAME,
	.init = tmpfs_init,
	MODULE_META,
};
