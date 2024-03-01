#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crab_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use crab_os::println;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    crab_os::test_panic_handler(info)
}


#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World!");

    crab_os::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    loop {}
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
