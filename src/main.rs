#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crab_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use crab_os::{
    allocator,
    memory::{self, BootInfoFrameAllocator},
    print, println,
};

entry_point!(kernel_main);

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

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use x86_64::VirtAddr;

    println!("Hello Hubert!");

    crab_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    let x = Box::new(41);
    println!("heap_value of {x} at {:p}", x);

    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vet ac {:p}", vec.as_slice());

    let reference_counted = Rc::new(vec![1, 2, 3]);
    let clone_reference = reference_counted.clone();
    println!(
        "current reference count is {}",
        Rc::strong_count(&clone_reference)
    );
    core::mem::drop(reference_counted);
    println!(
        "reference count is {} now",
        Rc::strong_count(&clone_reference)
    );

    #[cfg(test)]
    test_main();

    crab_os::hlt_loop();
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
