pub const S_IFMT: u32 = 0x0F000;
pub const S_IFBLK: u32 = 0x06000;
pub const S_IFCHR: u32 = 0x02000;
pub const S_IFIFO: u32 = 0x01000;
pub const S_IFREG: u32 = 0x08000;
pub const S_IFDIR: u32 = 0x04000;
pub const S_IFLNK: u32 = 0x0A000;
pub const S_IFSOCK: u32 = 0x0C000;

pub const S_IRWXU: u32 = 0o700;
pub const S_IRUSR: u32 = 0o400;
pub const S_IWUSR: u32 = 0o200;
pub const S_IXUSR: u32 = 0o100;
pub const S_IRWXG: u32 = 0o70;
pub const S_IRGRP: u32 = 0o40;
pub const S_IWGRP: u32 = 0o20;
pub const S_IXGRP: u32 = 0o10;
pub const S_IRWXO: u32 = 0o7;
pub const S_IROTH: u32 = 0o4;
pub const S_IWOTH: u32 = 0o2;
pub const S_IXOTH: u32 = 0o1;
pub const S_ISUID: u32 = 0o4000;
pub const S_ISGID: u32 = 0o2000;
pub const S_ISVTX: u32 = 0o1000;
pub const S_IREAD: u32 = S_IRUSR;
pub const S_IWRITE: u32 = S_IWUSR;
pub const S_IEXEC: u32 = S_IXUSR;

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct stat {
    pub st_dev: super::dev_t,
    pub st_ino: super::ino_t,
    pub st_mode: super::mode_t,
    pub st_nlink: super::nlink_t,
    pub st_uid: super::uid_t,
    pub st_gid: super::gid_t,
    pub st_rdev: super::dev_t,
    pub st_size: super::off_t,
    pub st_atim: super::time::timespec,
    pub st_mtim: super::time::timespec,
    pub st_ctim: super::time::timespec,
    pub st_blksize: super::blksize_t,
    pub st_blocks: super::blkcnt_t,
}
