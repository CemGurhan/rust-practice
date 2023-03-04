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
    copies();

    let s = String::from("hello"); // s comes into scope
    take_ownership(s); // s is moved into this function
                       // s is no longer valid here 

    let x = 5; // x comes intos cope
    take_copy(x); // x is copied into this function
                  // x is still valid here
    function_return_ownership();
} // x comes out of scope. s also comes out of scope, but nothing special happens (like drop). 
 // x and s's shallow copied stack data are popped off the stack

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

fn copies() {
    // Why does the "clone" method not have to be called here?
    let x = 5;
    let y = x;
    println!("x's value is: {}\ny's value after copying x's value is: {}", x, y);
    // This is because x and y are simple scalar types (integers). As they 
    // exist only on the stack (fixed and immutable), implementing clone
    // for them wouldn't make sense, as a shallow and deep copy of their 
    // values is identical. This is due to them having no data stored on the heap.
    // It also doesn't make sense to invalidate x for this same reason.

    // Instead, the implement the "copy" trait, which performs a trivial copy
    // of the variable on the stack. This allows for a variable to be still
    // valid after assignment to another variable
}

fn take_ownership(s: String) { // a string comes into scope (s in our case)
    println!("s is now about to be dropped: {}", s);
} // s gets dropped after going out of scope. Memory is freed.

fn take_copy(x: i32) { // an i32 comes into scope (x in our case)
    println!("x is now about to go out of scope: {}",x);
} // x comes out of scope. Nothing special happens. Popped off stack.

fn function_return_ownership() {
    let s1 = String::from("hello"); // s1 comes into scope

    let s2 = take_and_give_back(s1); // s1 is moved into take_and_give_back()
                                             // the function then moves it's return value (s1) back into s2

} // s2 gets dropped. Nothing special happens to s1.

fn take_and_give_back(s: String) -> String { // a string comes into scope
    s // the string is moved back to the calling function
}
