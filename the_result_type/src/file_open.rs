use std::fs::File;

pub mod propagate_error;

// We can use the `Result<T, E>` enum to encode the scenario
// where an operation is successful or fails.
// The `Result<T, E>` enum has two vairants, `Ok(T)` and
// `Err(E)`. The generic type parameter `T` represents the 
// value returned from a successful operation. `E` represents
// an error returned from an unsuccesful operation.
fn _open() {
    let greeting_file_result = File::open("hello.txt");

    // We can match and return a successful result to a variable using
    // a `match` expression, or do soemthing in the case of an error
    let _greeting_file = match greeting_file_result {
        Ok(file) => file, // return file handle to `greeting_file` variable
        Err(error) => panic!("Error {:?}", error), // call `panic!` macro on error
    };
}

fn _open_propagte_error() {

}