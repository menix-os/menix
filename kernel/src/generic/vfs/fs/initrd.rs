//! The initramfs (initial RAM file system) is a CPIO archive which is loaded into memory by the bootloader.
//!
//! This allows the kernel to load drivers needed in order to boot from a block
//! device. It also usually contains the init process which is responsible for
//! actually loading the modules and mounting the real root file system from
//! disk.

#![allow(unused)]

use crate::generic::{
    boot::BootInfo,
    posix::errno::{EResult, Errno},
    process::Identity,
    util::{self, mutex::Mutex},
    vfs::{
        cache::{Entry, PathNode},
        file::{File, OpenFlags},
        fs::{FileSystem, SuperBlock},
        inode::{INode, Mode, NodeType},
        mknod,
    },
};
use alloc::{string::String, sync::Arc};
use bytemuck::AnyBitPattern;
use core::ffi::CStr;

#[repr(C)]
#[derive(AnyBitPattern, Clone, Copy)]
struct FileHeader {
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
static_assert!(size_of::<FileHeader>() == 512);

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

/// Converts a 0-terminated octal string into a number.
fn oct2bin(str: &[u8]) -> usize {
    let mut n = 0;
    for s in str {
        if *s == 0 || *s == b' ' {
            break;
        }
        n *= 8;
        n += (*s - b'0') as usize;
    }
    return n;
}

/// Creates all directories in the given path and opens the last one. Also returns the final file name.
pub fn create_dirs(at: Arc<File>, path: &[u8]) -> EResult<(Arc<File>, &[u8])> {
    let mut current = at;
    let (path, file_name) = path.rsplit_once(|&x| x == b'/').ok_or(Errno::EINVAL)?;

    for component in path.split(|&x| x == b'/').filter(|&x| !x.is_empty()) {
        match mknod(
            Some(current.clone()),
            component,
            NodeType::Directory,
            Mode::from_bits_truncate(0o755),
            None,
            Identity::get_kernel(),
        ) {
            Ok(_) => {
                current = File::open(
                    Some(current.clone()),
                    component,
                    OpenFlags::Directory,
                    Mode::empty(),
                    Identity::get_kernel(),
                )?;
            }
            Err(e) => {
                if e != Errno::EEXIST {
                    return Err(e);
                }
            }
        }
    }

    return Ok((current, file_name));
}

pub fn load(target: Arc<File>, data: &[u8]) -> EResult<()> {
    let mut offset = 0;
    let mut name_override = None;
    let mut files_loaded = 0;

    loop {
        let current_file: &FileHeader =
            bytemuck::try_from_bytes(&data[offset..][..size_of::<FileHeader>()]).unwrap();
        if &current_file.signature != b"ustar\0" || &current_file.version != b"00" {
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

        // Create the folder structure for this file if it didn't exist already.
        let (dir, file_name) = create_dirs(target.clone(), file_name)?;

        match current_file.typ {
            REGULAR | NORMAL | CONTIGOUS => {
                let file = File::open(
                    Some(dir),
                    file_name,
                    OpenFlags::Create,
                    Mode::from_bits_truncate(file_mode as u32),
                    Identity::get_kernel(),
                )?;

                file.pwrite(&data[offset + 512..][..file_size], 0)?;
            }
            HARD_LINK => todo!(),
            SYM_LINK => todo!(),
            DIRECTORY => todo!(),
            _ => (),
        }

        files_loaded += 1;

        offset += 512 + util::align_up(file_size, 512);
    }

    log!("Loaded {files_loaded} files from initrd");

    return Ok(());
}

init_stage! {
    #[depends(super::super::VFS_STAGE, crate::generic::process::sched::SCHEDULER_STAGE)]
    INITRD_STAGE: "generic.vfs.initrd" => init;
}

fn init() {
    // Load the initramfs into the root directory.
    let root_dir = File::open(
        None,
        b"/",
        OpenFlags::Directory,
        Mode::empty(),
        Identity::get_kernel(),
    )
    .expect("Unable to open root directory");

    for file in BootInfo::get().files {
        load(root_dir.clone(), file.data).expect("Failed to load one of the provided initrds");
    }
}
