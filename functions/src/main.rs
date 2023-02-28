fn main() {
    println!("Hello from main");
    new_function_one();
    new_function_two(5);
}

// functions in Rust must use snakecase with all lowercase characters
fn new_function_one() {
    println!("Hello from new_function_one");
}

// functions can take parameters in the form of arguments upon the function call
fn new_function_two(x: i32) {
    println!("Value of argument provided as parameter: {}", x);
}
