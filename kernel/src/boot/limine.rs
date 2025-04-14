// Boot using the Limine protocol.

#![no_std]
#![no_main]

use super::{BootFile, BootInfo};
use crate::{
    arch::{PhysAddr, VirtAddr},
    generic::{
        cmdline::CmdLine,
        fbcon::{FbColorBits, FrameBuffer},
        init,
        memory::{PhysMemory, PhysMemoryUsage},
    },
};
use alloc::{borrow::ToOwned, vec::Vec};
use core::ptr::slice_from_raw_parts;
use limine::{BaseRevision, memory_map::EntryType, request::*};

#[used]
#[unsafe(link_section = ".boot.init")]
pub static START_MARKER: RequestsStartMarker = RequestsStartMarker::new();

#[used]
#[unsafe(link_section = ".boot")]
pub static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[unsafe(link_section = ".boot.fini")]
pub static END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

#[unsafe(link_section = ".boot")]
pub static BOOTLOADER_REQUEST: BootloaderInfoRequest = BootloaderInfoRequest::new();

#[unsafe(link_section = ".boot")]
pub static STACK_SIZE: StackSizeRequest = StackSizeRequest::new().with_size(256 * 1024); // We want 256 KiB of stack.

#[unsafe(link_section = ".boot")]
pub static MEMMAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();

#[unsafe(link_section = ".boot")]
pub static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();

#[unsafe(link_section = ".boot")]
pub static KERNEL_ADDR_REQUEST: ExecutableAddressRequest = ExecutableAddressRequest::new();

#[unsafe(link_section = ".boot")]
pub static COMMAND_LINE_REQUEST: ExecutableCmdlineRequest = ExecutableCmdlineRequest::new();

#[unsafe(link_section = ".boot")]
pub static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[unsafe(link_section = ".boot")]
pub static MODULE_REQUEST: ModuleRequest = ModuleRequest::new();

#[unsafe(link_section = ".boot")]
pub static RSDP_REQUEST: RsdpRequest = RsdpRequest::new();

#[unsafe(link_section = ".boot")]
pub static DTB_REQUEST: DeviceTreeBlobRequest = DeviceTreeBlobRequest::new();

#[unsafe(no_mangle)]
unsafe extern "C" fn _start() -> ! {
    init::early_init();

    if let Some(x) = BOOTLOADER_REQUEST.get_response() {
        print!(
            "boot: Booting with Limine protocol, loaded by {} {}\n",
            x.name(),
            x.version()
        )
    };

    // Make sure the stack size request was respected by the bootloader.
    // That way we can be sure we have enough stack memory for our allocations.
    // We might not be able to boot otherwise.
    _ = STACK_SIZE
        .get_response()
        .expect("Unable to boot without enough stack memory");

    {
        // Convert the memory map. This buffer has to be fixed since at this point
        // in the boot process there is no dynamic memory allocator available yet.
        // 128 entries should be enough for all use cases.
        let mut memmap_buf = [PhysMemory::new(); 128];
        let entries = MEMMAP_REQUEST.get_response().unwrap().entries();
        for (i, entry) in entries.iter().enumerate() {
            let elem = memmap_buf.get_mut(i).unwrap();
            elem.address = entry.base as VirtAddr;
            elem.length = entry.length as usize;
            elem.usage = match entry.entry_type {
                EntryType::USABLE => PhysMemoryUsage::Free,
                EntryType::RESERVED => PhysMemoryUsage::Reserved,
                EntryType::FRAMEBUFFER => PhysMemoryUsage::Reserved,
                EntryType::EXECUTABLE_AND_MODULES => PhysMemoryUsage::Kernel,
                _ => PhysMemoryUsage::Unknown,
            };
        }

        // Get kernel physical and virtual base.
        let kernel_addr = KERNEL_ADDR_REQUEST.get_response().unwrap();

        init::memory_init(
            &mut memmap_buf[0..entries.len()],
            HHDM_REQUEST.get_response().unwrap().offset() as VirtAddr,
            kernel_addr.physical_base() as PhysAddr,
            kernel_addr.virtual_base() as VirtAddr,
        );
    }

    let mut info = BootInfo::default();

    // Convert the command line from bytes to UTF-8 if there is any.
    info.command_line = {
        let line_buf = COMMAND_LINE_REQUEST.get_response().unwrap().cmdline();
        line_buf.to_str().unwrap_or_default().to_owned()
    };

    // The RSDP is a physical address.
    info.rsdp_addr = RSDP_REQUEST.get_response().map(|x| x.address() as PhysAddr);

    // The FDT is a virtual address.
    info.fdt_addr = DTB_REQUEST.get_response().map(|x| {
        ((x.dtb_ptr() as VirtAddr) - HHDM_REQUEST.get_response().unwrap().offset() as VirtAddr)
            as PhysAddr
    });

    // Get all modules.
    let mut file_buf = Vec::new();
    if let Some(response) = MODULE_REQUEST.get_response() {
        for file in response.modules() {
            file_buf.push(BootFile {
                data: unsafe { slice_from_raw_parts(file.addr(), file.size() as usize).as_ref() }
                    .unwrap()
                    .to_owned()
                    .into(),
                // Split off any parts of the path that come before the actual file name.
                name: file
                    .path()
                    .to_str()
                    .unwrap()
                    .rsplit_once('/')
                    .unwrap()
                    .1
                    .to_owned(),
                command_line: file
                    .string()
                    .to_str()
                    .expect("Module command line was not valid UTF-8")
                    .to_owned(),
            });
        }
        info.files = file_buf;
    }

    if let Some(response) = FRAMEBUFFER_REQUEST.get_response() {
        if let Some(fb) = response.framebuffers().next() {
            info.frame_buffer = Some(FrameBuffer {
                screen: fb.addr(),
                width: fb.width() as usize,
                height: fb.height() as usize,
                pitch: fb.pitch() as usize,
                cpp: fb.bpp() as usize / 8,
                red: FbColorBits {
                    offset: fb.red_mask_shift(),
                    size: fb.red_mask_size(),
                },
                green: FbColorBits {
                    offset: fb.green_mask_shift(),
                    size: fb.green_mask_size(),
                },
                blue: FbColorBits {
                    offset: fb.blue_mask_shift(),
                    size: fb.blue_mask_size(),
                },
            });
        }
    }

    // Finally, save the boot information.
    info.register();
    init::init();

    unreachable!();
}
