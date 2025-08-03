//! Message logs from the kernel.
// TODO: Convert to struct Console

use super::util::mutex::spin::SpinMutex;
use alloc::{boxed::Box, vec::Vec};
use core::{
    fmt,
    sync::atomic::{AtomicUsize, Ordering},
};

/// A sink to write logs to.
pub trait LoggerSink: Send {
    fn name(&self) -> &'static str;

    /// Writes a buffer to the logger.
    fn write(&mut self, input: &[u8]);
}

pub struct Logger {
    pub sinks: [Option<Box<dyn LoggerSink>>; 16],
}

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
    log!("Registered new logging sink \"{}\"", name);
}

/// Removes a named sink from the logger.
pub fn remove_sink(name: &str) {
    let mut logger = GLOBAL_LOGGERS.lock();
    for sink in &mut logger.sinks {
        if let Some(x) = sink
            && x.name() == name
        {
            *sink = None;
        }
    }
}

const MAX_LOGGERS: usize = 16;

/// The global registry for loggers.
pub static GLOBAL_LOGGERS: SpinMutex<Logger> = SpinMutex::new(Logger {
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

/// A static buffer to write early messages to.
static EARLY_BUFFER: SpinMutex<[u8; EARLY_BUFFER_LEN]> = SpinMutex::new([0; EARLY_BUFFER_LEN]);
/// The size of the early kernel buffer.
const EARLY_BUFFER_LEN: usize = 8192;
/// The current offset in this logger
static EARLY_BUFFER_ADDR: AtomicUsize = AtomicUsize::new(0);

/// Global in-memory logger.
static KERNEL_LOGGER: SpinMutex<Vec<u8>> = SpinMutex::new(Vec::new());

/// Primitive Logger sink for the kernel.
pub struct KernelLogger;
impl LoggerSink for KernelLogger {
    fn name(&self) -> &'static str {
        "kernel"
    }

    fn write(&mut self, input: &[u8]) {
        // As long as the message fits in the early buffer, use that.
        let offset = EARLY_BUFFER_ADDR.load(Ordering::Acquire);

        if offset + input.len() < EARLY_BUFFER_LEN {
            EARLY_BUFFER.lock()[offset..][..input.len()].copy_from_slice(input);
            EARLY_BUFFER_ADDR.fetch_add(input.len(), Ordering::Release);
        } else {
            let mut logger = KERNEL_LOGGER.lock();
            logger.extend_from_slice(input);
        }
    }
}

pub fn init() {
    add_sink(Box::new(KernelLogger));
}

pub fn get_kernel_log(target: &mut [u8]) {
    target.copy_from_slice(&EARLY_BUFFER.lock()[0..EARLY_BUFFER_ADDR.load(Ordering::Acquire)]);
}

// TODO: Broken
//early_init_call!(init);
