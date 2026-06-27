use core::usize;

use crate::vga::{self, spec::{self, Buffer, Character, ColorCode}};



pub struct Writer{
    column_positon: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer
}

impl Writer{

    pub fn new(col: usize, color_code: ColorCode) -> Writer{
        Writer { 
            column_positon: col, color_code,
            buffer: unsafe {&mut *(0xb8000 as *mut Buffer)}
        }
    }

    fn new_line(&mut self){}

    pub fn write_bytes(&mut self, byte: u8){
        match byte{
            b'\n' => self.new_line(),
            b => {
                if self.column_positon >= spec::BUFFER_WIDTH{
                    self.new_line();
                }
                let row = spec::BUFFER_HEIGHT - 1; //last row
                let col = self.column_positon;
                self.buffer.chars[row][col] = Character::new(b, self.color_code);
                self.column_positon += 1
            }
        }
    }

    pub fn print(&mut self, s: &str){
        for b in s.bytes(){
            match b{
                0x20..=0x7e => self.write_bytes(b), //ascii
                b'\n' => self.write_bytes(b),
                _ => self.write_bytes(0xfe),
            }
        }
    }
}
