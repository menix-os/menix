// Serial I/O

use super::asm::write8;
use crate::generic::log::{GLOBAL_LOGGERS, Logger, LoggerSink};
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

struct SerialLogger;
impl LoggerSink for SerialLogger {
    fn write(&mut self, input: &[u8]) {
        for ch in input {
            unsafe { write8(COM1_BASE + DATA_REG, *ch) };
        }
    }

    fn name(&self) -> &'static str {
        "com1"
    }
}

pub fn init() {
    Logger::add_sink(Box::new(SerialLogger));
}
