#include <menix/memory/pm.h>
#include <menix/memory/vm.h>
#include <menix/system/acpi/madt.h>
#include <menix/system/acpi/mcfg.h>
#include <menix/system/arch.h>
#include <menix/system/interrupts.h>
#include <menix/system/pci/pci.h>
#include <menix/system/time/clock.h>
#include <menix/util/log.h>
#include <menix/util/spin.h>

#include <uacpi/kernel_api.h>
#include <uacpi/status.h>
#include <uacpi/types.h>
#include <uacpi/uacpi.h>

#ifdef __x86_64__
#include <hpet.h>
#endif

static PhysAddr acpi_rsdp;

void acpi_init(PhysAddr rsdp)
{
	acpi_rsdp = rsdp;

	void* temp_buffer = kmalloc(4096);
	uacpi_setup_early_table_access(temp_buffer, 4096);

#ifdef __x86_64__
	hpet_init();
	madt_init();
#endif

	mcfg_init();

	uacpi_initialize(0);
	kfree(temp_buffer);

	uacpi_namespace_load();
}

uacpi_status uacpi_kernel_get_rsdp(uacpi_phys_addr* out_rsdp_address)
{
	if (acpi_rsdp == 0)
	{
		print_warn("acpi: No RSDP was set!\n");
		return UACPI_STATUS_INTERNAL_ERROR;
	}

	*out_rsdp_address = acpi_rsdp;
	return UACPI_STATUS_OK;
}

void uacpi_kernel_free(void* mem)
{
	kfree(mem);
}

void* uacpi_kernel_alloc(uacpi_size size)
{
	return kmalloc(size);
}

uacpi_status uacpi_kernel_pci_device_open(uacpi_pci_address address, uacpi_handle* out_handle)
{
	*out_handle = pci_platform.buses.items[address.bus]->slots[address.device].devices[address.function];
	return UACPI_STATUS_OK;
}

void uacpi_kernel_pci_device_close(uacpi_handle)
{
}

uacpi_status uacpi_kernel_pci_read(uacpi_handle device, uacpi_size offset, uacpi_u8 byte_width, uacpi_u64* value)
{
	PciDevice* dev = device;
	switch (byte_width)
	{
		case sizeof(u8): *value = mmio_read8(dev->config_space_addr + offset); break;
		case sizeof(u16): *value = mmio_read16(dev->config_space_addr + offset); break;
		case sizeof(u32): *value = mmio_read32(dev->config_space_addr + offset); break;
		default: return UACPI_STATUS_INVALID_ARGUMENT;
	}
	return UACPI_STATUS_OK;
}

uacpi_status uacpi_kernel_pci_write(uacpi_handle device, uacpi_size offset, uacpi_u8 byte_width, uacpi_u64 value)
{
	PciDevice* dev = device;
	switch (byte_width)
	{
		case sizeof(u8): mmio_write8(dev->config_space_addr + offset, value); break;
		case sizeof(u16): mmio_write16(dev->config_space_addr + offset, value); break;
		case sizeof(u32): mmio_write32(dev->config_space_addr + offset, value); break;
		default: return UACPI_STATUS_INVALID_ARGUMENT;
	}
	return UACPI_STATUS_OK;
}

uacpi_status uacpi_kernel_pci_read8(uacpi_handle device, uacpi_size offset, uacpi_u8* value)
{
	*value = mmio_read8(((PciDevice*)device)->config_space_addr + offset);
	return UACPI_STATUS_OK;
}

uacpi_status uacpi_kernel_pci_read16(uacpi_handle device, uacpi_size offset, uacpi_u16* value)
{
	*value = mmio_read16(((PciDevice*)device)->config_space_addr + offset);
	return UACPI_STATUS_OK;
}

uacpi_status uacpi_kernel_pci_read32(uacpi_handle device, uacpi_size offset, uacpi_u32* value)
{
	*value = mmio_read32(((PciDevice*)device)->config_space_addr + offset);
	return UACPI_STATUS_OK;
}

uacpi_status uacpi_kernel_pci_write8(uacpi_handle device, uacpi_size offset, uacpi_u8 value)
{
	mmio_write8(((PciDevice*)device)->config_space_addr + offset, value);
	return UACPI_STATUS_OK;
}

uacpi_status uacpi_kernel_pci_write16(uacpi_handle device, uacpi_size offset, uacpi_u16 value)
{
	mmio_write16(((PciDevice*)device)->config_space_addr + offset, value);
	return UACPI_STATUS_OK;
}

uacpi_status uacpi_kernel_pci_write32(uacpi_handle device, uacpi_size offset, uacpi_u32 value)
{
	mmio_write32(((PciDevice*)device)->config_space_addr + offset, value);
	return UACPI_STATUS_OK;
}

uacpi_status uacpi_kernel_io_map(uacpi_io_addr base, uacpi_size len, uacpi_handle* out_handle)
{
	*out_handle = vm_map_memory(base, len, VMProt_Read | VMProt_Write);
	return UACPI_STATUS_OK;
}

void uacpi_kernel_io_unmap(uacpi_handle handle)
{
	// TODO
}

uacpi_status uacpi_kernel_io_read8(uacpi_handle h, uacpi_size offset, uacpi_u8* out_value)
{
	*out_value = mmio_read8(h + offset);
	return UACPI_STATUS_OK;
}

