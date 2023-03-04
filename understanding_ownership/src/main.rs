fn main() {
    // Three rules of Ownership in Rust:
    // 1) every value must have an owner
    // 2) there can be only one owner at a time
    // 3) when an owner goes out of scope, it's value is dropped
    scope();
    string_type();
    free_memory();
    copying_primitive_complex();
    moving_values();
    clones();
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

fn free_memory() {
    // Rust will automatically return memory used by a complex variable type
    // back to the allocator once the variable goes out of scope. A function 
    // called `drop` is automatically called at the curly bracket.
    // We request memory from the allocator 
    {
    let s = String::from("some memory");
    println!("the String '{}' is about to return its memory used back to the allocator", s);
    }
}

fn copying_primitive_complex() {
    // When we assign a primitive data type, like an int, to another
    // variable, it's value gets copied off the stack and assigned to the 
    // new variable. In this example, we see 2 values of 5 get pushed on
    // the stack as x is copied and assigned to y
    let x = 5;
    let _y = x;

    // When we assign a complex type, like the String type, to another variable,
    // it's data `(which consists of a pointer that points to the allocated memory
    // on the heap, it's length, which is the amount of memory in bytes it is using and 
    // it's capicity, which is the amount of memory allocated to it from the allocator) 
    // is copied instead. Both pieces of data will point to the same point of memory
    // in the heap
    let s1 = String::from("hello");
    let _s2 = s1;
}

fn moving_values() {
    // as s1 and s2 both point to the same location in heap memory,
    // when they go out of scope, they will both try to free the same memory,
    // leading to a double free error. Instead, Rust will consider s1 no longer
    // valid after it is assigned to s2, and will hence not need to free
    // s1 when it goes out of scope.
    let s1 = String::from("hello");
    let _s2 = s1;
    // As the pointer, length and data for s1 is copied to s2 and the value of s1 is
    // invalidated, we call this a move. s1's value was moved to s2.

    // s2 alone will now free memory once it goes out of scope, as s1 is now invalid.
}

fn clones() {
    // You can choose to deeply copy a variable instead of moving it.
    // A deep copy can be achieved using the clone method, which will 
    // copy your stack and heap data (as opposed to just stack data with
    // invalidation during a move)
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("original s1: {} \ns1 cloned into s2: {}", s1, s2);
    // This is more expensive than using a move
}
