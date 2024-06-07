/*-------------------
Driver initialization
-------------------*/

#include <menix/common.h>
#include <menix/drv/driver.h>
#include <menix/stdio.h>

void drv_init()
{
	// Calculate the driver count.
	const uint32_t driver_count = (drv_end - drv_start) / sizeof(Driver);
	const Driver*  drivers = (Driver*)drv_start;

	// Bind all drivers.
	// TODO: Use Device Tree to filter compatible strings.
	for (uint32_t i = 0; i < driver_count; i++)
	{
		drivers[i].bind(NULL);
	}
	printf("Loaded %u drivers\n", driver_count);
}
