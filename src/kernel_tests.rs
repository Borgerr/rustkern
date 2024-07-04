use crate::{exit_qemu, serial_print, serial_println, QemuExitCode};

#[cfg(test)]
/// Runs each kernel test.
/// Invoked only with `cargo test`.
///
/// since we don't link against the standard library,
/// we need to do a bit of plumbing from
/// test_main, to the test runner, which is this function.
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("test passed!");
    }
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
