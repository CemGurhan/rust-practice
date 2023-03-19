use std::fs::File;

// We can use `unwrap` to handle a `Result<T,E>`. It will return the 
// contents of an `Ok` directly. If an `Err` is returned, `panic!` is
// automatically called instead. This is far more concise than using 
// match expressions.
fn _unrwap_example() {
    let _greeting_file = File::open("hello.txt").unwrap();
}