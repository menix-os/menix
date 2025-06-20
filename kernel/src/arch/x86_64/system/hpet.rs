use crate::generic::{
    clock::{ClockError, ClockSource},
    memory::mmio::Mmio,
};
use alloc::boxed::Box;
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

        let hpet = unsafe { table.__bindgen_anon_1.ptr } as *const acpi_hpet;
        let mmio = unsafe { Mmio::new_mmio(((*hpet).address.address as usize).into(), 0x1000) };
        unsafe { uacpi_table_unref(&raw mut table) };

        // Enable timer.
        mmio.write(regs::CONFIGURATION, mmio.read(regs::CONFIGURATION) | 1);
        let period = (mmio.read(regs::CAPABILITIES) >> 32) as u32;

        return Ok(Hpet { regs: mmio, period });
    }
}

init_stage! {
    #[depends(crate::system::acpi::TABLES_STAGE)]
    #[entails(crate::generic::clock::CLOCK_STAGE)]
    HPET_STAGE: "arch.x86_64.hpet" => || {
        if let Ok(x) = Hpet::new() {
            _ = crate::generic::clock::switch(Box::new(x));
        }
    };
}
