#include <menix/common.h>
#include <menix/drv/driver.h>
#include <menix/stdint.h>
#include <menix/stdio.h>

static void example_say_hello()
{
	printf("Hello, world!\n");
}

static int32_t bind(Device* d)
{
	printf("loaded the example driver!\n");
	example_say_hello();
	return 0;
}

static int32_t unbind(Device* d)
{
	printf("bye!\n");
	return 0;
}

MENIX_DRIVER(example) = {
	.name = "my_example_driver",
	.type = DeviceType_Misc,
	.bind = bind,
	.unbind = unbind,
};
