#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel_tests::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;
mod kernel_tests;

/// panic handler function required from no_std attribute
use core::panic::PanicInfo;
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // should never return; returns never `!` type
    println!("Oops!: {}", info);
    loop {}
}

#[no_mangle]    // keep stack traces tidy
/// kernel entry
/// bootloader calls this function first with C's ABI
pub extern "C" fn _start() -> ! {
    // entry point
    // linker looks for function named `_start` by default
    println!("hullo :]");
    println!("{} plus {} is {}, minus {} that's {} quick maffs", 2, 2, 4, 1, 3);

    #[cfg(test)]
    test_main();

    loop {}
}

