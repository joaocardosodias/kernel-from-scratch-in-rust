use core::fmt::{self, Write};

use spin::Mutex;

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) { WRITER.lock().write_fmt(args).unwrap(); }

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::drivers::vga::_print(core::format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! println {
    () => { $crate::print!("\n") };
    ($($arg:tt)*) => {{
        $crate::drivers::vga::_print(core::format_args!($($arg)*));
        $crate::print!("\n");
    }};
}

const FB_VIRT_ADDR: usize = 0xA00000;
const FONT_DATA: &[u8; 16384] = include_bytes!("font_ter16x32.bin");

pub fn get_width() -> u32 { unsafe { *(0x7008 as *const u32) } }
pub fn get_height() -> u32 { unsafe { *(0x700C as *const u32) } }
pub fn get_pitch() -> u32 { unsafe { *(0x7010 as *const u32) } }

fn get_buffer_width() -> usize { (get_width() / 16) as usize }
fn get_buffer_height() -> usize { (get_height() / 32) as usize }

fn draw_pixel(x: u32, y: u32, color: u32) {
    let width = get_width();
    let height = get_height();
    if x >= width || y >= height {
        return;
    }
    let pitch = get_pitch();
    let offset = (y * pitch + x * 4) as usize;
    unsafe {
        *((FB_VIRT_ADDR + offset) as *mut u32) = color;
    }
}

fn draw_char(c: u8, x: u32, y: u32, fg_color: u32, bg_color: u32) {
    let offset = (c as usize) * 64;
    for row in 0..32 {
        let byte1 = FONT_DATA[offset + row * 2];
        let byte2 = FONT_DATA[offset + row * 2 + 1];
        let row_data = ((byte1 as u32) << 8) | (byte2 as u32);
        for col in 0..16 {
            let bit = (row_data >> (15 - col)) & 1;
            let color = if bit == 1 { fg_color } else { bg_color };
            draw_pixel(x + col as u32, y + row as u32, color);
        }
    }
}

pub static WRITER: Mutex<Writer> = Mutex::new(Writer {
    column: 0,
    row:    0,
    color:  ColorCode {
        fg: Color::White as u32,
        bg: Color::Black as u32,
    },
});

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Black        = 0x00000000,
    Blue         = 0x000000AA,
    Green        = 0x0000AA00,
    Cyan         = 0x0000AAAA,
    Red          = 0x00AA0000,
    Magenta      = 0x00AA00AA,
    Brown        = 0x00AA5500,
    LightGrey    = 0x00AAAAAA,
    DarkGrey     = 0x00555555,
    LightBlue    = 0x005555FF,
    LightGreen   = 0x0055FF55,
    LightCyan    = 0x0055FFFF,
    LightRed     = 0x00FF5555,
    LightMagenta = 0x00FF55FF,
    Yellow       = 0x00FFFF55,
    White        = 0x00FFFFFF,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ColorCode {
    pub fg: u32,
    pub bg: u32,
}

impl ColorCode {
    pub const fn new(foreground: Color, background: Color) -> Self {
        ColorCode {
            fg: foreground as u32,
            bg: background as u32,
        }
    }
}

pub struct Writer {
    column: usize,
    row:    usize,
    color:  ColorCode,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        if byte == 8 {
            if self.column > 0 {
                self.column -= 1;
                let x = (self.column * 16) as u32;
                let y = (self.row * 32) as u32;
                draw_char(b' ', x, y, self.color.fg, self.color.bg);
            }
            return;
        }
        if byte == b'\n' {
            self.column = 0;
            self.row += 1;
            self.check_scroll();
            return;
        }
        let x = (self.column * 16) as u32;
        let y = (self.row * 32) as u32;
        draw_char(byte, x, y, self.color.fg, self.color.bg);
        self.column += 1;
        if self.column >= get_buffer_width() {
            self.column = 0;
            self.row += 1;
        }
        self.check_scroll();
    }

    fn check_scroll(&mut self) {
        if self.row >= get_buffer_height() {
            let fb = FB_VIRT_ADDR as *mut u32;
            let row_bytes = (get_pitch() * 32) / 4;
            let total_pixels = (get_pitch() * get_height()) / 4;
            unsafe {
                core::ptr::copy(
                    fb.add(row_bytes as usize),
                    fb,
                    (total_pixels - row_bytes) as usize,
                );
                let start = (total_pixels - row_bytes) as usize;
                for i in start..(total_pixels as usize) {
                    fb.add(i).write(self.color.bg);
                }
            }
            self.row = get_buffer_height() - 1;
            self.column = 0;
        }
    }

    pub fn clear_screen(&mut self) {
        let fb = FB_VIRT_ADDR as *mut u32;
        let total_pixels = (get_pitch() * get_height()) / 4;
        unsafe {
            for i in 0..(total_pixels as usize) {
                fb.add(i).write(self.color.bg);
            }
        }
        self.column = 0;
        self.row = 0;
    }
}

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.as_bytes() {
            self.write_byte(*byte);
        }
        Ok(())
    }
}
