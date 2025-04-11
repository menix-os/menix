// Advanced Configuration and Power Interface
// Wrapper for uACPI

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use crate::{
    arch::{PhysAddr, firmware},
    generic::{
        self,
        clock::{self},
        memory::{
            self,
            virt::{KERNEL_PAGE_TABLE, VmFlags},
        },
    },
};
use alloc::boxed::Box;
use core::{
    alloc::{Allocator, GlobalAlloc, Layout},
    ffi::{CStr, c_void},
    ptr::{NonNull, null_mut},
};
use spin::{Once, Spin, mutex::Mutex};
use uacpi::{
    self, uacpi_bool, uacpi_char, uacpi_cpu_flags, uacpi_firmware_request, uacpi_handle,
    uacpi_interrupt_handler, uacpi_io_addr, uacpi_log_level, uacpi_log_level_UACPI_LOG_DEBUG,
    uacpi_log_level_UACPI_LOG_ERROR, uacpi_log_level_UACPI_LOG_WARN, uacpi_pci_address,
    uacpi_phys_addr, uacpi_size, uacpi_status, uacpi_status_UACPI_STATUS_INTERNAL_ERROR,
    uacpi_status_UACPI_STATUS_MAPPING_FAILED, uacpi_status_UACPI_STATUS_OK,
    uacpi_status_UACPI_STATUS_UNIMPLEMENTED, uacpi_thread_id, uacpi_u8, uacpi_u16, uacpi_u32,
    uacpi_u64, uacpi_work_handler, uacpi_work_type,
};

static RSDP_ADDRESS: Once<PhysAddr> = Once::new();

