/*--------
USB driver
--------*/

#pragma once

#include <menix/drv/driver.h>

typedef struct
{
	Device* parent;
} UsbDevice;
