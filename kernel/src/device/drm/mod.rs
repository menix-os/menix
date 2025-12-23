use crate::{
    memory::{AddressSpace, UserPtr, VirtAddr, VmFlags},
    posix::errno::{EResult, Errno},
    util::mutex::spin::SpinMutex,
    vfs::{
        File,
        file::{FileOps, MmapFlags},
    },
};
use alloc::{sync::Arc, vec::Vec};
use core::num::NonZeroUsize;
use uapi::drm;
mod plainfb;

pub enum ModeObject {
    Crtc(Crtc),
    Connector(Connector),
    Encoder(Encoder),
    Framebuffer(Framebuffer),
}

impl ModeObject {
    pub fn id(&self) -> u32 {
        match self {
            ModeObject::Crtc(x) => x.id,
            ModeObject::Connector(x) => x.id,
            ModeObject::Encoder(x) => x.id,
            ModeObject::Framebuffer(x) => x.id,
        }
    }
}

pub struct Crtc {
    id: u32,
    cursor: (u32, u32),
    enabled: bool,
}

pub struct Encoder {
    id: u32,
}

pub struct Connector {
    id: u32,
}

pub struct Framebuffer {
    id: u32,
}

pub trait Device {
    fn create_dumb(&self, width: u32, height: u32, bpp: u32) -> ();
}

/// Represents a user-facing DRM card in form of a file.
pub struct DrmFile {
    card: usize,
    device: Arc<dyn Device>,

    crtcs: SpinMutex<Vec<Arc<Crtc>>>,
    encoders: SpinMutex<Vec<Arc<Encoder>>>,
    connectors: SpinMutex<Vec<Arc<Connector>>>,
    framebuffers: SpinMutex<Vec<Arc<Framebuffer>>>,
}

impl DrmFile {
    pub fn new(device: Arc<dyn Device>) -> Arc<Self> {
        Arc::new(Self {
            card: 0,
            device,
            crtcs: SpinMutex::new(Vec::new()),
            encoders: SpinMutex::new(Vec::new()),
            connectors: SpinMutex::new(Vec::new()),
            framebuffers: SpinMutex::new(Vec::new()),
        })
    }
}

impl FileOps for DrmFile {
    fn ioctl(&self, _file: &File, request: usize, arg: VirtAddr) -> EResult<usize> {
        match request as u32 {
            drm::DRM_IOCTL_SET_VERSION => {
                let mut ptr = UserPtr::<drm::drm_version>::new(arg);
                let mut val = ptr.read().ok_or(Errno::EFAULT)?;

                ptr.write(val);
            }
            drm::DRM_IOCTL_GET_CAP => {
                let mut ptr = UserPtr::<drm::drm_get_cap>::new(arg);
                let mut val = ptr.read().ok_or(Errno::EFAULT)?;
                match val.capability {
                    DRM_CAP_DUMB_BUFFER => val.value = 1,
                    _ => warn!("Unknown Capability {}!", val.capability),
                }
                ptr.write(val);
            }
            drm::DRM_IOCTL_MODE_GETRESOURCES => {
                let mut ptr = UserPtr::<drm::drm_mode_card_res>::new(arg);
                let mut val = ptr.read().ok_or(Errno::EFAULT)?;
                val.count_crtcs = 1;
                val.count_connectors = 1;
                ptr.write(val);
            }
            drm::DRM_IOCTL_MODE_GETCONNECTOR => {
                let mut ptr = UserPtr::<drm::drm_mode_get_connector>::new(arg);
                let mut res = ptr.read().ok_or(Errno::EFAULT)?;
                res.connection = 1;
                ptr.write(res);
            }
            drm::DRM_IOCTL_MODE_GETENCODER => {
                let mut ptr = UserPtr::<drm::drm_mode_get_encoder>::new(arg);
                let mut res = ptr.read().ok_or(Errno::EFAULT)?;
                ptr.write(res);
            }
            drm::DRM_IOCTL_MODE_GETPLANE => {
                let mut ptr = UserPtr::<drm::drm_mode_get_plane>::new(arg);
                let mut res = ptr.read().ok_or(Errno::EFAULT)?;
                ptr.write(res);
            }
            drm::DRM_IOCTL_MODE_CREATE_DUMB => {
                let mut ptr = UserPtr::<drm::drm_mode_create_dumb>::new(arg);
                let mut res = ptr.read().ok_or(Errno::EFAULT)?;
                ptr.write(res);
            }
            drm::DRM_IOCTL_MODE_GETCRTC => {
                let mut ptr = UserPtr::<drm::drm_mode_crtc>::new(arg);
                let mut val = ptr.read().ok_or(Errno::EFAULT)?;
                warn!("MODE_GETCRTC is a stub!");
                ptr.write(val);
            }
            _ => return Err(Errno::ENOSYS),
        }
        Ok(0)
    }

    fn mmap(
        &self,
        file: &File,
        space: &mut AddressSpace,
        addr: VirtAddr,
        len: NonZeroUsize,
        prot: VmFlags,
        flags: MmapFlags,
        offset: uapi::off_t,
    ) -> EResult<VirtAddr> {
        Err(Errno::ENOSYS)
    }
}
