#![no_std]
#![no_main]

// Add a function named _start, as an extern C function
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World! This is just a quick illustration";

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Colour {
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

// Add panic function
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let framebuffer = 0xb8000 as *mut u8;
    /*unsafe {
    framebuffer.offset(1).write_volatile(0x30);
    }*/
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *framebuffer.offset(i as isize * 2) = byte;
            // *framebuffer.offset(i as isize * 2 + 1) = 0xb;
            *framebuffer.offset(i as isize * 2 + 1) = Colour::LightGreen as u8;
        }
    }
    loop {}
}
