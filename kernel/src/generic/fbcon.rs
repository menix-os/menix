// Early framebuffer boot console (likely using an EFI GOP framebuffer).

use crate::generic::{
    boot::BootInfo,
    log::{self, LoggerSink},
    memory::{
        PhysAddr,
        slab::ALLOCATOR,
        virt::{KERNEL_PAGE_TABLE, VmFlags, VmLevel},
    },
};
use alloc::{boxed::Box, vec::Vec};
use core::{
    alloc::{GlobalAlloc, Layout},
    ffi::c_void,
    ptr::null_mut,
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
    back_buffer: Vec<u8>,
    /// The buffer to draw on.
    fb: FrameBuffer,
    /// The flanterm context.
    ctx: *mut flanterm_context,
}

unsafe impl Send for FbCon {}

unsafe extern "C" fn malloc(size: usize) -> *mut core::ffi::c_void {
    let mem = unsafe { ALLOCATOR.alloc(Layout::from_size_align(size, align_of::<u8>()).unwrap()) };
    mem as *mut core::ffi::c_void
}

unsafe extern "C" fn free(ptr: *mut core::ffi::c_void, size: usize) {
    unsafe {
        ALLOCATOR.dealloc(
            ptr as *mut u8,
            Layout::from_size_align(size, align_of::<u8>()).unwrap(),
        )
    };
}

pub fn init() {
    let Some(fb) = BootInfo::get().framebuffer.clone() else {
        return;
    };
    let mut buf = Vec::new();
    buf.resize(fb.pitch * fb.height, 0);

    // Map the framebuffer in memory.
    let mem = KERNEL_PAGE_TABLE
        .write()
        .map_memory(
            fb.base,
            VmFlags::Read | VmFlags::Write,
            VmLevel::L1,
            fb.pitch * fb.height,
        )
        .unwrap();

    log!(
        "Resolution = {}x{}x{}, Phys = {:#018X}, Virt = {:#018X}",
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
            back_buffer: buf,
            fb,
            ctx,
        }));
    }
}

impl LoggerSink for FbCon {
    fn name(&self) -> &'static str {
        "fbcon"
    }

    fn write(&mut self, input: &[u8]) {
        unsafe { flanterm_write(self.ctx, input.as_ptr() as *const i8, input.len()) };
    }
}
