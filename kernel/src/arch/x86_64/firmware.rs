use super::{PhysAddr, VirtAddr};
use crate::generic::{
    clock::{ClockError, ClockSource},
    memory::virt::{KERNEL_PAGE_TABLE, VmFlags},
};
use core::{mem::offset_of, ptr::NonNull};
use uacpi::{acpi_hpet, acpi_sdt_hdr, uacpi_status_UACPI_STATUS_OK, uacpi_table};

// TODO: Use IoSpace
#[repr(C, packed)]
struct HpetRegisters {
    capabilities: u64,
    _pad0: u64,
    configuration: u64,
    _pad1: u64,
    interrupt_status: u64,
    _pad2: [u64; 0x19],
    main_counter: u64,
    _pad3: u64,
}

#[derive(Default)]
pub struct Hpet {
    regs: Option<*mut u64>, // TODO: Use IoSpace
    period: u32,
}

unsafe impl Send for Hpet {}
unsafe impl Sync for Hpet {}

impl ClockSource for Hpet {
    fn name(&self) -> &'static str {
        "hpet"
    }

    fn reset(&mut self) {
        if let Some(x) = self.regs {
            unsafe {
                x.byte_add(offset_of!(HpetRegisters, main_counter))
                    .write_volatile(0)
            };
        }
    }

    fn get_priority(&self) -> u8 {
        // Always prefer the HPET if we have it.
        255
    }

    fn get_elapsed_ns(&self) -> usize {
        return match self.regs {
            Some(x) => unsafe {
                (x.byte_add(offset_of!(HpetRegisters, main_counter))
                    .read_volatile()
                    * self.period as u64
                    / 1_000_000) as usize
            },
            None => 0,
        };
    }

    fn setup(&mut self) -> Result<(), ClockError> {
        let mut table = uacpi_table::default();

        if unsafe { uacpi::uacpi_table_find_by_signature(c"HPET".as_ptr(), &raw mut table) }
            != uacpi_status_UACPI_STATUS_OK
        {
            return Err(ClockError::Unavailable);
        }

        let hpet: *mut acpi_hpet = unsafe { table.__bindgen_anon_1.ptr } as *mut acpi_hpet;
        self.regs = Some(KERNEL_PAGE_TABLE.write().map_memory(
            (unsafe { *hpet }).address.address as PhysAddr,
            VmFlags::Read | VmFlags::Write,
            0,
            size_of::<HpetRegisters>(),
        ) as *mut u64);

        match self.regs {
            Some(x) => unsafe {
                self.period = (x
                    .byte_add(offset_of!(HpetRegisters, capabilities))
                    .read_volatile()
                    >> 32) as u32;
                let cfg = x.byte_add(offset_of!(HpetRegisters, configuration));
                cfg.write_volatile(cfg.read_volatile() | 1);
            },
            None => return Err(ClockError::UnableToSetup),
        }

        unsafe { uacpi::uacpi_table_unref(&raw mut table) };

        return Ok(());
    }
}
