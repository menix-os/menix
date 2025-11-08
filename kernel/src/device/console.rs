use crate::{
    device::Device,
    log::GLOBAL_LOGGERS,
    memory::user::UserPtr,
    posix::errno::{EResult, Errno},
    process::{Identity, PROCESS_STAGE, Process},
    util::mutex::irq::IrqMutex,
    vfs::{
        self, File, VFS_DEV_MOUNT_STAGE,
        file::FileOps,
        inode::{Mode, NodeType},
    },
};
use alloc::{string::String, sync::Arc};
use core::fmt::Write;

#[derive(Debug)]
struct Console;

impl FileOps for Console {
    fn read(&self, _: &File, _: &mut [u8], _: u64) -> EResult<isize> {
        Ok(0)
    }

    fn write(&self, _: &File, buffer: &[u8], _: u64) -> EResult<isize> {
        let _lock = IrqMutex::lock();
        let mut writer = GLOBAL_LOGGERS.lock();
        _ = writer.write_str(&String::from_utf8_lossy(buffer));
        Ok(buffer.len() as _)
    }

    fn ioctl(&self, _: &File, request: usize, arg: usize) -> EResult<usize> {
        match request as _ {
            uapi::TIOCGWINSZ => {
                let arg: UserPtr<'_, uapi::winsize> = UserPtr::new(arg.into());
                arg.write(uapi::winsize {
                    ws_row: 25,
                    ws_col: 80,
                    ..Default::default()
                })
                .ok_or(Errno::EINVAL)?;
            }
            _ => return Err(Errno::ENOSYS),
        }
        Ok(0)
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
