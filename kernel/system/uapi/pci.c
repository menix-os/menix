// uAPI pci.h implementation

#include <menix/common.h>
#include <menix/io/mmio.h>
#include <menix/system/pci/pci.h>

#include <uapi/status.h>
#include <uapi/types.h>

#define UAPI_KERNEL_API
#define UAPI_WANTS_PCI
#include <uapi/pci.h>
#undef UAPI_KERNEL_API

uapi_status uapi_kernel_pci_cfg_read(uapi_handle handle, uapi_size offset, uapi_u8 byte_width, uapi_u64* value)
{
	PciDevice* const device = handle;
	switch (byte_width)
	{
		case sizeof(u8): *value = mmio_read8(device->config_space_addr + offset); break;
		case sizeof(u16): *value = mmio_read16(device->config_space_addr + offset); break;
		case sizeof(u32): *value = mmio_read32(device->config_space_addr + offset); break;
	}
	return UAPI_STATUS_OK;
}

uapi_status uapi_kernel_pci_cfg_write(uapi_handle handle, uapi_size offset, uapi_u8 byte_width, uapi_u64 value)
{
	PciDevice* const device = handle;
	switch (byte_width)
	{
		case sizeof(u8): mmio_write8(device->config_space_addr + offset, value); break;
		case sizeof(u16): mmio_write16(device->config_space_addr + offset, value); break;
		case sizeof(u32): mmio_write32(device->config_space_addr + offset, value); break;
	}
	return UAPI_STATUS_OK;
}

uapi_status uapi_kernel_pci_set_ctx(uapi_handle handle, void* ctx)
{
	PciDevice* const device = handle;
	device->dev->driver_data = ctx;
	return UAPI_STATUS_OK;
}

void* uapi_kernel_pci_get_ctx(uapi_handle handle)
{
	PciDevice* const device = handle;
	return device->dev->driver_data;
}
