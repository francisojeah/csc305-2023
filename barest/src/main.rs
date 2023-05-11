#![no_std]
#![no_main]

// Add a function named _start, as an extern C function
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World! This is just a quick illustration";

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
            *framebuffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}
