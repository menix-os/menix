//! The initrd (initial ram disk) is a CPIO archive which is loaded into memory
//! by the bootloader.
//!
//! This allows the kernel to load drivers needed in order to boot from a
//! block device. It also usually contains the init process which is responsible
//! for actually loading the modules and mounting the real root file system from
//! disk.

#![allow(unused)]

use crate::generic::{
    boot::BootInfo,
    posix::errno::{EResult, Errno},
    process::Identity,
    util::{self, mutex::Mutex},
    vfs::{
        entry::{Entry, Mount, MountFlags},
        file::{File, OpenFlags},
        fs::{FileSystem, SuperBlock},
        inode::{INode, Mode},
        path::Path,
    },
};
use alloc::sync::Arc;
use bytemuck::AnyBitPattern;
use core::ffi::CStr;

#[repr(C)]
#[derive(AnyBitPattern, Clone, Copy)]
struct UStarFsHeader {
    name: [u8; 100],
    mode: [u8; 8],
    uid: [u8; 8],
    gid: [u8; 8],
    size: [u8; 12],
    mtime: [u8; 12],
    checksum: [u8; 8],
    typ: u8,
    linkname: [u8; 100],
    signature: [u8; 6],
    version: [u8; 2],
    owner: [u8; 32],
    group: [u8; 32],
    devmajor: [u8; 8],
    devminor: [u8; 8],
    prefix: [u8; 155],
    pad: [u8; 12],
}
static_assert!(size_of::<UStarFsHeader>() == 512);

const REGULAR: u8 = 0;
const NORMAL: u8 = b'0';
const HARD_LINK: u8 = b'1';
const SYM_LINK: u8 = b'2';
const CHAR_DEV: u8 = b'3';
const BLOCK_DEV: u8 = b'4';
const DIRECTORY: u8 = b'5';
const FIFO: u8 = b'6';
const CONTIGOUS: u8 = b'7';
const GNULONG_PATH: u8 = b'L';

fn oct2bin(str: &[u8]) -> usize {
    let mut n = 0;
    for s in str {
        if *s == 0 {
            break;
        }
        n *= 8;
        n += (*s - b'0') as usize;
    }
    return n;
}

pub fn load(data: &[u8], root: Arc<Entry>) -> EResult<()> {
    let mut offset = 0;
    let mut name_override = None;
    let mut files_loaded = 0usize;

    loop {
        let current_file: &UStarFsHeader =
            bytemuck::try_from_bytes(&data[offset..][..size_of::<UStarFsHeader>()]).unwrap();
        if &current_file.signature != b"ustar\0" {
            break;
        }

        let mut file_name = CStr::from_bytes_until_nul(&current_file.name)
            .unwrap()
            .to_bytes();
        if let Some(n) = name_override {
            file_name = n;
            name_override = None;
        }

        let file_mode = oct2bin(&current_file.mode);
        let file_size = oct2bin(&current_file.size);

        // TODO
        let file = File::open(
            file_name,
            OpenFlags::Create,
            Mode::UserRead | Mode::UserWrite | Mode::UserExec,
            Identity::get_kernel(),
        );
        match current_file.typ {
            REGULAR => (),
            NORMAL => (),
            HARD_LINK => (),
            SYM_LINK => (),
            CHAR_DEV => (),
            BLOCK_DEV => (),
            DIRECTORY => (),
            FIFO => (),
            CONTIGOUS => (),
            GNULONG_PATH => (),
            _ => (),
        }

        files_loaded += 1;

        offset += 512 + util::align_up(file_size, 512);
    }

    return Ok(());
}
