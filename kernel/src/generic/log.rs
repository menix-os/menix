//! Message logs from the kernel.
// TODO: Convert to struct Console

use alloc::{boxed::Box, vec::Vec};
use core::fmt;
use spin::Mutex;

/// A sink to write logs to.
pub trait LoggerSink: Send {
    fn name(&self) -> &'static str;

    /// Writes a buffer to the logger.
    fn write(&mut self, input: &[u8]);
}

pub struct Logger {
    pub sinks: [Option<Box<dyn LoggerSink>>; 16],
}

impl Logger {
    /// Adds a sink to the logger.
    pub fn add_sink(sink: Box<dyn LoggerSink>) {
        let name = sink.name();
        {
            let mut logger = GLOBAL_LOGGERS.lock();
            for s in &mut logger.sinks {
                match s {
                    Some(_) => continue,
                    None => {
                        *s = Some(sink);
                        break;
                    }
                }
            }
        }
        log!("log: Registered new logging sink \"{}\"", name);
    }

    pub fn remove_sink(name: &str) {
        let mut logger = GLOBAL_LOGGERS.lock();
        for sink in &mut logger.sinks {
            if let Some(x) = sink {
                if x.name() == name {
                    *sink = None;
                }
            }
        }
    }
}

const MAX_LOGGERS: usize = 16;

/// The global registry for loggers.
pub static GLOBAL_LOGGERS: Mutex<Logger> = Mutex::new(Logger {
    sinks: [const { None }; MAX_LOGGERS],
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

/// Global in-memory logger.
static KERNEL_LOGGER: Mutex<Vec<u8>> = Mutex::new(Vec::new());

/// Primitive Logger sink for the kernel.
pub struct KernelLogger;
impl LoggerSink for KernelLogger {
    fn name(&self) -> &'static str {
        "kernel"
    }

    fn write(&mut self, input: &[u8]) {
        let mut logger = KERNEL_LOGGER.lock();
        logger.extend_from_slice(input);
    }
}

impl KernelLogger {
    /// Copies the current log into a buffer.
    pub fn get_log() -> Vec<u8> {
        return KERNEL_LOGGER.lock().clone();
    }
}
