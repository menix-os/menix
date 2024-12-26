// uDRM Bridge

#include <menix/common.h>
#include <menix/io/mmio.h>
#include <menix/system/pci/pci.h>

#include <uapi/status.h>
#include <uapi/types.h>

uapi_status uapi_kernel_pci_cfg_read(uapi_handle handle, uapi_size offset, uapi_u8 byte_width, uapi_u64* value)
{
	PciDevice* const device = handle;
	switch (byte_width)
	{
		case sizeof(u8): mmio_read8(device->config_space + offset); break;
		case sizeof(u16): mmio_read16(device->config_space + offset); break;
		case sizeof(u32): mmio_read32(device->config_space + offset); break;
	}
	return UAPI_STATUS_OK;
}

uapi_status uapi_kernel_pci_cfg_write(uapi_handle handle, uapi_size offset, uapi_u8 byte_width, uapi_u64 value)
{
	PciDevice* const device = handle;
	switch (byte_width)
	{
		case sizeof(u8): mmio_write8(device->config_space + offset, value); break;
		case sizeof(u16): mmio_write16(device->config_space + offset, value); break;
		case sizeof(u32): mmio_write32(device->config_space + offset, value); break;
	}
	return UAPI_STATUS_OK;
}
