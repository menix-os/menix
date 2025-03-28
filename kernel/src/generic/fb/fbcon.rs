// Framebuffer console for kernel messages.

use alloc::{boxed::Box, vec::Vec};
use spin::Mutex;

use super::FRAMEBUFFER;

const FONT_DATA: &[u8] = include_bytes!("../../../bin/fbcon_font.bin");
const FONT_WIDTH: usize = 8;
const FONT_HEIGHT: usize = 12;
const FONT_GLYPH_SIZE: usize = ((FONT_WIDTH * FONT_HEIGHT) / 8);

static FBCON: Mutex<FbCon> = Mutex::new(FbCon {
    ch_width: 0,
    ch_height: 0,
    ch_xpos: 0,
    ch_ypos: 0,
    active: false,
    internal_buffer: Vec::new(),
});

struct FbCon {
    /// Screen width in characters.
    ch_width: usize,
    /// Screen height in characters.
    ch_height: usize,
    /// Current cursor position on the X axis.
    ch_xpos: usize,
    /// Current cursor position on the Y axis.
    ch_ypos: usize,
    /// If messages should be drawn to the framebuffer.
    active: bool,
    /// Back buffer to draw updates on.
    internal_buffer: Vec<u8>,
}

impl FbCon {
    /// Flips the entire backbuffer to the screen.
    fn flip(&self) {
        if let Some(x) = &mut *FRAMEBUFFER.write() {
            // TODO: Write to buffer
        }
    }

    /// Scrolls the console by one line.
    fn scroll(&mut self) {
        if let Some(x) = &*FRAMEBUFFER.read() {
            // Amount of bytes for each line.
            let line_bytes = FONT_HEIGHT * x.mode.pitch as usize;
            // Copy line 1.. to 0..
            self.internal_buffer.copy_within(line_bytes.., 0);
            // Clear the last line.
            self.internal_buffer[((self.ch_height - 1) * line_bytes)..].fill(0);
        }
    }

    fn putchar(&mut self, ch: char) {
        if let Some(x) = &mut *FRAMEBUFFER.write() {
            let code_point = ch as u8; // TODO: UTF-8 support.
            let pix_pos = (self.ch_xpos * FONT_WIDTH, self.ch_ypos * FONT_HEIGHT);
            let pitch = x.mode.pitch as usize;
            let cpp = x.mode.cpp;

            for y in 0..FONT_HEIGHT {
                for x in 0..FONT_WIDTH {
                    let pixel: u32 = if FONT_DATA[(code_point as usize * FONT_GLYPH_SIZE) + y]
                        & 1 << (FONT_WIDTH - x - 1)
                        != 0
                    {
                        0xffff_ffff
                    } else {
                        0x0000_0000
                    };
                    let offset = (pitch * (pix_pos.1 + y));
                }
            }
        }
    }
}
