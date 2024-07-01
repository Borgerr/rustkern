use crate::println;

#[cfg(test)]
/// Runs each kernel test.
/// Invoked only with `cargo test`.
///
/// since we don't link against the standard library,
/// we need to do a bit of plumbing from
/// test_main, to the test runner, which is this function.
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[test_case]
fn trivial_assertion() {
    println!("bow chica wow wow");
    assert_eq!(1, 1);
    println!("test passed!");
}

#[test_case]
fn no_pass() {
    println!("falls over and dies");
    assert_eq!(1, 2);
    println!("NOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOO");
}
