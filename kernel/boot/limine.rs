use core::str;

use super::BootInfo;
use crate::arch::Arch;
use limine::{request::*, BaseRevision};

#[no_mangle]
unsafe extern "C" fn kernel_boot() -> ! {
    {
        let mut info = BootInfo::default();

        // Get command line.
        info.command_line = match KERNEL_FILE_REQUEST.get_response() {
            Some(x) => {
                // Convert the command line from bytes to UTF-8.
                let cmdline_utf8 = match str::from_utf8(x.file().cmdline()) {
                    Ok(s) => s,
                    Err(_) => todo!(),
                };
                Some(cmdline_utf8)
            }
            None => None,
        };

        Arch::early_init(&info);
        Arch::init(&info);
    }
    loop {}
}

#[used]
#[link_section = ".requests_start_marker"]
static _START_MARKER: RequestsStartMarker = RequestsStartMarker::new();

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
static _END_MARKER: RequestsEndMarker = RequestsEndMarker::new();
