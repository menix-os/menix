use super::{fsblkcnt_t, fsfilcnt_t};

pub const ST_RDONLY: u32 = 1;
pub const ST_NOSUID: u32 = 2;
pub const ST_MANDLOCK: u32 = 64;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct statvfs {
    pub f_bsize: usize,
    pub f_frsize: usize,
    pub f_blocks: fsblkcnt_t,
    pub f_bfree: fsblkcnt_t,
    pub f_bavail: fsblkcnt_t,
    pub f_files: fsfilcnt_t,
    pub f_ffree: fsfilcnt_t,
    pub f_favail: fsfilcnt_t,
    pub f_fsid: usize,
    pub f_flag: usize,
    pub f_namemax: usize,
    pub f_basetype: [u8; 80],
}
