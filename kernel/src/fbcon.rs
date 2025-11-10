//! Early frame buffer boot console (likely using an EFI GOP framebuffer).

use crate::{
    boot::BootInfo,
    log::{self, LoggerSink},
    memory::{
        PhysAddr, free, malloc,
        pmm::KernelAlloc,
        virt::{VmFlags, mmu::PageTable},
    },
};
use alloc::boxed::Box;
#[allow(unused)]
use alloc::vec;
use core::{
    ffi::{c_char, c_void},
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

const FONT_DATA: &[u8] = include_bytes!("../assets/builtin_font.bin");
const FONT_WIDTH: usize = 8;
const FONT_HEIGHT: usize = 12;

struct FbCon {
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

        PageTable::get_kernel()
            .unmap_range::<KernelAlloc>(
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
                input.as_ptr() as *const c_char,
                input.len(),
            )
        };
    }
}

static BG_COLOR: u32 = 0x00111111;

#[initgraph::task(
    name = "generic.fbcon",
    depends = [super::memory::MEMORY_STAGE],
)]
pub fn FBCON_STAGE() {
    let Some(fb) = BootInfo::get().framebuffer.clone() else {
        return;
    };

    // Map the framebuffer in memory.
    let mem = PageTable::get_kernel()
        .map_memory::<KernelAlloc>(
            fb.base,
            VmFlags::Read | VmFlags::Write,
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
            (&raw const BG_COLOR) as *mut u32,
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
            fb,
            mem: AtomicPtr::new(mem),
            ctx: AtomicPtr::new(ctx),
        }));
    }
}
