#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(kev_os::test_runner)]

use core::panic::PanicInfo;

use kev_os::println;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    // use core::fmt::Write;

    // vga_buffer::WRITER
    //     .lock()
    //     .write_str("Hello stranger! Welcome to the brand new KevOS, your new home :)")
    //     .unwrap();

    println!("Hello stranger! Welcome to the brand new KevOS, your new home :)");

    kev_os::init(); // initialize interrupts

    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    use kev_os::print;

    print!("{}", _info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    kev_os::test_panic_handler(_info)
}
