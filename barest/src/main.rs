// Strip main.rs of std and main

#![no_std]
#![no_main]

// Add a function named _start, as an extern C function
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

// Add panic function
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
