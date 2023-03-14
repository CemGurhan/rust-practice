fn main() {
    vector_defines();
    adding_to_vectors();
    receiving_from_vectors();
}


fn vector_defines() {
// A vector can be used to store values next to
// each other in memory. You can instantiate an empty
// vector, but must define which type of data it holds,
// as Rust wont be able to infer from an empty Vector
let v: Vec<i32> = Vec::new();

// You can also instantiate a vector with preset values
// inside using the `vec!` macro. This way, you also
// don't have to define the type stored inside the Vec
let v1 = vec![0,33,-10,4,1];
}

fn adding_to_vectors() {
    // We must make the vec mutable, as all variables in
    // Rust are immutable by default. As we are pushing an
    // int, we dont need to define the type of the Vector.
    let mut v = Vec::new();
    v.push(2);
}

fn receiving_from_vectors() {
    let v = vec![3,4,22,1,-10];
    // We can take values of a vector using either it's index value,
    // or by using the `get` method
    let third = &v[2];
    println!("via index: {}", third);
    
    let third = v.get(2);
    match third {
        Some(v) => println!("via get: {}", v),
        None => println!("Nothing returned via get!")
    };
}

fn using_enums_in_vectors() {
    // we can use enums to store values of different data types in a vector
    enum Values {
        Integer(i32),
        Text(String),
        Float(f64),
    }
    let v = vec![Values::Integer(2), 
                              Values::Text(String::from("Hello")), 
                              Values::Float(2.1)];
}