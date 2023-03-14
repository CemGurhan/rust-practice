fn main() {
    create_string();
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
