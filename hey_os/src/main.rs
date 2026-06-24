#![no_std]
#![no_main]

use core::isize;
mod prelude;
mod conf;
mod vga;

static HELLO: &[u8] = b"Hey, welcome";

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let vga_buf = 0xb8000 as *mut u8;
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buf.offset(i as isize * 2) = byte;
            *vga_buf.offset(i as isize * 2 + 1) = 0xF;
        }
    }
    loop {}
}
