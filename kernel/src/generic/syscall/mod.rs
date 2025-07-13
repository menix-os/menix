mod memory;
mod numbers;
mod process;
mod system;
mod vfs;

use super::posix::errno::Errno;

/// Executes the syscall as identified by `num`.
/// Returns a tuple of (value, error) to the user. An error code of 0 inidcates success.
/// If the error code is not 0, `value` is not valid and indicates failure.
pub fn dispatch(
    num: usize,
    a0: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
) -> (usize, usize) {
    let result = match num {
        numbers::EXIT => process::exit(a0),
        numbers::UNAME => system::uname(a0.into()),
        numbers::ARCHCTL => system::archctl(a0, a1),
        numbers::MMAP => memory::mmap(a0.into(), a1, a2 as _, a3 as _, a4 as _, a5 as _),
        numbers::GETTID => Ok(process::gettid()),
        numbers::GETPID => Ok(process::getpid()),
        numbers::GETPPID => Ok(process::getppid()),
        numbers::WRITE => vfs::write(a0, a1, a2),
        _ => {
            warn!("Unknown syscall {num}");
            Err(Errno::ENOSYS)
        }
    };
    match result {
        Ok(x) => return (x, 0),
        Err(x) => return (0, x as usize),
    }
}
