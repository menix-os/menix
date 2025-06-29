use crate::{
    arch::x86_64::asm::{read8, write8},
    generic::log::{self, LoggerSink},
};
use alloc::boxed::Box;

/// Serial port
pub const COM1_BASE: u16 = 0x3F8;
/// Data Register
pub const DATA_REG: u16 = 0;

struct SerialLogger;

impl SerialLogger {
    fn is_ready() -> bool {
        return unsafe { read8(COM1_BASE + 5) } & 0x20 != 0;
    }
}

impl LoggerSink for SerialLogger {
    fn write(&mut self, input: &[u8]) {
        for ch in input {
            while !Self::is_ready() {
                core::hint::spin_loop();
            }

            unsafe { write8(COM1_BASE + DATA_REG, *ch) };

            // Most consoles expect a carriage return after a newline.
            if *ch == b'\n' {
                unsafe { write8(COM1_BASE + DATA_REG, b'\r') };
            }
        }
    }

    fn name(&self) -> &'static str {
        "com1"
    }
}

fn init() {
    unsafe {
        write8(COM1_BASE + 1, 0x00); // Disable interrupts
        write8(COM1_BASE + 3, 0x80); // Enable DLAB (set baud rate divisor)
        write8(COM1_BASE, 0x01); // Set divisor low byte (115200 baud if 1)
        write8(COM1_BASE + 1, 0x00); // Set divisor high byte
        write8(COM1_BASE + 3, 0x03); // 8 bits, no parity, one stop bit (8n1)
        write8(COM1_BASE + 2, 0xC7); // Enable FIFO, clear them, with 14-byte threshold
        write8(COM1_BASE + 4, 0x0B); // IRQs enabled, RTS/DSR set

        write8(COM1_BASE + 4, 0x1E); // Set to loopback mode.
        write8(COM1_BASE, 0xAE); // Send a test byte.

        // If we don't get the same value back, this serial port doesn't work.
        if read8(COM1_BASE) != 0xAE {
            return;
        }

        write8(COM1_BASE + 4, 0x0F); // Disable loopback mode.
    };

    log::add_sink(Box::new(SerialLogger));
}

init_stage! {
    #[entails(crate::arch::EARLY_INIT_STAGE)]
    SERIAL_STAGE: "arch.x86_64.serial" => init;
}
