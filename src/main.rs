#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler] // ! is the never return type
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // ! is the never return type
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        //!("we could create a VGA buffer type that encapsulates all unsafety and ensures that it is impossible to do anything wrong from the outside. ");
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
// we can't use unwinding as it is dependent on some std os features
// language features have no type checking.


