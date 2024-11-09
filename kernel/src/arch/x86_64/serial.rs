// Serial I/O

use super::asm::write8;
use crate::{
    dbg,
    fs::{fd::FileDescriptor, handle::Handle},
    log,
    misc::log::STDOUT,
    system::error::Errno,
};
use alloc::boxed::Box;

/// Serial port
pub const COM1_BASE: u16 = 0x3F8;
/// Data Register
pub const DATA_REG: u16 = 0;
/// Interrupt Enable Register
pub const INT_ENABLE_REG: u16 = 1;
/// Divisor Latch LSB
pub const DIV_LSB: u16 = 0;
/// Divisor Latch MSB
pub const DIV_MSB: u16 = 1;
/// Interrupt Identification and FIFO Control Register
pub const INT_ID_FIFO_CTRL_REG: u16 = 2;
/// Line Control Register
pub const LINE_CTRL_REG: u16 = 3;
/// Modem Control Register
pub const MODEM_CTRL_REG: u16 = 4;
/// Line Status Register
pub const LINE_STATUS_REG: u16 = 5;

struct SerialHandle;
impl Handle for SerialHandle {
    fn read(
        &mut self,
        fd: Option<&FileDescriptor>,
        output: &mut [u8],
        offset: usize,
    ) -> Result<usize, Errno> {
        todo!()
    }

    fn write(
        &mut self,
        fd: Option<&FileDescriptor>,
        input: &[u8],
        offset: usize,
    ) -> Result<usize, Errno> {
        for ch in input {
            // TODO: This can be done better.
            unsafe { write8(COM1_BASE + DATA_REG, *ch) };
        }
        Ok(input.len())
    }

    fn ioctl(
        &mut self,
        fd: Option<&FileDescriptor>,
        request: u32,
        argument: usize,
    ) -> Result<usize, Errno> {
        Ok((0))
    }
}

pub fn init() {
    STDOUT.lock().live = Some(Box::new(SerialHandle));
    log!("serial: Initialized early serial output.\n");
}
