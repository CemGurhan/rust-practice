fn main() {
    if_statment_one(8);
    if_statment_one(1);
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