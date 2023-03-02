use std::ops::Range;

fn main() {
    if_expression_one(8);
    if_expression_one(1);
    else_if_expression(1);
    else_if_expression(4);
    else_if_expression(111);
    println!("returned variable from if expression output: {}", let_if());
    println!("returned content from `loop`: {}", loop_in_let_statement());
    loop_label();
    while_loop();
    for_loops();
}

fn if_expression_one(x: u32) {
    // if expressions allow for logic to be implemented depending on a condition.
    // Conditions should always be evaluated with booleans, Rust does not automatically turn numbers into booleans.
    // Logic that is ran after a condition is evaluated is done in an if expression 'arm'.
    if x > 7 {
        println!("condition was true with argument: {}", x);
    } else {
        println!{"condition was false with argument: {}", x};
    }
}

fn else_if_expression(x: u32) {
    // else if expressions can be useful for evaluating multiple conditions.
    // They wont evaluate any further conditions past the fast true codition however
    // and can get quite messy. Match can be useful as a cleaner alternative in that case
    if x > 7 {
        println!("else if first condition true with argument: {}", x);
    } else if x >= 3 {
        println!("else if second condition true with argument: {}", x);
    } else if x > 5 {
        println!("else if first condition true with argument: {}", x);
    } else {
        println!("no conditions true for argument: {}", x);
    }
}

fn let_if() -> i32 {
    // As `if` is an expression, it's outcome can be assigned to a variable in a let statement.
    // The final expression returned to the variable depends on the arm of the if expression accessed
    let return_from_if_expression = if true {5} else {6};
    return_from_if_expression
    // Each arm from the if expression must return an expression of the same type. If not, 
    // the compiler will throw an error. The compiler must be sure of a variables type to be sure
    // of it's value everywhere in the codebase. It cannot do this at runtime or if an if expression returns
    // mismatched types to a variable.
}

fn loop_in_let_statement() -> i32 {
    // `loop` can be used in Rust to infinetely cycle through code until told to stop.
    // You can use a `break` to back out of a loop. You can also assign the contents of the 
    // loop in a let statement after a `break`
    let mut x = 1;
    let content = loop {
        x = x + 1;
        if x == 5 {
            break x * 2;
        }
    };
    content
}

fn loop_label() {
    // you can assign a label to a loop in order to distinguish between different loops:
    'break_me : loop {
        loop {
            println!("about to break from outer loop with loop label 'break_me'");
            break 'break_me;
        }
    }
    println!("broken from outer loop with loop label 'break_me'");
}

fn while_loop() {
    // while loops allow you to have a loop function with some conditional logic
    let mut x = 5;
    while x > 2 {
        x = x - 1;
    }
    println!("broken out of conditional while loop as x has reached the value {}", x);
}

fn for_loops() {
    // `for` loops can be great for cycling via a range of values (such as an array)
    // safely. `for i in array` will pull out the values inside the array.
    let array: [i32; 5] = [2, -44, 21, 6, 88];
    for element in array {
        if element == 88 {
            println!("for loop halted at array value: {}", element);
        }
    }
    // you can start a for loop at a particular array index. Ranging over array.len() will
    // return `usize` indexes
    for i in 2..array.len() {
        println!("array value for index {} after starting at second index: {}", i, array[i]);
    }
    // you can use the `rev()` method to reverse over a set of values in a `for` loop. The `4`
    // wont be included in the range
    for i in (1 .. 4).rev() {
        println!("countdown at: {}", i);
    }
}