//! Early frame buffer boot console (likely using an EFI GOP framebuffer).

use crate::generic::{
    boot::BootInfo,
    log::{self, LoggerSink},
    memory::{
        PhysAddr, free, malloc,
        virt::{KERNEL_PAGE_TABLE, VmFlags, VmLevel},
    },
};
use alloc::{boxed::Box, vec::Vec};
use core::{
    ffi::c_void,
    ptr::null_mut,
    sync::atomic::{AtomicPtr, Ordering},
};
use flanterm_sys::{flanterm_context, flanterm_write};

#[derive(Default, Debug, Clone)]
pub struct FbColorBits {
    pub offset: u8,
    pub size: u8,
}

#[derive(Debug, Clone)]
pub struct FrameBuffer {
    pub base: PhysAddr,
    pub width: usize,
    pub height: usize,
    pub pitch: usize,
    pub cpp: usize,
    pub red: FbColorBits,
    pub green: FbColorBits,
    pub blue: FbColorBits,
}

const FONT_DATA: &[u8] = include_bytes!("../../assets/builtin_font.bin");
const FONT_WIDTH: usize = 8;
const FONT_HEIGHT: usize = 12;

struct FbCon {
    /// Back buffer to draw updates on.
    _buf: Vec<u8>,
    /// Frame buffer that is being drawn on.
    fb: FrameBuffer,
    /// Start of memory mapped region that is used to access the frame buffer.
    mem: AtomicPtr<u8>,
    /// The flanterm context.
    ctx: AtomicPtr<flanterm_context>,
}

impl Drop for FbCon {
    fn drop(&mut self) {
        unsafe { flanterm_sys::flanterm_deinit(self.ctx.load(Ordering::Acquire), Some(free)) };

        KERNEL_PAGE_TABLE
            .lock()
            .unmap_range(
                self.mem.load(Ordering::Relaxed).into(),
                self.fb.pitch * self.fb.height,
            )
            .unwrap();
    }
}

impl LoggerSink for FbCon {
    fn name(&self) -> &'static str {
        "fbcon"
    }

    fn write(&mut self, input: &[u8]) {
        unsafe {
            flanterm_write(
                self.ctx.load(Ordering::Acquire),
                input.as_ptr() as *const i8,
                input.len(),
            )
        };
    }
}

init_call_if_cmdline!("fbcon", true, init);
pub fn init() {
    let Some(fb) = BootInfo::get().framebuffer.clone() else {
        return;
    };

    let mut back_buffer = Vec::new();
    back_buffer.resize(fb.pitch * fb.height, 0);

    // Map the framebuffer in memory.
    let mem = KERNEL_PAGE_TABLE
        .lock()
        .map_memory(
            fb.base,
            VmFlags::Read | VmFlags::Write,
            VmLevel::L1,
            fb.pitch * fb.height,
        )
        .unwrap();

    log!(
        "Resolution = {}x{}x{}, Phys = {:#018x}, Virt = {:#018x}",
        fb.width,
        fb.height,
        fb.cpp * 8,
        fb.base.value(),
        mem as usize
    );

    unsafe {
        let ctx = flanterm_sys::flanterm_fb_init(
            Some(malloc),
            Some(free),
            mem as *mut u32,
            fb.width,
            fb.height,
            fb.pitch,
            fb.red.size,
            fb.red.offset,
            fb.green.size,
            fb.green.offset,
            fb.blue.size,
            fb.blue.offset,
            null_mut(),
            null_mut(),
            null_mut(),
            null_mut(),
            null_mut(),
            null_mut(),
            null_mut(),
            FONT_DATA.as_ptr() as *mut c_void,
            FONT_WIDTH,
            FONT_HEIGHT,
            0,
            1,
            1,
            0,
        );

        log::add_sink(Box::new(FbCon {
            _buf: back_buffer,
            fb,
            mem: AtomicPtr::new(mem),
            ctx: AtomicPtr::new(ctx),
        }));
    }
}
