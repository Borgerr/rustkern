use uart_16550::SerialPort; // represents UART registers
use spin::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

use core::fmt::Write;
#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    SERIAL1.lock().write_fmt(args).expect("Printing to serial failed");
}

// macros for printing to console connected by serial
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
            concat!($fmt, "\n"), $($arg)*
    ));
}

