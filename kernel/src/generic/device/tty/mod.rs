use crate::generic::{
    device::Device,
    log::GLOBAL_LOGGERS,
    memory::user::UserPtr,
    posix::errno::{EResult, Errno},
    process::PROCESS_STAGE,
    util::mutex::{Mutex, irq::IrqMutex},
    vfs::{File, VFS_DEV_MOUNT_STAGE, file::FileOps},
};
use alloc::{string::String, sync::Arc};
use core::fmt::{Debug, Write};

#[derive(Debug)]
struct Tty {
    index: usize,
    name: String,
    driver: Arc<dyn TtyDriver>,
    winsize: Mutex<uapi::winsize>,
}

impl FileOps for Tty {
    fn read(&self, _: &File, buffer: &mut [u8], _: uapi::off_t) -> EResult<isize> {
        buffer.fill(0);
        Ok(0)
    }

    fn write(&self, _: &File, buffer: &[u8], _: uapi::off_t) -> EResult<isize> {
        let _lock = IrqMutex::lock();
        let mut writer = GLOBAL_LOGGERS.lock();
        _ = writer.write_str(&String::from_utf8_lossy(buffer));
        Ok(buffer.len() as _)
    }

    fn ioctl(&self, file: &File, request: usize, arg: usize) -> EResult<usize> {
        match request as _ {
            uapi::TIOCGWINSZ => {
                let arg: UserPtr<'_, uapi::winsize> = UserPtr::new(arg.into());
                arg.write(uapi::winsize {
                    ws_row: 80,
                    ws_col: 25,
                    ws_xpixel: 0, // Unused
                    ws_ypixel: 0, // Unused
                })
                .ok_or(Errno::EINVAL)?;
            }
            _ => return self.driver.ioctl(file, request, arg),
        }
        Ok(0)
    }
}

impl Device for Tty {
    fn open(&self) -> EResult<()> {
        self.driver.open(self)
    }

    fn name(&self) -> &str {
        "tty"
    }
}

trait TtyDriver: Debug {
    fn open(&self, tty: &Tty) -> EResult<()> {
        _ = tty;
        Ok(())
    }

    fn ioctl(&self, file: &File, request: usize, arg: usize) -> EResult<usize> {
        _ = (file, request, arg);
        Err(Errno::ENOSYS)
    }
}

#[initgraph::task(
    name = "generic.device.tty",
    depends = [PROCESS_STAGE, VFS_DEV_MOUNT_STAGE]
)]
pub fn TTY_STAGE() {}
