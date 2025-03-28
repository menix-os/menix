use spin::RwLock;

pub mod fbcon;

pub struct FbColorBits {
    offset: u32,
    size: u32,
}

pub struct FbModeInfo {
    width: u32,
    height: u32,
    v_width: u32,
    v_height: u32,
    v_off_x: u32,
    v_off_y: u32,
    cpp: u8,
    pitch: u32,
    red: FbColorBits,
    green: FbColorBits,
    blue: FbColorBits,
    alpha: FbColorBits,
}

pub struct FrameBuffer {
    pub screen: (),
    pub mode: FbModeInfo,
}

pub static FRAMEBUFFER: RwLock<Option<FrameBuffer>> = RwLock::new(None);
