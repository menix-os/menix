#include <menix/common.h>
#include <menix/system/device.h>
#include <menix/util/log.h>

void dev_set_data(Device* dev, void* data)
{
	kassert(dev != NULL, "No device given!");
	dev->driver_data = data;
}

void* dev_get_data(Device* dev)
{
	kassert(dev != NULL, "No device given!");
	return dev->driver_data;
}
