fn main() {
    // A reference is like a pointer, in that it is an address
    // that we can follow to get the data stored at that address.
    // It is different to a pointer as the data it points to
    // is owned by another variable. A reference in rust can 
    // be represented with an ampersand '&'

    // Borrowing is the action of creating a reference. In the folowing code,
    // we are essentially borrowing the value of s1  by creating a reference to it, 
    // and passing it to the func 'reference_arg_func'
    let s1 = String::from("hello");
    let len = reference_arg_func(&s1);
    println!("length of string '{}' is: {}", s1, len);

    // references are immutable by default. In order to change a borrowed value
    // you must utilize a mutable reference to that value instead.
    let mut s2 = String::from("hello");
    mutable_reference(&mut s2);
} // s1 is dropped here

fn reference_arg_func(s: &String) -> usize { // s is a reference to a String. In our case this String is s1
    s.len() 
} // As s was only a reference, the value of s1 does not have to be dropped here. This is because we never 
  // had ownership of this value, we just had a pointer to it's representation on the stack. 

fn mutable_reference(s: &mut String) {
    s.push_str(", world");
} 