uacpi_status uacpi_kernel_io_read16(uacpi_handle h, uacpi_size offset, uacpi_u16* out_value)
{
	*out_value = mmio_read16(h + offset);
	return UACPI_STATUS_OK;
}

uacpi_status uacpi_kernel_io_read32(uacpi_handle h, uacpi_size offset, uacpi_u32* out_value)
{
	*out_value = mmio_read32(h + offset);
	return UACPI_STATUS_OK;
}

uacpi_status uacpi_kernel_io_write8(uacpi_handle h, uacpi_size offset, uacpi_u8 in_value)
{
	mmio_write8(h + offset, in_value);
	return UACPI_STATUS_OK;
}

uacpi_status uacpi_kernel_io_write16(uacpi_handle h, uacpi_size offset, uacpi_u16 in_value)
{
	mmio_write16(h + offset, in_value);
	return UACPI_STATUS_OK;
}

uacpi_status uacpi_kernel_io_write32(uacpi_handle h, uacpi_size offset, uacpi_u32 in_value)
{
	mmio_write32(h + offset, in_value);
	return UACPI_STATUS_OK;
}

void* uacpi_kernel_map(uacpi_phys_addr addr, uacpi_size len)
{
	return addr + pm_get_phys_base();
}

void uacpi_kernel_unmap(void* addr, uacpi_size len)
{
}

void uacpi_kernel_log(uacpi_log_level level, const uacpi_char* msg)
{
	switch (level)
	{
		case UACPI_LOG_INFO:
		case UACPI_LOG_TRACE:
		case UACPI_LOG_DEBUG: print_log("acpi: %s", msg); break;
		case UACPI_LOG_WARN: print_warn("acpi: %s", msg); break;
		case UACPI_LOG_ERROR: print_error("acpi: %s", msg); break;
	}
}

uacpi_u64 uacpi_kernel_get_nanoseconds_since_boot(void)
{
	return clock_get_elapsed();
}

void uacpi_kernel_stall(uacpi_u8 usec)
{
	clock_wait(usec * 1000);
}

void uacpi_kernel_sleep(uacpi_u64 msec)
{
	// TODO: Convert to sleep
	clock_wait(msec * 1000000);
}

uacpi_handle uacpi_kernel_create_event(void)
{
	return kzalloc(8);
}

void uacpi_kernel_free_event(uacpi_handle handle)
{
	kfree(handle);
}

uacpi_thread_id uacpi_kernel_get_thread_id(void)
{
	return (void*)arch_current_cpu();
}

uacpi_handle uacpi_kernel_create_mutex(void)
{
	return kzalloc(8);
}

void uacpi_kernel_free_mutex(uacpi_handle mutex)
{
	kfree(mutex);
}

uacpi_status uacpi_kernel_acquire_mutex(uacpi_handle, uacpi_u16)
{
	return UACPI_STATUS_OK;
}

void uacpi_kernel_release_mutex(uacpi_handle)
{
}

uacpi_bool uacpi_kernel_wait_for_event(uacpi_handle, uacpi_u16)
{
	return false;
}

void uacpi_kernel_signal_event(uacpi_handle)
{
}

void uacpi_kernel_reset_event(uacpi_handle)
{
}

uacpi_status uacpi_kernel_handle_firmware_request(uacpi_firmware_request*)
{
	return UACPI_STATUS_UNIMPLEMENTED;
}

static Context* irq_handler_wrapper(usize isr, Context* context, void* data)
{
	void** frame = (void**)data;
	((uacpi_interrupt_handler)frame[0])(frame[1]);
	return context;
}

uacpi_status uacpi_kernel_install_interrupt_handler(uacpi_u32 irq, uacpi_interrupt_handler handler, uacpi_handle ctx,
													uacpi_handle* out_irq_handle)
{
	void** context = kzalloc(sizeof(void*) * 2);
	context[0] = handler;
	context[1] = ctx;

	isr_register_handler(arch_current_cpu()->id, irq + 32, irq_handler_wrapper, context);
	*out_irq_handle = context;
	return UACPI_STATUS_OK;
}

uacpi_status uacpi_kernel_uninstall_interrupt_handler(uacpi_interrupt_handler handler, uacpi_handle irq_handle)
{
	void** frame = (void**)irq_handle;

	// TODO
	(void)frame;
	// isr_unregister_handler(frame[0]);
	kfree(irq_handle);
	return UACPI_STATUS_OK;
}

uacpi_handle uacpi_kernel_create_spinlock(void)
{
	SpinLock* lock = kzalloc(sizeof(SpinLock));
	return lock;
}

void uacpi_kernel_free_spinlock(uacpi_handle lock)
{
	kfree(lock);
}

uacpi_cpu_flags uacpi_kernel_lock_spinlock(uacpi_handle lock)
{
	spin_lock(lock);
	return 0;
}

void uacpi_kernel_unlock_spinlock(uacpi_handle lock, uacpi_cpu_flags)
{
	spin_unlock(lock);
}

uacpi_status uacpi_kernel_schedule_work(uacpi_work_type type, uacpi_work_handler handler, uacpi_handle ctx)
{
	todo();
	return UACPI_STATUS_UNIMPLEMENTED;
}

uacpi_status uacpi_kernel_wait_for_work_completion(void)
{
	todo();
	return UACPI_STATUS_UNIMPLEMENTED;
}
