// Kernel logging.

use crate::{
    arch::x86_64::{asm::write8, serial::COM1_BASE},
    fs::handle::Handle,
};
use alloc::{boxed::Box, rc::Rc, sync::Arc};
use core::{
    cell::{OnceCell, RefCell},
    fmt,
    iter::Once,
};
use spin::Mutex;

pub struct Writer {
    /// Callback for writing data to a live output.
    pub live: Option<Box<dyn Handle>>,

    /// Callback for writing to the kernel log file.
    /// This should be set as soon as VFS is initialized.
    pub file: Option<Box<dyn Handle>>,
}

/// Global reference to kernel standard output.
pub static STDOUT: Mutex<Writer> = Mutex::new(Writer {
    live: None,
    file: None,
});

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        match self
            .live
            .as_mut()
            .unwrap()
            .as_mut()
            .write(None, s.as_bytes(), 0)
        {
            Ok(_) => Ok(()),
            Err(_) => Err(fmt::Error),
        }
    }
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let mut writer = $crate::misc::log::STDOUT.lock();
		writer.write_fmt(format_args!($($arg)*)).unwrap();
    });
}

#[macro_export]
macro_rules! dbg {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let mut writer = $crate::misc::log::STDOUT.lock();
		writer.write_fmt(format_args!("{:#?}\n", $($arg)*)).unwrap();
    });
}