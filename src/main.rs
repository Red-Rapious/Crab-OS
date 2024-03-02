#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crab_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use crab_os::{print, println};

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    crab_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    crab_os::test_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    crab_os::init();

    print!("> ");

    use x86_64::registers::control::Cr3;

    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at {:?}", level_4_page_table.start_address());

    #[cfg(test)]
    test_main();

    crab_os::hlt_loop();
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
