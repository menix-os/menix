use core::fmt;

use alloc::{boxed::Box, vec::Vec};
use spin::Mutex;

/// A sink to write logs to.
pub trait LoggerSink: Send {
    /// Writes a buffer to the logger.
    fn write(&mut self, input: &[u8]);
}

pub struct Logger {
    pub sinks: [Option<Box<dyn LoggerSink>>; 8],
}

impl Logger {
    /// Adds a sink to the logger.
    pub fn add_sink(&mut self, sink: Box<dyn LoggerSink>) {
        for s in &mut self.sinks {
            match s {
                Some(_) => continue,
                None => {
                    *s = Some(sink);
                    break;
                }
            }
        }
    }
}

/// The global registry for loggers.
pub static KERNEL_LOGGER: Mutex<Logger> = Mutex::new(Logger {
    sinks: [const { None }; 8],
});

impl fmt::Write for Logger {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for sink in &mut self.sinks {
            if let Some(x) = sink {
                x.write(s.as_bytes());
            }
        }
        Ok(())
    }
}
