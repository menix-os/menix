/*-----------------
Device driver model
-----------------*/

#pragma once

#include <menix/common.h>
#include <menix/stdint.h>

// Driver instance.
typedef struct Device
{
	const struct Driver* driver;	// The driver currently mapped to the device.
	const char*			 name;		// The name of the device.
} Device;

// Callback for driver functions.
typedef int32_t (*DriverFn)(Device* d);

typedef enum
{
	DeviceType_Misc,	// Unrelated to a specific system.
	DeviceType_PCI,		// PCI/PCIe driver.
	DeviceType_USB,		// USB driver.
} DriverType;

// Driver structure. Contains all core functionality of the driver.
typedef struct Driver
{
	char*	   name;		  // Name of the device.
	DriverType type;		  // Which subsystem the driver belongs to.
	DriverFn   bind;		  // Called when a device is bound to the driver.
	DriverFn   unbind;		  // Called when a device is unbound from the driver.
	DriverFn   connect;		  // Called to connect a device.
	DriverFn   prepare;		  // Called when a device is about to be connected.
	DriverFn   disconnect;	  // Called to disconnect a device.
	DriverFn   cleanup;		  // Called after a device has been disconnected.
} MENIX_ATTR(packed) Driver;

// Declare a new driver. Drivers should use this at the end of their source.
#define MENIX_DRIVER(name) MENIX_ATTR(used) MENIX_ATTR(section(".drv")) static Driver menix_drv_##name

// Start and end of the driver section. Defined in the linker script.
extern size_t drv_start;
extern size_t drv_end;

// Initialize all drivers.
void drv_init();
