use alloc::sync::Arc;

use crate::{
    device::drm::{Device, DrmFile},
    process::{Identity, Process},
    vfs::{self, inode::Mode},
};

struct PlainFb;
impl Device for PlainFb {
    fn create_dumb(&self, width: u32, height: u32, bpp: u32) -> () {
        todo!()
    }
}

#[initgraph::task(
    name = "generic.drm.plainfb",
    depends = [crate::vfs::VFS_DEV_MOUNT_STAGE, crate::process::PROCESS_STAGE]
)]
fn PLAINFB_STAGE() {
    let proc = Process::get_kernel();
    let root = proc.root_dir.lock();
    let cwd = proc.working_dir.lock();

    let card = DrmFile::new(Arc::new(PlainFb));

    vfs::mknod(
        root.clone(),
        cwd.clone(),
        b"/dev/drmcard0",
        vfs::inode::NodeType::CharacterDevice,
        Mode::from_bits_truncate(0o777),
        Some(card),
        &Identity::get_kernel(),
    )
    .expect("ajasdfkjsdfhn");
}
