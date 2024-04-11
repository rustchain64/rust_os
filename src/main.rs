#![no_std]
#![no_main]

mod vga_text_buffer;

use core::panic::PanicInfo;

#[panic_handler] // ! is the never return type
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;

    vga_text_buffer:: WRITER.lock().write_str("Hello Again").unwrap();
    write!(vga_text_buffer::WRITER.lock(),
    ", some numbers: {} {}",
     42, 1.337
    )
    .unwrap();

    loop {}
}


