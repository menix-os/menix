/*--------
USB driver
--------*/

#pragma once

#include <menix/drv/device.h>

typedef struct
{
	Device* parent;
} UsbDevice;
