#![no_std]

use menix::{
    generic::{
        memory::{
            pmm::{AllocFlags, KernelAlloc, PageAllocator},
            virt::{VmFlags, mmu::PageTable},
        },
        posix::errno::EResult,
    },
    log,
    system::pci::{
        device::Device,
        driver::{Driver, PciVariantBuilder},
    },
};

menix::module!("NVMe block devices", "Marvin Friedrich", main);

static DRIVER: Driver = Driver {
    name: "nvme",
    probe: probe,
    remove: None,
    suspend: None,
    resume: None,
    variants: &[PciVariantBuilder::new()
        .mass_storage_controller()
        .non_volatile_memory_controller()
        .nvm_express_controller()],
};

pub fn probe(dev: &Device) -> EResult<()> {
    log!("Probing NVMe device on {}", dev.address);

    Ok(())
}

pub fn main() {
    _ = DRIVER.register();

    let mem = KernelAlloc::alloc_bytes(0x1000, AllocFlags::Zeroed).unwrap();
    PageTable::get_kernel()
        .map_memory::<KernelAlloc>(mem, VmFlags::Read | VmFlags::Write, 0x1000)
        .unwrap();
}
