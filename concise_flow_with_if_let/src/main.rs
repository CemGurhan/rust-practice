fn main() {
    if_let();
}

fn if_let() {
    // When a match expression is too verbose, an `if let` 
    // can be useful as an alternative. `if let` is syntax 
    // sugar for a match expression that runs code only on
    // on particular pattern and ignores everything else.
    let config_max = Some(33);
    match config_max {
        Some(max) => println!("Max value is: {}", max),
        _ => ()
    };
    // Can be represented by an `if let` as such:
    if let Some(max) = config_max {
        println!("Max value is: {}", max);
    }
}
