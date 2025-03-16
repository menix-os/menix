// Framebuffer console for kernel messages.

use alloc::boxed::Box;
use spin::Mutex;

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
    internal_buffer: None,
});

pub struct FbCon {
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
    internal_buffer: Option<Box<[u8]>>,
}

impl FbCon {
    // TODO
}
