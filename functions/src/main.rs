fn main() {
    println!("Hello from main");
    new_function_one();
    new_function_two(5);
    new_function_three();
}

// functions in Rust must use snakecase with all lowercase characters
fn new_function_one() {
    println!("Hello from new_function_one");
}

// functions can take parameters in the form of arguments upon the function call.
// Parameters must always contain a type.
fn new_function_two(x: i32) {
    println!("Hello from new_function_two. Value of argument provided as parameter: {}", x);
}

fn new_function_three() {
    // Statements are a set of instructions that perform an action and do not return a value.
    // Expressions return a value.
    let _y = 5;
    // let y = 5; is an expression. You cannot assign let y = 5 to another let statement:
    // ley x = (let y = 5); as it returns no value. There is an expression in the let statement
    // in the form of the number 5. Stating the number 5 causes it to be returned to the variable y.

    // expressions can come in many forms, such as a function call, a macro call or a scope declaration:
    let z = {
        let x = 3;
        x + 3
    };
    // notice how the scope does not contain an ending semi-colon, allowing for the x + 3 value to be returned
    // from the expression to the `z` variable.
    
    println!("Hello from new_function_three: {}", z);
}