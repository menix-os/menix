// Serial I/O

use super::asm::write8;
use crate::generic::log::{Logger, LoggerSink};
use alloc::boxed::Box;

/// Serial port
pub const COM1_BASE: u16 = 0x3F8;
/// Data Register
pub const DATA_REG: u16 = 0;

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
