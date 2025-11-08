use crate::{
    posix::errno::{EResult, Errno},
    process::{Process, task::Task},
    vfs::{File, exec::ExecFormat, file::OpenFlags, inode::Mode},
};
use alloc::{sync::Arc, vec::Vec};

#[derive(Debug)]
struct ShebangFormat;

impl ExecFormat for ShebangFormat {
    fn identify(&self, file: &File) -> bool {
        let mut buffer = [0u8; 2];
        match file.pread(&mut buffer, 0) {
            Ok(x) => {
                if x != buffer.len() as _ {
                    return false;
                }
            }
            Err(_) => return false,
        }

        buffer == *b"#!"
    }

    fn load(&self, proc: &Arc<Process>, info: &mut super::ExecInfo) -> EResult<Task> {
        let mut interp = vec![];
        for i in 0..(uapi::PATH_MAX as _) {
            let mut buf = [0u8];
            info.executable.pread(&mut buf, i + 2)?; // Skip the #!
            if buf[0] == b'\n' {
                break;
            }
            interp.push(buf[0]);
        }

        // Parse the shebang command line.
        let mut args = interp
            .split(|&x| x == b' ')
            .map(|x| x.to_vec())
            .collect::<Vec<_>>();
        args.append(&mut info.argv); // Append the rest to argv.

        info.argv = args;

        let interp_path = info.argv.first().ok_or(Errno::EINVAL)?;
        {
            let inner = proc.inner.lock();
            info.executable = File::open(
                &inner,
                None,
                interp_path,
                OpenFlags::Read | OpenFlags::Executable,
                Mode::UserRead | Mode::UserExec,
                &inner.identity,
            )?;
        }

        let format = super::identify(&info.executable).ok_or(Errno::ENOEXEC)?;
        format.load(proc, info)
    }
}

#[initgraph::task(
    name = "generic.vfs.exec-shebang",
    depends = [crate::memory::MEMORY_STAGE],
    entails = [crate::vfs::VFS_STAGE],
)]
fn ELF_STAGE() {
    super::register("shebang", Arc::new(ShebangFormat));
}
