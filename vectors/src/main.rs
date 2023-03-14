fn main() {
    vector_defines();
    adding_to_vectors();
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
