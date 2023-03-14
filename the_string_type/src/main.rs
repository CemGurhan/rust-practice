fn main() {
    create_string();
    updating_a_string();
    string_concatenation();
}

// The String type has many similar properties to a vector,
// as it acts as a wrapper around a Vector of bytes. 
fn create_string() {
    let mut s1 = String::new();
    s1 = "hi".to_string();
    // we can condense this down with the String::from() method
    {
        let _s1 = String::from("hi");
    }
}

fn updating_a_string() {
    // We can update a string using `push_str`
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("Mutated string with `push_str`: {}", s);
    
    // We can update a string with a single character using
    // `push`
    let mut s2 = String::from("by");
    s2.push('e');
    println!("Mutated string with `push`: {}", s2);
}

fn string_concatenation() {
    // You can add together two String's with the `+` operator.
    // This operator calls the `add` function, which takes ownership
    // of the string you are adding to, takes a reference of the 
    // String's data you are adding with and returns ownership to 
    // the result. You cannot concatenate two String's, hence, a 
    // reference is required. 
    let s1 = String::from("foo");
    let s2 = String::from("bar");
    let s3 = s1 + &s2; // s1 no longer valid from this point on
    println!("concatenated with the `+` operator: {s3}");
    // Rust can use `deref coercian` to coerce the type &String
    // to &str when using &s2.

    // If you don't want ownership to be a problem and you
    // want a neater representation of multiple strings added together,
    // you can use the `format!` macro
    let st1 = String::from("hi");
    let st2 = String::from("hello");
    let st3 = String::from("greetings");
    let st4 = format!("Different ways to greet: {st1}, {st2} and {st3}");
    println!("{}",st4);
}

fn iterating_over_strings() {
    for c in "Зд".chars() {
        println!("{c}");
    }
    // each unicode scalar value for these non-ASCII
    // types represent 2 bytes after UTF-8 encoding
    for b in "Зд".bytes() {
        println!("{b}");
    }
}
