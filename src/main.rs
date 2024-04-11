#![no_std]
#![no_main]

mod vga_text_buffer;

use core::panic::PanicInfo;

#[panic_handler] // ! is the never return type
fn panic(info: &PanicInfo) -> ! {
    println!("{info}");
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, World{}", "!!!");
    panic!("Some panic happened");
    
    loop {}
}


