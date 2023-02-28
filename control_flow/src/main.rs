fn main() {
    if_statment_one(8);
    if_statment_one(1);
    else_if_statement(1);
    else_if_statement(4);
    else_if_statement(111);
}

fn if_statment_one(x: u32) {
    // if statements allow for logic to be implemented depending on a condition.
    // Conditions should always be evaluated with booleans, Rust does not automatically turn numbers into booleans.
    // Logic that is ran after a condition is evaluated is doen in an if statement 'arm'.
    if x > 7 {
        println!("condition was true with argument: {}", x);
    } else {
        println!{"condition was false with argument: {}", x};
    }
}

fn else_if_statement(x: u32) {
    // else if statements can be useful for evaluating multiple conditions.
    // They wont evaluate any further conditions past the fast true codition however
    // and can get quite messy. Match statements can be useful as a cleaner alternative in that case
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