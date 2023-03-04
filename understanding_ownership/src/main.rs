fn main() {
    // Three rules of Ownership in Rust:
    // 1) every value must have an owner
    // 2) there can be only one owner at a time
    // 3) when an owner goes out of scope, it's value is dropped
    scope();
    string_type();
}

fn scope() {
    // a scope is the range in which a value is valid in a Rust program
       { // s is not valid here or before here
        let s = "am I valid?"; // s is valid here onwards as it comes into scope
        println!("{} Yes!", s);        
    }   // s is not valid from this point on 
}

fn string_type() {
    // String literals are data types that can be stored on the stack.
    // Their size is known at compile time and is fixed as they are immutable.
    // The String type is instead allocated to the heap. It's size is
    // unknown at compile type and is mutable. 
    // It is a complex data type.

    // String type can be mutated
    let mut string_type = String::from("hello");
    string_type.push_str(", world!");
    println!("{}",string_type);

}
