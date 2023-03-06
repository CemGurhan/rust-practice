fn main() {
    string_slices();
    let string1 = String::from("hello");
    
    // Rust now knows this reference's lifetime begins here as it is passed into the function
    let string1_slice = returning_string_slice(&string1); 
    println!("&str returned from function: {}",string1_slice); 
    immutable_mutable_error();
    string_literal_immutable();

    let string_type = String::from("hello"); // a reference to a String type is basically an entire slice of that string
    let string_literal = "hello";
    improved_signature(&string_type);
    improved_signature(string_literal);
    improved_signature(&string_type[..3]);
    improved_signature(&string_literal[1..]);
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

// If we uncomment out the `println!("the first word is: {}", word);` here,
// we get a compilation error. 
//
// This is because the function `returning_string_slice` creates an
// immutable reference to the String `s` and stores it in the variable
// `word`. 
//
// If we now try to use the print statement after, the print statement is 
// going to need the immutable reference `word`. However, if the variable
// `s` that this immutable reference `word` refers to is cleared before this,
// `word` has nothing to reference or borrow. Hence, we get a compilation error
fn immutable_mutable_error() {
    let mut s = String::from("hello world");

    let word = returning_string_slice(&s);

    s.clear(); // error!

    // println!("the first word is: {}", word); 
}

// string literals are stored inside the binary. Hence, they are just
// slices that point to that point in binary. Their type is &str, which
// is an immutable reference. This is therefore why string literals are immutable
fn string_literal_immutable() {
    let _string_literal = "Im immutable";
}

// Taking in the &str type over &String type as a parameter to a function is better.
// This is because the &str type can cover more types of potential arguments.
// e.g you can pass a reference to a string &String, as this counts as an entire slice
// of the String. You can pass in a string literal, as this is already a slice.
fn improved_signature(s: &str) -> &str {
    let whole_s = &s[..];
    whole_s
}