use std::fs::File;

// We can use `expect` to handle a `Result<T, E>`. If an `Ok` is returned,
// `expect` will return the contents of the `Ok`. If an `Err` is returned,
// we can pass a message to `expect` to have the `panic!` call print a 
// custom message
fn _expect_example() {
    let _greeting_file = File::open("hello.txt")
    .expect("The file 'hello.txt' should be prsent in this project.");
}
// `expect` is what should be used in production instead of `unwrap`.
// It is more descriptive as to what the context of the code you are 
// always expecting to succeed is.