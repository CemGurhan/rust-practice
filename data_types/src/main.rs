fn main() {
    // If running on debug mode, Rust will check for integer overflow and panic.
    // If on release mode, this check will not occur. Instead, two's
    // complement wrapping will occur, causing the value of the overflowed integer
    // to wrap around the minimum value it can take. E.g a u8 with 256 asigned to it will
    // be transfered to 0, 257 to 1 and so on.
    // You can use various methods from the std lib to account for this.
    
}
