#![no_std]
#![no_main]

use core::fmt::Write;
mod vga_buffer;

// panic handler function required
use core::panic::PanicInfo;
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // should never return; returns never `!` type
    println!("Oops!: {}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // entry point
    // linker looks for function named `_start` by default
    println!("hullo :]");
    println!("{} plus {} is {}, minus {} that's {} quick maffs", 2, 2, 4, 1, 3);
    //panic!();

    loop {}
}
