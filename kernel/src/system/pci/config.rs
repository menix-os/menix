#![allow(unused)]

use crate::{
    generic::{
        memory::PhysAddr,
        util::{align_down, once::Once},
    },
    system::pci::device::PciDevice,
};
use alloc::vec::Vec;
use uacpi_sys::{UACPI_STATUS_OK, uacpi_table, uacpi_table_find_by_signature};

pub mod common {
    use crate::generic::memory::mmio::{Field, Register};

    pub const REG0: Register<u32> = Register::new(0x00).with_le();
    pub const VENDOR_ID: Field<u32, u16> = Field::new(REG0, 0);
    pub const DEVICE_ID: Field<u32, u16> = Field::new(REG0, 2);

    pub const REG1: Register<u32> = Register::new(0x04).with_le();
    pub const COMMAND: Field<u32, u16> = Field::new(REG1, 0);
    pub const STATUS: Field<u32, u16> = Field::new(REG1, 2);

    pub const REG2: Register<u32> = Register::new(0x08).with_le();

    pub const REG3: Register<u32> = Register::new(0x0C).with_le();
}

pub mod generic {
    use crate::generic::memory::mmio::{Field, Register};

    pub const BAR0: Register<u32> = Register::new(0x10).with_le();
    pub const BAR1: Register<u32> = Register::new(0x14).with_le();
    pub const BAR2: Register<u32> = Register::new(0x18).with_le();
    pub const BAR3: Register<u32> = Register::new(0x1C).with_le();
    pub const BAR4: Register<u32> = Register::new(0x20).with_le();
    pub const BAR5: Register<u32> = Register::new(0x24).with_le();

    pub const CARDBUS_CIS_PTR: Register<u32> = Register::new(0x28).with_le();

    pub const REG11: Register<u32> = Register::new(0x2C).with_le();
    pub const SUBSYSTEM_VENDOR_ID: Field<u32, u16> = Field::new(REG11, 0);
    pub const SUBSYSTEM_ID: Field<u32, u16> = Field::new(REG11, 2);

    pub const EXPANSION_ROM: Register<u32> = Register::new(0x30).with_le();

    pub const REG13: Register<u32> = Register::new(0x34).with_le();
    pub const CAPABILITIES_PTR: Field<u32, u8> = Field::new(REG11, 0);

    pub const REG14: Register<u32> = Register::new(0x3C).with_le();
    pub const INTERRUPT_LINE: Field<u32, u8> = Field::new(REG14, 0);
    pub const INTERRUPT_PIN: Field<u32, u8> = Field::new(REG14, 1);
    pub const MIN_GRANT: Field<u32, u8> = Field::new(REG14, 2);
    pub const MAX_LATENCY: Field<u32, u8> = Field::new(REG14, 3);
}

#[derive(Copy, Clone)]
pub struct PciAddress {
    pub segment: u16,
    pub bus: u8,
    pub slot: u8,
    pub function: u8,
}

pub trait PciAccess {
    fn read32(&self, addr: PciAddress, offset: u32) -> u32;
    fn write32(&self, addr: PciAddress, offset: u32, value: u32);
}

impl dyn PciAccess {
    fn read8(&self, addr: PciAddress, offset: u32) -> u8 {
        let aligned = align_down(offset, size_of::<u32>() as u32);
        let reg = self.read32(addr, aligned);
        (reg >> ((offset - aligned) * 8)) as u8
    }

    fn read16(&self, addr: PciAddress, offset: u32) -> u16 {
        assert!(offset % 2 == 0);
        let aligned = align_down(offset, size_of::<u32>() as u32);
        let reg = self.read32(addr, aligned);
        (reg >> ((offset - aligned) * 8)) as u16
    }

    fn write8(&self, addr: PciAddress, offset: u32, value: u8) {
        let aligned = align_down(offset, size_of::<u32>() as u32);
        let mut reg = self.read32(addr, aligned);
        reg &= !(0xFF << ((offset - aligned) * 8));
        reg |= (value as u32) << ((offset - aligned) * 8);
        self.write32(addr, aligned, reg);
    }

    fn write16(&self, addr: PciAddress, offset: u32, value: u16) {
        assert!(offset % 2 == 0);
        let aligned = align_down(offset, size_of::<u32>() as u32);
        let mut reg = self.read32(addr, aligned);
        reg &= !(0xFFFF << ((offset - aligned) * 8));
        reg |= (value as u32) << ((offset - aligned) * 8);
        self.write32(addr, aligned, reg);
    }
}

pub static ACCESS: Once<&dyn PciAccess> = Once::new();

pub fn scan_config_space() -> Vec<PciDevice> {
    todo!()
}
