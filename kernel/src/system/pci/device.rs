use crate::{
    generic::util::mutex::spin::SpinMutex,
    system::pci::{
        config::{self, Access, Address},
        driver::Driver,
    },
};
use alloc::vec::Vec;

pub struct Device {
    pub address: Address,
    /// The driver currently bound to this device.
    pub driver: Option<&'static Driver>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PciBar {
    Mmio32 {
        address: u32,
        size: usize,
        prefetchable: bool,
    },
    Mmio64 {
        address: u64,
        size: usize,
        prefetchable: bool,
    },
    Io {
        address: u16,
        size: usize,
    },
}

impl PciBar {
    fn is_valid(&self) -> bool {
        match self {
            PciBar::Mmio32 { address, .. } => *address != 0,
            PciBar::Mmio64 { address, .. } => *address != 0,
            PciBar::Io { address, .. } => *address != 0,
        }
    }
}

impl Device {
    pub fn bar(&self, access: &dyn Access, index: usize) -> Option<PciBar> {
        let bar_offset = config::generic::BAR0.offset() + index * size_of::<u32>();
        let bar = access.read32(self.address, bar_offset as u32);

        let is_mmio = bar & 0x1 == 0x0;
        let is_mmio64 = is_mmio && ((bar >> 1) & 0x3) == 0x2;
        let is_prefetchable = is_mmio && (bar & (1 << 3) != 0);

        let command_register =
            access.read16(self.address, config::common::COMMAND.byte_offset() as u32);

        // Disable IO and memory decoding while probing BAR sizes.
        access.write16(
            self.address,
            config::common::COMMAND.byte_offset() as u32,
            command_register & !(1 << 0 | 1 << 1),
        );

        // Probe the size of this BAR.
        access.write32(self.address, bar_offset as u32, 0xFFFF_FFFF);

        let new_bar = access.read32(self.address, bar_offset as u32);

        // Restore the original BAR value.
        access.write32(self.address, bar_offset as u32, bar);

        let kind = if is_mmio {
            let address = (bar & 0xFFFF_FFF0) as usize;
            let size = (!(new_bar & 0xFFFF_FFF0) + 1) as usize;

            if is_mmio64 {
                assert!(index + 1 < 6);

                let next_bar_offset = bar_offset + size_of::<u32>();
                let next_bar = access.read32(self.address, next_bar_offset as u32);

                PciBar::Mmio64 {
                    address: (next_bar as u64) << 32 | (address as u64),
                    size,
                    prefetchable: is_prefetchable,
                }
            } else {
                PciBar::Mmio32 {
                    address: address as u32,
                    size,
                    prefetchable: is_prefetchable,
                }
            }
        } else {
            let address = (bar & 0x0000_FFF0) as usize;
            let size = (!(new_bar & 0xFFFF_FFFC) + 1) as usize;

            PciBar::Io {
                address: address as u16,
                size,
            }
        };

        // Restore the command register.
        access.write16(
            self.address,
            config::common::COMMAND.byte_offset() as u32,
            command_register,
        );

        kind.is_valid().then_some(kind)
    }
}

pub static DEVICES: SpinMutex<Vec<Device>> = SpinMutex::new(Vec::new());
