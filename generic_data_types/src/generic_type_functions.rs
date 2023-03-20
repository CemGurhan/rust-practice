// We can have generic types present in a function
// signature or return, which can help add flexibility 
// to callers of a function. 
// Generic types are often named with shortened words,
// and are often declared as a single letter. Types in
// Rust are declared with CamelCase by default.
// We must declare the generic type before the list of
// parameters in the function signature
// Here, our function can take in a slice containing
// the generic type `T`
// We have to only accept generic types that implement
// the `std::cmp::PartialOrd` trait, as you can only compare
// values that can be ordered on line 17
pub fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}