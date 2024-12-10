use super::{BootInfo, EarlyBootInfo};
use crate::{
    arch::{Arch, CommonArch, PhysAddr, VirtAddr},
    memory::pm::{PhysMemory, PhysMemoryUsage},
    misc::units,
};
use core::str;
use limine::{memory_map::EntryType, request::*, BaseRevision};

#[used]
#[link_section = ".boot.init"]
pub static START_MARKER: RequestsStartMarker = RequestsStartMarker::new();

#[used]
#[link_section = ".boot"]
pub static BASE_REVISION: BaseRevision = BaseRevision::new();

#[link_section = ".boot"]
pub static STACK_SIZE: StackSizeRequest =
    StackSizeRequest::with_size(StackSizeRequest::new(), 512 * units::KiB as u64);

#[link_section = ".boot"]
pub static MEMMAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();

#[link_section = ".boot"]
pub static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();

#[link_section = ".boot"]
pub static KERNEL_ADDR_REQUEST: KernelAddressRequest = KernelAddressRequest::new();

#[link_section = ".boot"]
pub static KERNEL_FILE_REQUEST: KernelFileRequest = KernelFileRequest::new();

#[link_section = ".boot"]
pub static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[link_section = ".boot"]
pub static MODULE_REQUEST: ModuleRequest = ModuleRequest::new();

#[link_section = ".boot"]
pub static SMP_REQUEST: SmpRequest = SmpRequest::new();

#[cfg(feature = "sys_acpi")]
#[link_section = ".boot"]
pub static RSDP_REQUEST: RsdpRequest = RsdpRequest::new();

#[cfg(feature = "sys_open_firmware")]
#[link_section = ".boot"]
pub static DTB_REQUEST: DeviceTreeBlobRequest = DeviceTreeBlobRequest::new();

#[used]
#[link_section = ".boot.fini"]
pub static END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

/// This is the absolute entry point of menix.
#[no_mangle]
unsafe extern "C" fn kernel_boot() -> ! {
    {
        // Check if requested stack size was respected by the bootloader.
        _ = STACK_SIZE.get_response().unwrap();

        let mut early_info = EarlyBootInfo::default();

        // Get kernel physical and virtual base.
        let kernel_addr = KERNEL_ADDR_REQUEST.get_response().unwrap();
        early_info.kernel_addr = (kernel_addr.physical_base(), kernel_addr.virtual_base());
        early_info.identity_base = HHDM_REQUEST.get_response().unwrap().offset() as VirtAddr;

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
        early_info.memory_map = &mut memmap_buf[0..entries.len()];

        // Convert the command line from bytes to UTF-8 if there is any.
        early_info.command_line = {
            let line_buf = KERNEL_FILE_REQUEST.get_response().unwrap().file().cmdline();
            match line_buf.len() {
                0 => None,
                1.. => Some(str::from_utf8(line_buf).expect("Command line was not valid UTF-8!")),
            }
        };

        #[cfg(feature = "sys_acpi")]
        {
            early_info.rsdp_addr = RSDP_REQUEST.get_response().unwrap().address() as PhysAddr;
        }

        // Initialize the physical allocator and serial output.
        Arch::early_init(&mut early_info);

        let mut info = BootInfo::default();

        // Finalize CPU initialization.
        Arch::init(&mut info);
    }

    // TODO: Invoke the scheduler.

    loop {}
}
