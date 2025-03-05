// Boot using the Limine protocol.

#![no_std]
#![no_main]

use super::{BootFile, BootInfo, init};
use crate::{
    arch::{PhysAddr, VirtAddr},
    generic::phys::{PhysMemory, PhysMemoryUsage},
};
use core::str;
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
pub static BOOTLODAER_REQUEST: BootloaderInfoRequest = BootloaderInfoRequest::new();

#[unsafe(link_section = ".boot")]
pub static STACK_SIZE: StackSizeRequest = StackSizeRequest::new().with_size(0x20000); // We want 128 KiB of stack.

#[unsafe(link_section = ".boot")]
pub static MEMMAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();

#[unsafe(link_section = ".boot")]
pub static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();

#[unsafe(link_section = ".boot")]
pub static KERNEL_ADDR_REQUEST: KernelAddressRequest = KernelAddressRequest::new();

#[unsafe(link_section = ".boot")]
pub static KERNEL_FILE_REQUEST: KernelFileRequest = KernelFileRequest::new();

#[unsafe(link_section = ".boot")]
pub static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[unsafe(link_section = ".boot")]
pub static MODULE_REQUEST: ModuleRequest = ModuleRequest::new();

#[unsafe(link_section = ".boot")]
pub static RSDP_REQUEST: RsdpRequest = RsdpRequest::new();

#[unsafe(no_mangle)]
unsafe extern "C" fn _start() -> ! {
    init::early_init();

    // Make sure the stack size request was respected by the bootloader.
    // We might not be able to boot otherwise.
    _ = STACK_SIZE
        .get_response()
        .expect("Unable to boot without enough stack memory!");

    let mut info = BootInfo::default();

    match BOOTLODAER_REQUEST.get_response() {
        Some(x) => print!("boot: Loaded by {} {}\n", x.name(), x.version()),
        None => {}
    };

    // Convert the memory map. This buffer has to be fixed since at this point
    // in the boot process there is no dynamic memory allocator available yet.
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
            EntryType::KERNEL_AND_MODULES => PhysMemoryUsage::Kernel,
            _ => PhysMemoryUsage::Unknown,
        };
    }

    // Get kernel physical and virtual base.
    let kernel_addr = KERNEL_ADDR_REQUEST.get_response().unwrap();

    super::init::memory_init(
        &mut memmap_buf[0..entries.len()],
        HHDM_REQUEST.get_response().unwrap().offset() as VirtAddr,
        (kernel_addr.physical_base(), kernel_addr.virtual_base()),
    );

    // Convert the command line from bytes to UTF-8 if there is any.
    info.command_line = {
        let line_buf = KERNEL_FILE_REQUEST.get_response().unwrap().file().cmdline();
        match line_buf.len() {
            0 => None,
            1.. => Some(str::from_utf8(line_buf).expect("Command line was not valid UTF-8!")),
        }
    };

    info.rsdp_addr = match RSDP_REQUEST.get_response() {
        Some(x) => x.address() as PhysAddr,
        None => {
            panic!("No RSDP provided! Can't configure firmware!")
        }
    };

    // Get all modules.
    let mut file_buf = [BootFile::default(); 32];

    super::init::init(&mut info);

    loop {}
}
