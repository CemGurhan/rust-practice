fn main() {
    string_slices();
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