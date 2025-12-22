pub const DT_UNKNOWN: u8 = 0;
pub const DT_FIFO: u8 = 1;
pub const DT_CHR: u8 = 2;
pub const DT_DIR: u8 = 4;
pub const DT_BLK: u8 = 6;
pub const DT_REG: u8 = 8;
pub const DT_LNK: u8 = 10;
pub const DT_SOCK: u8 = 12;
pub const DT_WHT: u8 = 14;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct dirent {
    pub d_ino: super::ino_t,
    pub d_off: super::off_t,
    pub d_reclen: u16,
    pub d_type: u8,
    pub d_name: [u8; 1024],
}
