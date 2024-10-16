// Device driver model

#pragma once

#include <menix/common.h>

typedef struct Device Device;

// Callback for driver functions. The device argument is never NULL.
typedef i32 (*DriverFn)(Device* d);

// Driver structure. Contains all core functionality of the driver.
typedef struct Driver
{
	char* name;				// Name of the device.
	void* data;				// Custom driver structure.
	DriverFn bind;			// Called when a device is bound to the driver.
	DriverFn unbind;		// Called when a device is unbound from the driver.
	DriverFn connect;		// Called to connect a device.
	DriverFn disconnect;	// Called to disconnect a device.
	DriverFn prepare;		// Called when a device is about to be connected.
	DriverFn cleanup;		// Called after a device has been disconnected.
} Driver;
