// Device abstraction

#pragma once
#include <menix/common.h>
#include <menix/drv/driver.h>

// Driver instance.
typedef struct Device Device;
typedef struct Device
{
	const char* name;	  // The name of the device.
	Device* parent;		  // Parent of this device, e.g. a bus or controller.
	Driver* driver;		  // The driver currently bound to the device.
	void* driver_data;	  // Driver data. Use `dev_{s,g}et_driver_data` to modify.
} Device;
