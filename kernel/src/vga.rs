use core::fmt::{self, Write};
use spin::Mutex;
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::vga::WRITER.lock().write_fmt(core::format_args!($($arg)*)).unwrap();
    };
}
#[macro_export]
macro_rules! println {
    () => { print!("\n") };
    ($($arg:tt)*) => {{
        let mut writer = $crate::vga::WRITER.lock();
        writer.write_fmt(core::format_args!($($arg)*)).unwrap();
        writer.write_str("\n").unwrap();
    }};
}

const BLANK: ScreenChar = ScreenChar {
    ascii_code: b' ',
    color_code: ColorCode::new(Color::White, Color::Black),
};
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
const BUFFER_ADDR: usize = 0xB8000;

pub static WRITER: Mutex<Writer> = Mutex::new(Writer {
    column: 0,
    row: 0,
    color: ColorCode::new(Color::White, Color::Black),
});

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Black = 0x00,
    Blue = 0x01,
    Green = 0x02,
    Cyan = 0x03,
    Red = 0x04,
    Magenta = 0x05,
    Brown = 0x06,
    LightGrey = 0x07,
    DarkGrey = 0x08,
    LightBlue = 0x09,
    LightGreen = 0x0A,
    LightCyan = 0x0B,
    LightRed = 0x0C,
    LightMagenta = 0x0D,
    Yellow = 0x0E,
    White = 0x0F,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ColorCode(u8);

impl ColorCode {
    pub const fn new(foreground: Color, background: Color) -> Self {
        let color = ((background as u8) << 4) | ((foreground as u8) & 0x0F);
        Self(color)
    }
}
#[repr(C)]
pub struct ScreenChar {
    ascii_code: u8,
    color_code: ColorCode,
}

pub struct Writer {
    column: usize,
    row: usize,
    color: ColorCode,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        let buffer = BUFFER_ADDR as *mut ScreenChar;
        let offset = self.row * BUFFER_WIDTH + self.column;

        if byte == b'\n' {
            self.column = 0;
            self.row += 1;
            return;
        }
        unsafe {
            buffer.add(offset).write(ScreenChar {
                ascii_code: byte,
                color_code: self.color,
            });
        }

        self.column += 1;
        if self.column >= BUFFER_WIDTH {
            self.column = 0;
            self.row += 1;
        }
        if self.row >= BUFFER_HEIGHT {
            for i in 0..(BUFFER_HEIGHT - 1) {
                for j in 0..BUFFER_WIDTH {
                    unsafe {
                        let current = buffer.add(BUFFER_WIDTH * (i + 1) + j).read();
                        buffer.add(BUFFER_WIDTH * i + j).write(current);
                    }
                }
            }
            for k in 0..BUFFER_WIDTH {
                unsafe {
                    buffer
                        .add((BUFFER_HEIGHT - 1) * BUFFER_WIDTH + k)
                        .write(BLANK);
                }
            }
            self.row = 24;
            self.column = 0;
        }
    }

    pub fn clear_screen(&mut self) {
        let buffer = BUFFER_ADDR as *mut ScreenChar;
        for i in 0..(BUFFER_HEIGHT * BUFFER_WIDTH) {
            unsafe { buffer.add(i).write(BLANK) }
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
