use core::str;
use limine::{request::*, BaseRevision};

use crate::{
    arch::{Arch, CommonArch, VirtAddr},
    memory::pm::PhysMemory,
};

use super::BootInfo;

#[used]
#[link_section = ".requests_start_marker"]
static START_MARKER: RequestsStartMarker = RequestsStartMarker::new();
#[used]
#[link_section = ".limine_requests"]
static BASE_REVISION: BaseRevision = BaseRevision::new();
#[used]
#[link_section = ".limine_requests"]
static MEMMAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();
#[used]
#[link_section = ".limine_requests"]
static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();
#[used]
#[link_section = ".limine_requests"]
static KERNEL_ADDR_REQUEST: KernelAddressRequest = KernelAddressRequest::new();
#[used]
#[link_section = ".limine_requests"]
static KERNEL_FILE_REQUEST: KernelFileRequest = KernelFileRequest::new();
#[used]
#[link_section = ".limine_requests"]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();
#[used]
#[link_section = ".limine_requests"]
static MODULE_REQUEST: ModuleRequest = ModuleRequest::new();
#[used]
#[link_section = ".limine_requests"]
static SMP_REQUEST: SmpRequest = SmpRequest::new();
#[cfg(feature = "fw_acpi")]
#[used]
#[link_section = ".limine_requests"]
static RSDP_REQUEST: RsdpRequest = RsdpRequest::new();
#[cfg(feature = "fw_of")]
#[used]
#[link_section = ".limine_requests"]
static DTB_REQUEST: DeviceTreeBlobRequest = DeviceTreeBlobRequest::new();
#[used]
#[link_section = ".requests_end_marker"]
static END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

/// This is the absolute entry point of menix.
#[no_mangle]
unsafe extern "C" fn kernel_boot() -> ! {
    unsafe {
        let mut info = BootInfo::default();

        // Get kernel physical and virtual base.
        let kernel_addr = KERNEL_ADDR_REQUEST.get_response().unwrap();
        info.kernel_addr = (kernel_addr.physical_base(), kernel_addr.virtual_base());

        // Get memory map. Has to be a fixed buffer since at this point there is no memory allocator available yet.
        let map = MEMMAP_REQUEST.get_response().unwrap().entries();
        let memory_map = Vec::new;

        info.memory_map;

        // Initialize the allocator and serial output.
        Arch::early_init(&info);

        // Get command line.
        info.command_line = {
            // Convert the command line from bytes to UTF-8 if there is any.
            let line_buf = KERNEL_FILE_REQUEST.get_response().unwrap().file().cmdline();
            if line_buf.len() > 0 {
                Some(str::from_utf8(line_buf).unwrap())
            } else {
                None
            }
        };

        // Finalize CPU initialization.
        Arch::init(&info);

        #[cfg(feature = "fw_acpi")]
        {
            // Initialize ACPI.
            info.rsdp_addr = RSDP_REQUEST.get_response().unwrap().address() as VirtAddr;
        }
    }

    // TODO: Invoke the scheduler.

    loop {}
}
