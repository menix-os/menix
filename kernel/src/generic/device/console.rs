use core::fmt::Write;

use alloc::{string::String, sync::Arc};

use crate::generic::{
    device::Device,
    log::GLOBAL_LOGGERS,
    memory::user::UserPtr,
    posix::errno::{EResult, Errno},
    process::{Identity, PROCESS_STAGE, Process},
    sched::Scheduler,
    util::mutex::irq::IrqMutex,
    vfs::{
        self, File, VFS_DEV_MOUNT_STAGE,
        file::FileOps,
        inode::{Mode, NodeType},
    },
};

#[derive(Debug)]
struct Console;

impl FileOps for Console {
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

    fn ioctl(&self, _: &File, request: usize, arg: usize) -> EResult<usize> {
        let proc = Scheduler::get_current().get_process();
        let space = { proc.inner.lock().address_space.clone() };
        match request as _ {
            uapi::TIOCGWINSZ => {
                let arg: UserPtr<'_, uapi::winsize> = UserPtr::new(arg.into());
                arg.write(
                    &space,
                    uapi::winsize {
                        ws_row: 80,
                        ws_col: 25,
                        ws_xpixel: 0, // Unused
                        ws_ypixel: 0, // Unused
                    },
                )
                .ok_or(Errno::EINVAL)?;
            }
            _ => return Err(Errno::ENOSYS),
        }
        Ok(0)
    }

    fn poll(&self, file: &File, mask: u16) -> EResult<u16> {
        _ = (file, mask);
        Ok(mask)
    }
}

impl Device for Console {
    fn open(&self) -> EResult<()> {
        Ok(())
    }

    fn name(&self) -> &str {
        "console"
    }
}

#[initgraph::task(
    name = "generic.device.console",
    depends = [PROCESS_STAGE, VFS_DEV_MOUNT_STAGE]
)]
fn CONSOLE_STAGE() {
    let inner = Process::get_kernel().inner.lock();
    vfs::mknod(
        &inner,
        None,
        b"/dev/console",
        NodeType::CharacterDevice,
        Mode::from_bits_truncate(0o666),
        Some(Arc::new(Console)),
        Identity::get_kernel(),
    )
    .expect("Unable to create console");
}
