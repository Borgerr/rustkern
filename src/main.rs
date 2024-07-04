#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel_tests::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;
mod serial;

#[cfg(test)]
mod kernel_tests;

#[no_mangle]    // keep stack traces tidy
/// kernel entry
/// bootloader calls this function first with C's ABI
pub extern "C" fn _start() -> ! {
    // entry point
    // linker looks for function named `_start` by default
    println!("hullo :]");
    println!("{} plus {} is {}, minus {} that's {} quick maffs", 2, 2, 4, 1, 3);

    #[cfg(test)]
    serial_println!("Running tests...");
    #[cfg(test)]
    test_main();

    loop {}
}

/// panic handler function required from no_std attribute
use core::panic::PanicInfo;
#[cfg(not(test))]   // want to print inside qemu instance when not in test
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("falls over and dies: {}", info);
    loop {}
}

/// panic handler function required from no_std attribute
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("falls over and dies: {}\n", info);
    exit_qemu(QemuExitCode::Failure);
    loop {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failure = 0x11,
}

use x86_64::instructions::port::Port;
pub fn exit_qemu(exit_code: QemuExitCode) {
    unsafe {
        let mut port = Port::new(0xf4); // isa-debug-exit iobase
        port.write(exit_code as u32);
    }
}

