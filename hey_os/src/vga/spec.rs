#[derive(Debug, Clone)]
#[repr(u8)]
pub enum Color {
    //iRGB
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    pub fn new(f: Color, b: Color) -> Self {
        Self((b as u8) << 4 | (f as u8))
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Character {
    char: u8, //ascii
    code: ColorCode,
}

impl Character {
    pub fn new(char: u8, code: ColorCode) -> Self {
        Self { char, code }
    }
}

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
pub struct Buffer {
    pub chars: [[Character; BUFFER_WIDTH]; BUFFER_HEIGHT], //prealloc..
}
