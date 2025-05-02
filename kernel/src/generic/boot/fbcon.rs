// Early framebuffer boot console (likely using an EFI GOP framebuffer).

use alloc::{boxed::Box, vec::Vec};
use core::{intrinsics::volatile_copy_nonoverlapping_memory, ptr::NonNull};

use crate::generic::{
    boot::BootInfo,
    log::{Logger, LoggerSink},
    memory::{
        PhysAddr,
        virt::{KERNEL_PAGE_TABLE, VmFlags, VmLevel},
    },
    util::align_up,
};

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

unsafe impl Sync for FrameBuffer {}
unsafe impl Send for FrameBuffer {}

const FONT_DATA: &[u8] = include_bytes!("../../../assets/builtin_font.bin");
const FONT_WIDTH: usize = 8;
const FONT_HEIGHT: usize = 12;
const FONT_GLYPH_SIZE: usize = (FONT_WIDTH * FONT_HEIGHT) / 8;

struct FbCon {
    /// Screen width in characters.
    ch_width: usize,
    /// Screen height in characters.
    ch_height: usize,
    /// Current cursor position on the X axis.
    ch_xpos: usize,
    /// Current cursor position on the Y axis.
    ch_ypos: usize,
    /// Back buffer to draw updates on.
    back_buffer: Vec<u8>,
    /// The buffer to draw on.
    fb: FrameBuffer,
    screen: *mut u8,
}

init_call_if_cmdline!("fbcon", true, init);

fn init() {
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
        "fbcon: Resolution = {}x{}x{}, Phys = {:#018X}, Virt = {:#018X}",
        fb.width,
        fb.height,
        fb.cpp * 8,
        fb.base.value(),
        mem as usize
    );

    Logger::add_sink(Box::new(FbCon {
        ch_width: fb.width / FONT_WIDTH,
        ch_height: fb.height / FONT_HEIGHT,
        ch_xpos: 0,
        ch_ypos: 0,
        back_buffer: buf,
        fb,
        screen: mem,
    }));
}

impl FbCon {
    /// Scrolls the console by one line.
    fn scroll(&mut self) {
        // Amount of bytes for each line.
        let line_bytes = FONT_HEIGHT * self.fb.pitch;
        // Copy line 1.. to 0..
        self.back_buffer.copy_within(line_bytes.., 0);
        // Clear the last line.
        self.back_buffer[((self.ch_height - 1) * line_bytes)..].fill(0);

        unsafe {
            volatile_copy_nonoverlapping_memory(
                self.screen,
                self.back_buffer.as_ptr(),
                self.back_buffer.len(),
            )
        };
    }

    fn putchar(&mut self, ch: u8) {
        let code_point = ch as u8;
        let pix_pos = (self.ch_xpos * FONT_WIDTH, self.ch_ypos * FONT_HEIGHT);
        let pitch = self.fb.pitch;
        let cpp = self.fb.cpp;

        for y in 0..FONT_HEIGHT {
            for x in 0..FONT_WIDTH {
                let offset = (pitch * (pix_pos.1 + y)) + (cpp * (pix_pos.0 + x));
                let addr = self.screen;
                if FONT_DATA[(code_point as usize * FONT_GLYPH_SIZE) + y]
                    & 1 << (FONT_WIDTH - x - 1)
                    != 0
                {
                    unsafe {
                        addr.add(offset + 0).write_volatile(0xff);
                        addr.add(offset + 1).write_volatile(0xff);
                        addr.add(offset + 2).write_volatile(0xff);
                        addr.add(offset + 3).write_volatile(0xff);
                    };
                    self.back_buffer[offset + 0] = 0xff;
                    self.back_buffer[offset + 1] = 0xff;
                    self.back_buffer[offset + 2] = 0xff;
                    self.back_buffer[offset + 3] = 0xff;
                } else {
                    unsafe {
                        addr.add(offset + 0).write_volatile(0x00);
                        addr.add(offset + 1).write_volatile(0x00);
                        addr.add(offset + 2).write_volatile(0x00);
                        addr.add(offset + 3).write_volatile(0x00);
                    };
                    self.back_buffer[offset + 0] = 0x00;
                    self.back_buffer[offset + 1] = 0x00;
                    self.back_buffer[offset + 2] = 0x00;
                    self.back_buffer[offset + 3] = 0x00;
                }
            }
        }

        self.ch_xpos += 1;
    }
}

unsafe impl Send for FbCon {}

impl LoggerSink for FbCon {
    fn name(&self) -> &'static str {
        "fbcon"
    }

    fn write(&mut self, input: &[u8]) {
        for ch in input {
            match ch {
                b'\n' => {
                    self.ch_xpos = 0;
                    self.ch_ypos += 1;
                    continue;
                }
                b'\t' => {
                    self.ch_xpos = align_up(self.ch_xpos + 1, 8);
                    continue;
                }
                _ => (),
            }

            if self.ch_xpos >= self.ch_width {
                self.ch_xpos = 0;
                self.ch_ypos += 1;
            }

            if self.ch_ypos >= self.ch_height {
                self.scroll();
                self.ch_ypos = self.ch_height - 1;
            }

            self.putchar(*ch);
        }
    }
}
