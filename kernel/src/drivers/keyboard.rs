use spin::Mutex;

const BUFFER_SIZE: usize = 128;

pub struct RingBuffer {
    data: [u8; BUFFER_SIZE],
    head: usize,
    tail: usize,
}

impl Default for RingBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl RingBuffer {
    pub const fn new() -> Self {
        RingBuffer {
            data: [0; BUFFER_SIZE],
            head: 0,
            tail: 0,
        }
    }

    pub fn push(&mut self, val: u8) {
        let next = (self.head + 1) % BUFFER_SIZE;
        if next != self.tail {
            self.data[self.head] = val;
            self.head = next;
        }
    }

    pub fn pop(&mut self) -> Option<u8> {
        if self.head == self.tail {
            None
        } else {
            let val = self.data[self.tail];
            self.tail = (self.tail + 1) % BUFFER_SIZE;
            Some(val)
        }
    }
}

pub static KEYBOARD_BUFFER: Mutex<RingBuffer> = Mutex::new(RingBuffer::new());

pub fn scancode_to_ascii(scancode: u8) -> Option<u8> {
    match scancode {
        0x02 => Some(b'1'),
        0x03 => Some(b'2'),
        0x04 => Some(b'3'),
        0x05 => Some(b'4'),
        0x06 => Some(b'5'),
        0x07 => Some(b'6'),
        0x08 => Some(b'7'),
        0x09 => Some(b'8'),
        0x0A => Some(b'9'),
        0x0B => Some(b'0'),
        0x0C => Some(b'-'),
        0x0D => Some(b'='),
        0x0E => Some(8),     
        0x0F => Some(b'\t'), 
        0x10 => Some(b'q'),
        0x11 => Some(b'w'),
        0x12 => Some(b'e'),
        0x13 => Some(b'r'),
        0x14 => Some(b't'),
        0x15 => Some(b'y'),
        0x16 => Some(b'u'),
        0x17 => Some(b'i'),
        0x18 => Some(b'o'),
        0x19 => Some(b'p'),
        0x1C => Some(b'\n'),
        0x1E => Some(b'a'),
        0x1F => Some(b's'),
        0x20 => Some(b'd'),
        0x21 => Some(b'f'),
        0x22 => Some(b'g'),
        0x23 => Some(b'h'),
        0x24 => Some(b'j'),
        0x25 => Some(b'k'),
        0x26 => Some(b'l'),
        0x2C => Some(b'z'),
        0x2D => Some(b'x'),
        0x2E => Some(b'c'),
        0x2F => Some(b'v'),
        0x30 => Some(b'b'),
        0x31 => Some(b'n'),
        0x32 => Some(b'm'),
        0x39 => Some(b' '),   
        _ => None,            
    }
}
