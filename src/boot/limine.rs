use super::BootInfo;
use crate::{
    arch::{Arch, CommonArch, VirtAddr},
    memory::pm::{PhysMemory, PhysMemoryUsage::Unknown},
};
use core::str;
use limine::{request::*, BaseRevision};

#[link_section = ".requests_start_marker"]
pub static START_MARKER: RequestsStartMarker = RequestsStartMarker::new();
#[link_section = ".limine_requests"]
pub static BASE_REVISION: BaseRevision = BaseRevision::new();
#[link_section = ".limine_requests"]
pub static MEMMAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();
#[link_section = ".limine_requests"]
pub static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();
#[link_section = ".limine_requests"]
pub static KERNEL_ADDR_REQUEST: KernelAddressRequest = KernelAddressRequest::new();
#[link_section = ".limine_requests"]
pub static KERNEL_FILE_REQUEST: KernelFileRequest = KernelFileRequest::new();
#[link_section = ".limine_requests"]
pub static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();
#[link_section = ".limine_requests"]
pub static MODULE_REQUEST: ModuleRequest = ModuleRequest::new();
#[link_section = ".limine_requests"]
pub static SMP_REQUEST: SmpRequest = SmpRequest::new();
#[cfg(feature = "fw_acpi")]
#[link_section = ".limine_requests"]
pub static RSDP_REQUEST: RsdpRequest = RsdpRequest::new();
#[cfg(feature = "fw_open_firmware")]
#[link_section = ".limine_requests"]
pub static DTB_REQUEST: DeviceTreeBlobRequest = DeviceTreeBlobRequest::new();
#[link_section = ".requests_end_marker"]
pub static END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

/// This is the absolute entry point of menix.
#[no_mangle]
unsafe extern "C" fn kernel_boot() -> ! {
    unsafe {
        let mut info = BootInfo::default();

        // Get kernel physical and virtual base.
        let kernel_addr = KERNEL_ADDR_REQUEST.get_response().unwrap();
        info.kernel_addr = (kernel_addr.physical_base(), kernel_addr.virtual_base());

        // Convert the memory map. This buffer has to be fixed since at this point
        // in the boot process there is no dynamic memory allocator available yet.
        let mut memmap_buf: [PhysMemory; 256] = [PhysMemory::new(); 256];
        for (i, entry) in MEMMAP_REQUEST
            .get_response()
            .unwrap()
            .entries()
            .iter()
            .enumerate()
        {
            memmap_buf[i] = PhysMemory {
                address: entry.base as VirtAddr,
                length: entry.length as usize,
                usage: match entry.entry_type {
                    _ => Unknown,
                },
            };
        }
        info.memory_map = &memmap_buf;

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
