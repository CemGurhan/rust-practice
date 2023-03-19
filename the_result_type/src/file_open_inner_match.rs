use std::fs::File;
use std::io::ErrorKind;


// We can execute a certain operation based on specific errors, as
// opposed to panicking on any error. We can do this with inner
// `match` expressions:
fn _create() {
    let greeting_file_result = File::open("hello.txt");

    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.tx") {
                Ok(fc) => fc,
                Err(error) => panic!("error creating file {:?}", error),
            }
            other_error => panic!("error opening file: {:?}", other_error)
            }
        };
}

// We can instead use closures, as they are more concise than these inner `match` 
// expressions
fn _closure_create() {
    let _greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Unable to create file: {:?}", error);})
            } else {
                panic!("Unable to open file: {:?}", error);
            }
        }
    );
}
