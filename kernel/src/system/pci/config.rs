use crate::generic::util::{align_down, once::Once};
use alloc::{boxed::Box, vec::Vec};
use core::fmt::Display;

pub mod common {
    use crate::generic::memory::view::{Field, Register};

    pub const REG0: Register<u32> = Register::new(0x00).with_le();
    pub const VENDOR_ID: Field<u32, u16> = Field::new(REG0, 0);
    pub const DEVICE_ID: Field<u32, u16> = Field::new(REG0, 2);

    pub const REG1: Register<u32> = Register::new(0x04).with_le();
    pub const COMMAND: Field<u32, u16> = Field::new(REG1, 0);
    pub const STATUS: Field<u32, u16> = Field::new(REG1, 2);

    pub const REG2: Register<u32> = Register::new(0x08).with_le();
    pub const PROG_IF: Field<u32, u8> = Field::new(REG2, 0x01);
    pub const SUB_CLASS: Field<u32, u8> = Field::new(REG2, 0x02);
    pub const CLASS_CODE: Field<u32, u8> = Field::new(REG2, 0x03);

    pub const REG3: Register<u32> = Register::new(0x0C).with_le();
}

pub mod generic {
    use crate::generic::memory::view::{Field, Register};

    pub const BAR0: Register<u32> = Register::new(0x10).with_le();
    pub const BAR1: Register<u32> = Register::new(0x14).with_le();
    pub const BAR2: Register<u32> = Register::new(0x18).with_le();
    pub const BAR3: Register<u32> = Register::new(0x1C).with_le();
    pub const BAR4: Register<u32> = Register::new(0x20).with_le();
    pub const BAR5: Register<u32> = Register::new(0x24).with_le();

    pub const CARDBUS_CIS_PTR: Register<u32> = Register::new(0x28).with_le();

    pub const REG11: Register<u32> = Register::new(0x2C).with_le();

    pub const EXPANSION_ROM: Register<u32> = Register::new(0x30).with_le();

    pub const REG13: Register<u32> = Register::new(0x34).with_le();
    pub const CAPABILITIES_PTR: Field<u32, u8> = Field::new(REG11, 0);

    pub const REG14: Register<u32> = Register::new(0x3C).with_le();
    pub const INTERRUPT_LINE: Field<u32, u8> = Field::new(REG14, 0);
    pub const INTERRUPT_PIN: Field<u32, u8> = Field::new(REG14, 1);
    pub const MIN_GRANT: Field<u32, u8> = Field::new(REG14, 2);
    pub const MAX_LATENCY: Field<u32, u8> = Field::new(REG14, 3);
}

#[derive(Copy, Clone, Debug)]
pub struct Address {
    pub segment: u16,
    pub bus: u8,
    pub slot: u8,
    pub function: u8,
}

impl Display for Address {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!(
            "{:04x}:{:02x}:{:02x}.{:02x}",
            self.segment, self.bus, self.slot, self.function
        ))
    }
}

pub trait Access {
    fn segment(&self) -> u16;
    fn start_bus(&self) -> u8;
    fn end_bus(&self) -> u8;
    fn read32(&self, addr: Address, offset: u32) -> u32;
    fn write32(&self, addr: Address, offset: u32, value: u32);
}

impl dyn Access + '_ {
    /// Returns true if this [`Access`] contains the device as addressed by `address`.
    pub fn decodes(&self, address: Address) -> bool {
        self.segment() == address.segment
            && address.bus >= self.start_bus()
            && address.bus <= self.end_bus()
    }

    pub fn read8(&self, addr: Address, offset: u32) -> u8 {
        let aligned = align_down(offset, size_of::<u32>() as u32);
        let reg = self.read32(addr, aligned);
        (reg >> ((offset - aligned) * 8)) as u8
    }

    pub fn read16(&self, addr: Address, offset: u32) -> u16 {
        assert!(offset % 2 == 0);
        let aligned = align_down(offset, size_of::<u32>() as u32);
        let reg = self.read32(addr, aligned);
        (reg >> ((offset - aligned) * 8)) as u16
    }

    pub fn write8(&self, addr: Address, offset: u32, value: u8) {
        let aligned = align_down(offset, size_of::<u32>() as u32);
        let mut reg = self.read32(addr, aligned);
        reg &= !(0xFF << ((offset - aligned) * 8));
        reg |= (value as u32) << ((offset - aligned) * 8);
        self.write32(addr, aligned, reg);
    }

    pub fn write16(&self, addr: Address, offset: u32, value: u16) {
        assert!(offset % 2 == 0);
        let aligned = align_down(offset, size_of::<u32>() as u32);
        let mut reg = self.read32(addr, aligned);
        reg &= !(0xFFFF << ((offset - aligned) * 8));
        reg |= (value as u32) << ((offset - aligned) * 8);
        self.write32(addr, aligned, reg);
    }
}

pub struct EcamPciAccess {
    pub segment: u16,
    pub start_bus: u8,
    pub end_bus: u8,
    pub base: *mut u32,
}

impl Access for EcamPciAccess {
    fn segment(&self) -> u16 {
        self.segment
    }

    fn start_bus(&self) -> u8 {
        self.start_bus
    }

    fn end_bus(&self) -> u8 {
        self.end_bus
    }

    fn read32(&self, addr: Address, offset: u32) -> u32 {
        unsafe {
            self.base
                .byte_add(
                    (addr.bus as usize) << 20
                        | (addr.slot as usize) << 15
                        | (addr.function as usize) << 12
                        | offset as usize & 0xFFF,
                )
                .read_volatile()
        }
    }

    fn write32(&self, addr: Address, offset: u32, value: u32) {
        unsafe {
            self.base
                .byte_add(
                    (addr.bus as usize) << 20
                        | (addr.slot as usize) << 15
                        | (addr.function as usize) << 12
                        | offset as usize & 0xFFF,
                )
                .write_volatile(value)
        }
    }
}

pub static ACCESS: Once<Vec<Box<dyn Access>>> = Once::new();
