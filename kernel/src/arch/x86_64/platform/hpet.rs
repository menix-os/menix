use crate::generic::{
    clock::{ClockError, ClockSource},
    memory::mmio::Mmio,
};
use uacpi_sys::{
    UACPI_STATUS_OK, acpi_hpet, uacpi_table, uacpi_table_find_by_signature, uacpi_table_unref,
};

pub struct Hpet {
    regs: Mmio,
    period: u32,
}

mod regs {
    use crate::generic::memory::mmio::Register;

    pub const CAPABILITIES: Register<u64> = Register::new(0);
    pub const CONFIGURATION: Register<u64> = Register::new(0x10);
    pub const MAIN_COUNTER: Register<u64> = Register::new(0xF0);
}

impl ClockSource for Hpet {
    fn name(&self) -> &'static str {
        "hpet"
    }

    fn get_priority(&self) -> u8 {
        75
    }

    fn reset(&mut self) {
        self.regs.write(regs::MAIN_COUNTER, 0 as u64);
    }

    fn get_elapsed_ns(&self) -> usize {
        return (self.regs.read(regs::MAIN_COUNTER) * self.period as u64 / 1_000_000) as usize
            as usize;
    }
}

impl Hpet {
    pub fn new() -> Result<Self, ClockError> {
        let mut table = uacpi_table::default();
        let uacpi_status =
            unsafe { uacpi_table_find_by_signature(c"HPET".as_ptr(), &raw mut table) };

        if uacpi_status != UACPI_STATUS_OK {
            return Err(ClockError::Unavailable);
        }

        let hpet: *mut acpi_hpet = unsafe { table.__bindgen_anon_1.ptr } as *mut acpi_hpet;
        let mut mmio = unsafe { Mmio::new_mmio(((*hpet).address.address as usize).into(), 0x1000) };
        unsafe { uacpi_table_unref(&raw mut table) };

        // Enable timer.
        mmio.write(regs::CONFIGURATION, mmio.read(regs::CONFIGURATION) | 1);
        let period = (mmio.read(regs::CAPABILITIES) >> 32) as u32;

        return Ok(Hpet {
            regs: mmio,
            period: period,
        });
    }
}
