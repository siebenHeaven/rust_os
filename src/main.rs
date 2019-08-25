#![no_std]
#![no_main]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]

use core::panic::PanicInfo;
use rust_os::{println,print};

#[cfg(not(test))] // new attribute
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// our panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello world{}", "!");
    
    rust_os::init();
/*
    // trigger a page fault
    unsafe {
        *(0xdeadbeef as *mut u64) = 42;
    };
*/

/*
    fn stack_overflow() {
        stack_overflow();
    }

    stack_overflow();
*/
    #[cfg(test)]
    test_main();

    println!("Works!");
    loop {
    	print!("-");
    }
}

