fn main() {
    if_expression_one(8);
    if_expression_one(1);
    else_if_expression(1);
    else_if_expression(4);
    else_if_expression(111);
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