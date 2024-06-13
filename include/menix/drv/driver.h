/*-----------------
Device driver model
-----------------*/

#pragma once

#include <menix/common.h>
#include <menix/stdint.h>

// Driver instance.
typedef struct Device
{
	const char*			 name;		// The name of the device.
	const struct Driver* driver;	// The driver currently mapped to the device.
} Device;

// Callback for driver functions. The device argument is never NULL.
typedef int32_t (*DriverFn)(Device* d);

typedef enum : uint16_t
{
	DriverType_Other,	 // Unrelated to a specific system.
	DriverType_PCI,		 // PCI/PCIe driver.
	DriverType_USB,		 // USB driver.
} DriverType;

// Driver structure. Contains all core functionality of the driver.
typedef struct Driver
{
	char*	   name;	// Name of the device.
	DriverType type;	// Which subsystem the driver belongs to.
	void*	   data;	// Generic driver data.

	DriverFn bind;			// Called when a device is bound to the driver.
	DriverFn unbind;		// Called when a device is unbound from the driver.
	DriverFn connect;		// Called to connect a device.
	DriverFn disconnect;	// Called to disconnect a device.
	DriverFn prepare;		// Called when a device is about to be connected.
	DriverFn cleanup;		// Called after a device has been disconnected.
} ATTR(packed) Driver;
