// Boot using the Limine protocol.

use super::{BootFile, BootInfo, PhysMemory, PhysMemoryUsage};
use crate::{
    arch,
    generic::{
        boot::fbcon::{FbColorBits, FrameBuffer},
        cmdline::CmdLine,
        memory::{PhysAddr, VirtAddr},
    },
};
use core::ptr::slice_from_raw_parts;
use limine::{BaseRevision, memory_map::EntryType, paging::Mode, request::*};

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
pub static PAGING_REQUEST: PagingModeRequest = PagingModeRequest::new();

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

static mut MEMMAP_BUF: [PhysMemory; 128] = [PhysMemory::new(); 128];
static mut FILE_BUF: [BootFile; 128] = [BootFile::new(); 128];

#[unsafe(no_mangle)]
extern "C" fn _start() -> ! {
    let mut info = BootInfo::new();

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
        // in the boot process since there is no dynamic memory allocator available yet.
        // 128 entries should be enough for all use cases.
        let entries = MEMMAP_REQUEST.get_response().unwrap().entries();
        for (i, entry) in entries.iter().enumerate() {
            unsafe {
                MEMMAP_BUF[i] = PhysMemory {
                    length: entry.length as usize,
                    usage: match entry.entry_type {
                        EntryType::USABLE => PhysMemoryUsage::Free,
                        EntryType::BOOTLOADER_RECLAIMABLE => PhysMemoryUsage::Reclaimable,
                        EntryType::RESERVED => PhysMemoryUsage::Reserved,
                        EntryType::FRAMEBUFFER => PhysMemoryUsage::Reserved,
                        EntryType::EXECUTABLE_AND_MODULES => PhysMemoryUsage::Kernel,
                        _ => PhysMemoryUsage::Unknown,
                    },
                    address: PhysAddr(entry.base as usize),
                };
            }
        }

        // Get kernel physical and virtual base.
        let kernel_addr = KERNEL_ADDR_REQUEST.get_response().unwrap();

        info.hhdm_address = Some(VirtAddr(
            HHDM_REQUEST.get_response().unwrap().offset() as usize
        ));

        let paging = PAGING_REQUEST.get_response().unwrap().mode();
        #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
        {
            if paging == Mode::FOUR_LEVEL {
                info.paging_level = Some(4);
            } else if paging == Mode::FIVE_LEVEL {
                info.paging_level = Some(5);
            }
        }
        #[cfg(target_arch = "riscv64")]
        {
            if paging == Mode::SV39 {
                info.paging_level = Some(3);
            } else if paging == Mode::SV48 {
                info.paging_level = Some(4);
            } else if paging == Mode::SV57 {
                info.paging_level = Some(5);
            }
        }
        #[cfg(target_arch = "loongarch64")]
        {
            if paging == Mode::FOUR_LEVEL {
                info.paging_level = Some(4);
            }
        }

        unsafe {
            info.memory_map = &MEMMAP_BUF[0..entries.len()];
        }
        info.kernel_phys = Some(PhysAddr(kernel_addr.physical_base() as usize));
        info.kernel_virt = Some(VirtAddr(kernel_addr.virtual_base() as usize));
    }

    // Convert the command line from bytes to UTF-8 if there is any.
    info.command_line = {
        let line_buf = COMMAND_LINE_REQUEST.get_response().unwrap().cmdline();
        CmdLine::new(line_buf.to_str().unwrap_or_default())
    };

    // The RSDP is a physical address.
    info.rsdp_addr = RSDP_REQUEST.get_response().map(|x| PhysAddr(x.address()));

    // The FDT is a virtual address.
    info.fdt_addr = DTB_REQUEST.get_response().map(|x| {
        PhysAddr(unsafe {
            x.dtb_ptr()
                .byte_sub(HHDM_REQUEST.get_response().unwrap().offset() as usize)
        } as usize)
    });

    // Get all modules.
    if let Some(response) = MODULE_REQUEST.get_response() {
        for (i, entry) in response.modules().iter().enumerate() {
            unsafe {
                FILE_BUF[i] = BootFile {
                    data: slice_from_raw_parts(entry.addr(), entry.size() as usize)
                        .as_ref()
                        .unwrap(),
                    // Split off any parts of the path that come before the actual file name.
                    name: entry.path().to_str().unwrap().rsplit_once('/').unwrap().1,
                    command_line: CmdLine::new(
                        entry
                            .string()
                            .to_str()
                            .expect("Module command line was not valid UTF-8"),
                    ),
                }
            };
        }
        unsafe {
            info.files = &FILE_BUF[0..response.modules().len()];
        }
    }

    if let Some(response) = FRAMEBUFFER_REQUEST.get_response() {
        if let Some(fb) = response.framebuffers().next() {
            info.framebuffer = Some(FrameBuffer {
                base: PhysAddr(fb.addr() as usize - info.hhdm_address.unwrap().0),
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

    // Call the kernel common entry point.
    crate::main();
}
