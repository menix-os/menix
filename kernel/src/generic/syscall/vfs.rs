use crate::generic::{memory::user::UserPtr, posix::errno::EResult};
use alloc::string::String;

pub fn write(fd: usize, buf: usize, len: usize) -> EResult<usize> {
    let buf = unsafe { core::slice::from_raw_parts(buf as *const u8, len) };
    use core::fmt::Write;
    {
        let mut writer = crate::generic::log::GLOBAL_LOGGERS.lock();
        _ = writer.write_str(&String::from_utf8_lossy(buf));
    }
    Ok(len)
}
