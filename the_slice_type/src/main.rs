fn main() {
    string_slices();
    let string1 = String::from("hello");
    let string1_slice = returning_string_slice(&string1); // Rust now knows this reference's lifetime begins here as it is passed into the function
    println!("&str returned from function: {}",string1_slice);
}

fn string_slices() {
    // A slice is a reference to a contigous sequence of elements
    // in a collection, rather than the whole collection itself.
    // A string slice is a reference to a part of a String.
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    // notice how we reference s with an ampersand & when we are 
    // creating our slices. The slice borrows from a part of the 
    // string "hello world"
    println!("both slices together: {} {}", hello, world);
}
// The type of a String slice is &str, as can be seen from 
// `returning_string_slice`'s return
// With this function, the argument must be a reference to a String.
// This is because we create a reference to a part of the String in
// the function itself. Hence, when the function returns, Rust won't 
// know where the lifetime of the reference to this part of the String 
// began (where the reference came into scope).
fn returning_string_slice(s: &String) -> &str {
    let slice = &s[..];
    slice
}