// Device abstraction

#pragma once
#include <menix/common.h>

// Driver instance.
typedef struct Device Device;
struct Device
{
	const char* name;	  // The name of the device.
	Device* parent;		  // Parent of this device, e.g. a bus or controller.
	void* driver_data;	  // Driver data. Use `dev_{s,g}et_driver_data` to modify.
};

// Returns the driver_data field.
void* dev_get_data(Device* dev);

// Sets the driver_data field.
void dev_set_data(Device* dev, void* data);
