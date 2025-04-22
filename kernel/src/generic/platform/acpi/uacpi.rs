#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use core::{
    alloc::Layout,
    ffi::{CStr, c_void},
    ptr::{NonNull, null_mut},
};

use alloc::{alloc::Allocator, alloc::GlobalAlloc, boxed::Box};
use spin::Mutex;
pub use uacpi::*;

use crate::generic::{
    clock,
    memory::{
        self, PhysAddr,
        virt::{KERNEL_PAGE_TABLE, VmFlags},
    },
};

#[unsafe(no_mangle)]
unsafe extern "C" fn uacpi_kernel_get_rsdp(out_rsdp_address: *mut uacpi_phys_addr) -> uacpi_status {
    match super::RSDP_ADDRESS.get() {
        Some(x) => unsafe {
            *out_rsdp_address = x.0 as uacpi_phys_addr;
            return uacpi_status_UACPI_STATUS_OK;
        },
        None => return uacpi_status_UACPI_STATUS_INTERNAL_ERROR,
    }
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_map(addr: uacpi_phys_addr, len: uacpi_size) -> *mut c_void {
    return KERNEL_PAGE_TABLE.write().map_memory(
        PhysAddr(addr as usize),
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
        PhysAddr(base as usize),
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
        memory::heap::ALLOCATOR.alloc(Layout::from_size_align(size, align_of::<usize>()).unwrap())
            as *mut c_void
    };
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_free(mem: *mut c_void, size: uacpi_size) {
    // Frees of NULL are a no-op in uACPI.
    if mem == null_mut() {
        return;
    }

    unsafe {
        memory::heap::ALLOCATOR.dealloc(
            mem as *mut u8,
            Layout::from_size_align(size, align_of::<usize>()).unwrap(),
        )
    };
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_get_nanoseconds_since_boot() -> uacpi_u64 {
    return clock::get_elapsed() as uacpi_u64;
}

#[unsafe(no_mangle)]
extern "C" fn uacpi_kernel_stall(usec: uacpi_u8) {
    clock::wait_ns(usec as usize);
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
    let mut b = Box::new(Mutex::<usize>::new(0));
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
