#include <menix/memory/pm.h>
#include <menix/system/acpi/madt.h>
#include <menix/system/arch.h>
#include <menix/system/interrupts.h>
#include <menix/system/pci/pci.h>
#include <menix/system/time/clock.h>
#include <menix/util/log.h>
#include <menix/util/spin.h>

#include <uacpi/kernel_api.h>
#include <uacpi/types.h>
#include <uacpi/uacpi.h>

#ifdef CONFIG_pci
#include <menix/system/acpi/mcfg.h>
#endif

#ifdef CONFIG_arch_x86_64
#include <hpet.h>
#endif

static PhysAddr acpi_rsdp;

#undef todo
#define todo()

void acpi_init(PhysAddr rsdp)
{
	acpi_rsdp = rsdp;

	void* temp_buffer = kmalloc(4096);
	uacpi_setup_early_table_access(temp_buffer, 4096);

#ifdef CONFIG_arch_x86_64
	hpet_init();
	madt_init();
#endif

#ifdef CONFIG_pci
	mcfg_init();
#endif

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

void* uacpi_kernel_calloc(uacpi_size count, uacpi_size size)
{
	return kzalloc(count * size);
}

uacpi_status uacpi_kernel_raw_memory_read(uacpi_phys_addr address, uacpi_u8 byte_width, uacpi_u64* out_value)
{
	void* ptr = pm_get_phys_base() + address;
	switch (byte_width)
	{
		case sizeof(u8): *out_value = (*(mmio8*)ptr); break;
		case sizeof(u16): *out_value = (*(mmio16*)ptr); break;
		case sizeof(u32): *out_value = (*(mmio32*)ptr); break;
		case sizeof(u64): *out_value = (*(mmio64*)ptr); break;
		default: return UACPI_STATUS_INVALID_ARGUMENT;
	}
	return UACPI_STATUS_OK;
}

uacpi_status uacpi_kernel_raw_memory_write(uacpi_phys_addr address, uacpi_u8 byte_width, uacpi_u64 in_value)
{
	volatile void* ptr = pm_get_phys_base() + address;
	switch (byte_width)
	{
		case sizeof(u8): (*(u8*)ptr) = in_value; break;
		case sizeof(u16): (*(u16*)ptr) = in_value; break;
		case sizeof(u32): (*(u32*)ptr) = in_value; break;
		case sizeof(u64): (*(u64*)ptr) = in_value; break;
		default: return UACPI_STATUS_INVALID_ARGUMENT;
	}
	return UACPI_STATUS_OK;
}

uacpi_status uacpi_kernel_raw_io_read(uacpi_io_addr address, uacpi_u8 byte_width, uacpi_u64* out_value)
{
	void* ptr = pm_get_phys_base() + address;
	switch (byte_width)
	{
		case sizeof(u8): *out_value = (*(mmio8*)ptr); break;
		case sizeof(u16): *out_value = (*(mmio16*)ptr); break;
		case sizeof(u32): *out_value = (*(mmio32*)ptr); break;
		default: return UACPI_STATUS_INVALID_ARGUMENT;
	}
	return UACPI_STATUS_OK;
}

uacpi_status uacpi_kernel_raw_io_write(uacpi_io_addr address, uacpi_u8 byte_width, uacpi_u64 in_value)
{
	volatile void* ptr = pm_get_phys_base() + address;
	switch (byte_width)
	{
		case sizeof(u8): (*(u8*)ptr) = in_value; break;
		case sizeof(u16): (*(u16*)ptr) = in_value; break;
		case sizeof(u32): (*(u32*)ptr) = in_value; break;
		default: return UACPI_STATUS_INVALID_ARGUMENT;
	}
	return UACPI_STATUS_OK;
}

uacpi_status uacpi_kernel_pci_read(uacpi_pci_address* address, uacpi_size offset, uacpi_u8 byte_width, uacpi_u64* value)
{
	return UACPI_STATUS_UNIMPLEMENTED;
}

uacpi_status uacpi_kernel_pci_write(uacpi_pci_address* address, uacpi_size offset, uacpi_u8 byte_width, uacpi_u64 value)
{
	todo();
	return UACPI_STATUS_UNIMPLEMENTED;
}

uacpi_status uacpi_kernel_io_map(uacpi_io_addr base, uacpi_size len, uacpi_handle* out_handle)
{
	todo();
	return UACPI_STATUS_UNIMPLEMENTED;
}

void uacpi_kernel_io_unmap(uacpi_handle handle)
{
	todo();
}

uacpi_status uacpi_kernel_io_read(uacpi_handle, uacpi_size offset, uacpi_u8 byte_width, uacpi_u64* value)
{
	todo();
	return UACPI_STATUS_UNIMPLEMENTED;
}

uacpi_status uacpi_kernel_io_write(uacpi_handle, uacpi_size offset, uacpi_u8 byte_width, uacpi_u64 value)
{
	todo();
	return UACPI_STATUS_UNIMPLEMENTED;
}

void* uacpi_kernel_map(uacpi_phys_addr addr, uacpi_size len)
{
	return addr + pm_get_phys_base();
}

void uacpi_kernel_unmap(void* addr, uacpi_size len)
{
	(void)addr;
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
	clock_wait(msec * 1000000);
}

uacpi_handle uacpi_kernel_create_mutex(void)
{
	todo();
	return kzalloc(8);
}

void uacpi_kernel_free_mutex(uacpi_handle mutex)
{
	todo();
}

uacpi_handle uacpi_kernel_create_event(void)
{
	todo();
	return kzalloc(8);
}

void uacpi_kernel_free_event(uacpi_handle)
{
	todo();
}

uacpi_thread_id uacpi_kernel_get_thread_id(void)
{
	todo();
	return (void*)arch_current_cpu();
}

uacpi_status uacpi_kernel_acquire_mutex(uacpi_handle, uacpi_u16)
{
	todo();
	return UACPI_STATUS_OK;
}

void uacpi_kernel_release_mutex(uacpi_handle)
{
	todo();
}

uacpi_bool uacpi_kernel_wait_for_event(uacpi_handle, uacpi_u16)
{
	todo();
	return false;
}

void uacpi_kernel_signal_event(uacpi_handle)
{
	todo();
}

void uacpi_kernel_reset_event(uacpi_handle)
{
	todo();
}

uacpi_status uacpi_kernel_handle_firmware_request(uacpi_firmware_request*)
{
	todo();
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

	isr_register_handler(arch_current_cpu()->id, irq, irq_handler_wrapper, context);
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
