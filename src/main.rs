#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{BootInfo, entry_point};
use rust_os::println;
use core::panic::PanicInfo;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
   //use rust_os::memory::active_level_4_table;
    // we drop our own translate_addr in memory module
    use rust_os::memory;
    use x86_64::VirtAddr;
    //use x86_64::structures::paging::PageTable;

    println!("Hello World!");    

    // call init within lib.rs on start
    rust_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut _mapper = unsafe { memory::init(phys_mem_offset)};
    let mut _frame_allocator = 
    unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };
    
    #[cfg(test)]
    test_main();

    println!("It did not crash!");

    rust_os::hlt_loop(); 
}

/// Called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rust_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
