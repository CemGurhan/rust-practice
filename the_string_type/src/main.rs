fn main() {
    create_string();
    updating_a_string();
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
