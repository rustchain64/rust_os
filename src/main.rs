#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler] // ! is the never return type
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // ! is the never return type
pub extern "C" fn _start() -> ! {
    loop {}
}
// we can't use unwinding as it is dependent on some os features
// language features have no type checking.