pub fn init(rsdp: PhysAddr) {
    RSDP_ADDRESS.call_once(|| return rsdp);

    let mut uacpi_status = uacpi_status_UACPI_STATUS_OK;

    // Get an early table window so we can initialize e.g. HPET and MADT.
    let mut early_mem = Box::<[u8]>::new_uninit_slice(4096);
    uacpi_status = unsafe {
        uacpi::uacpi_setup_early_table_access(
            early_mem.as_mut_ptr() as *mut c_void,
            early_mem.len(),
        )
    };
    if uacpi_status != uacpi_status_UACPI_STATUS_OK {
        error!(
            "acpi: Early table access failed with error {}!\n",
            uacpi_status
        );
        return;
    }

    #[cfg(target_arch = "x86_64")]
    clock::switch(Box::new(firmware::Hpet::default()));

    print!("acpi: Initializing...\n");
    uacpi_status = unsafe { uacpi::uacpi_initialize(0) };
    if uacpi_status != uacpi_status_UACPI_STATUS_OK {
        error!(
            "acpi: Initialization failed with error \"{}\"!\n",
            uacpi_status
        );
    }

    // Setup the boot CPU.
    generic::percpu::setup_cpu();

    print!("acpi: Booting CPUs using MADT\n");
    // TODO: Evaluate MADT and initialize all remaining CPUs.

    uacpi_status = unsafe { uacpi::uacpi_namespace_load() };

    if uacpi_status != uacpi_status_UACPI_STATUS_OK {
        error!(
            "acpi: Namespace loading failed with error \"{}\"!\n",
            uacpi_status
        );
    } else {
        unsafe { uacpi::uacpi_namespace_initialize() };
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn uacpi_kernel_get_rsdp(out_rsdp_address: *mut uacpi_phys_addr) -> uacpi_status {
    match RSDP_ADDRESS.get() {
        Some(x) => unsafe {
            *out_rsdp_address = *x as uacpi_phys_addr;
            return uacpi_status_UACPI_STATUS_OK;
        },
        None => return uacpi_status_UACPI_STATUS_INTERNAL_ERROR,
    }
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_map(addr: uacpi_phys_addr, len: uacpi_size) -> *mut c_void {
    return KERNEL_PAGE_TABLE.write().map_memory(
        addr as PhysAddr,
        VmFlags::Read | VmFlags::Write,
        0,
        len,
    ) as *mut c_void;
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_unmap(addr: *mut c_void, len: uacpi_size) {
    // TODO
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_log(arg1: uacpi_log_level, arg2: *const uacpi_char) {
    let msg = unsafe { CStr::from_ptr(arg2) }.to_str().unwrap();
    match arg1 {
        uacpi_log_level_UACPI_LOG_WARN => warn!("acpi: {}", msg),
        uacpi_log_level_UACPI_LOG_ERROR => error!("acpi: {}", msg),
        _ => print!("acpi: {}", msg),
    }
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_pci_device_open(
    address: uacpi_pci_address,
    out_handle: *mut uacpi_handle,
) -> uacpi_status {
    // TODO
    return uacpi_status_UACPI_STATUS_UNIMPLEMENTED;
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_pci_device_close(arg1: uacpi_handle) {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_pci_read8(
    device: uacpi_handle,
    offset: uacpi_size,
    value: *mut uacpi_u8,
) -> uacpi_status {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_pci_read16(
    device: uacpi_handle,
    offset: uacpi_size,
    value: *mut uacpi_u16,
) -> uacpi_status {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_pci_read32(
    device: uacpi_handle,
    offset: uacpi_size,
    value: *mut uacpi_u32,
) -> uacpi_status {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_pci_write8(
    device: uacpi_handle,
    offset: uacpi_size,
    value: uacpi_u8,
) -> uacpi_status {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_pci_write16(
    device: uacpi_handle,
    offset: uacpi_size,
    value: uacpi_u16,
) -> uacpi_status {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_pci_write32(
    device: uacpi_handle,
    offset: uacpi_size,
    value: uacpi_u32,
) -> uacpi_status {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_io_map(
    base: uacpi_io_addr,
    len: uacpi_size,
    out_handle: *mut uacpi_handle,
) -> uacpi_status {
    let mem = KERNEL_PAGE_TABLE.write().map_memory(
        base as PhysAddr,
        VmFlags::Read | VmFlags::Write,
        0,
        len,
    );
    if mem == null_mut() {
        return uacpi_status_UACPI_STATUS_MAPPING_FAILED;
    }

    unsafe {
        (*out_handle) = mem as uacpi_handle;
    }
    return uacpi_status_UACPI_STATUS_OK;
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_io_unmap(handle: uacpi_handle) {
    // TODO
}
#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_alloc(size: uacpi_size) -> *mut c_void {
    return unsafe {
        memory::ALLOCATOR.alloc(Layout::from_size_align(size, 16).unwrap()) as *mut c_void
    };
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_free(mem: *mut c_void, size: uacpi_size) {
    // Frees of NULL are a no-op in uACPI.
    if mem == null_mut() {
        return;
    }

    unsafe {
        memory::ALLOCATOR.deallocate(
            NonNull::new(mem as *mut u8).unwrap(),
            Layout::from_size_align(size, 16).unwrap(),
        )
    };
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_get_nanoseconds_since_boot() -> uacpi_u64 {
    return clock::get_elapsed() as uacpi_u64;
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_stall(usec: uacpi_u8) {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_sleep(msec: uacpi_u64) {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_create_mutex() -> uacpi_handle {
    // TODO
    let mut b = Box::new(0);
    return Box::into_raw(b) as uacpi_handle;
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_free_mutex(arg1: uacpi_handle) {
    // TODO
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_create_event() -> uacpi_handle {
    // TODO
    let mut b = Box::new(0);
    return Box::into_raw(b) as uacpi_handle;
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_free_event(arg1: uacpi_handle) {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_get_thread_id() -> uacpi_thread_id {
    // TODO
    return 0 as uacpi_thread_id;
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_acquire_mutex(arg1: uacpi_handle, arg2: uacpi_u16) -> uacpi_status {
    // TODO
    return uacpi_status_UACPI_STATUS_UNIMPLEMENTED;
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_release_mutex(arg1: uacpi_handle) {
    // TODO
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_wait_for_event(arg1: uacpi_handle, arg2: uacpi_u16) -> uacpi_bool {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_signal_event(arg1: uacpi_handle) {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_reset_event(arg1: uacpi_handle) {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_handle_firmware_request(
    arg1: *mut uacpi_firmware_request,
) -> uacpi_status {
    todo!()
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_install_interrupt_handler(
    irq: uacpi_u32,
    arg1: uacpi_interrupt_handler,
    ctx: uacpi_handle,
    out_irq_handle: *mut uacpi_handle,
) -> uacpi_status {
    // TODO
    return uacpi_status_UACPI_STATUS_UNIMPLEMENTED;
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_uninstall_interrupt_handler(
    arg1: uacpi_interrupt_handler,
    irq_handle: uacpi_handle,
) -> uacpi_status {
    // TODO
    return uacpi_status_UACPI_STATUS_UNIMPLEMENTED;
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_create_spinlock() -> uacpi_handle {
    let mut b = Box::new(Mutex::<usize, Spin>::new(0));
    return Box::into_raw(b) as uacpi_handle;
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_free_spinlock(arg1: uacpi_handle) {
    // TODO
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_lock_spinlock(arg1: uacpi_handle) -> uacpi_cpu_flags {
    // TODO
    return 0;
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_unlock_spinlock(arg1: uacpi_handle, arg2: uacpi_cpu_flags) {
    // TODO
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_schedule_work(
    arg1: uacpi_work_type,
    arg2: uacpi_work_handler,
    ctx: uacpi_handle,
) -> uacpi_status {
    // TODO
    return uacpi_status_UACPI_STATUS_UNIMPLEMENTED;
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_wait_for_work_completion() -> uacpi_status {
    // TODO
    return uacpi_status_UACPI_STATUS_UNIMPLEMENTED;
}
