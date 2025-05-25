//! The initrd (initial ram disk) is a CPIO archive which is loaded into memory
//! by the bootloader.
//!
//! This allows the kernel to load drivers needed in order to boot from a
//! block device. It also usually contains the init process which is responsible
//! for actually loading the modules and mounting the real root file system from
//! disk.

use crate::generic::{posix::vfs::inode::INode, util};
use alloc::sync::Arc;
use bytemuck::AnyBitPattern;

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
}
static_assert!(size_of::<UStarFsHeader>() == 500);

#[repr(u8)]
enum UStarFileType {
    Regular = 0,
    Normal = b'0',
    HardLink = b'1',
    SymLink = b'2',
    CharDev = b'3',
    BlockDev = b'4',
    Directory = b'5',
    FIFO = b'6',
    Contigous = b'7',
    GNULongPath = b'L',
}

fn oct2bin(str: &[u8]) -> usize {
    let mut n = 0;
    for s in str {
        n *= 8;
        n += (*s - b'0') as usize;
    }
    return n;
}

pub fn load(data: &[u8], mount: Arc<INode>) {
    let offset = 0;
    let mut name_override = None;

    let files_loaded = 0usize;
    loop {
        let current_file: &UStarFsHeader =
            bytemuck::from_bytes(&data[offset..][0..size_of::<UStarFileType>()]);
        if &current_file.signature != b"ustar\0" {
            break;
        }

        let mut name = current_file.name;
        if let Some(n) = name_override {
            name = n;
            name_override = None;
        }

        let file_mode = oct2bin(&current_file.mode);
        let file_size = oct2bin(&current_file.size);

        match current_file.typ {
            // TODO
            _ => break,
        }

        offset += 512 + util::align_up(file_size, 512);
    }

    log!("Loaded {} files from UStar archive", files_loaded);
}
