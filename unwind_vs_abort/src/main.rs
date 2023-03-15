fn main() {
    lets_panic();
}

// In rust, there are two types of errors:
// 1. recovrable errors - these are errors that can
//    just be rreported to the user and have the 
//    operation that caused the error to be retried.
//    They don't stop the program from running.
// 2. unrecoverable errors - these are are errors
//    that cause the program to stop immedietely, 
//    and are results from bugs.
// 
// In Rust, unrecoverable errors can be caused by panics.
// A panic can happen for two reasons:
// 1. your program has panicked, because of an action you
//    took, such as when you are trying to access an out of range
//    of range index in an array.
// 2. The panic! macro was explicitly called
//
// panics will cause the program to print a message, unwind, clean
// the stack and quit.
// Unwinding the stack is a procsess where Rust walks back up the
// stack and cleans up any data that associated to functions it finds.
// This process can be pretty demanding, so instead rust offers an 
// alternative known as `abort`. 
// Aborting allows you to panic without cleaning up data your program
// was using, this will instead be handled by your operating system.
// An example of how to convert from the default behaviour of unwinding
// on a panic to aborting is shown in Cargo.toml.

fn lets_panic() {
    panic!("Ive panicked")
}
