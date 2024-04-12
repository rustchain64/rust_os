#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
//#![test_runner(crate::test_runner)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rust_os::println;

#[no_mangle] // don't mangle the name of this function, also test is outside main
pub extern "C" fn _start() -> ! {
    test_main();

    #[allow(clippy::empty_loop)]
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}

/*
    By testing println in a basic_boot environment without 
    calling _start init routines, 
    we affirm println works right at boot.
*/

#[test_case]
fn test_println() {
    println!("test_println output");
}