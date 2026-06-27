#![no_std]
#![no_main]

use crate::vga::{spec::{Color, ColorCode}, writer::Writer};
mod prelude;
mod vga;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let mut writer = Writer::new(0, ColorCode::new(Color::Blue, Color::LightGreen));
    writer.print("Hey, welcome");
    loop {}
}
