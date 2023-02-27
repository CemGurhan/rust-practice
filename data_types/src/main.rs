fn main() {
    // If running on debug mode, Rust will check for integer overflow and panic.
    // If on release mode, this check will not occur. Instead, two's
    // complement wrapping will occur, causing the value of the overflowed integer
    // to wrap around the minimum value it can take. E.g a u8 with 256 asigned to it will
    // be transfered to 0, 257 to 1 and so on.
    // You can use various methods from the std lib to account for this.

    // You can use `checked_add` to decide what occurs if integer overflow occurs after an addition. 
    // If overflow occurs, a `None` is returned.
    let first_int: u8 = 254;
    let second_int: u8 = 1;

    let no_overflow_checked_add = first_int.checked_add(second_int);
    println!("Output of checked_add without overflow is {:?}", no_overflow_checked_add);

    let overflow_checked_add = no_overflow_checked_add.unwrap().checked_add(22);
    println!("Output of checked_add with overflow is: {:?}", overflow_checked_add);
}
