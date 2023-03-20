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
pub fn take_a_list<T>(list: &[T]) -> &T {

}