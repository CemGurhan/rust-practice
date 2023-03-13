mod front_of_house;

// It is idiomatic to bring in a struct, enum or other type
// with the full path
use std::collections::HashMap;
// if two items have the same name, then it is idiomatic to 
// specify the full path
use std::io;
use std::fmt;

fn an_example_of_idiomatic() {
    let r1 = io::Result::Ok(());
    let r2 = fmt::Result::Ok(());
}

use front_of_house::hosting;

fn main() {
    // It is idiomatic to not use the full path to a function.
    // This is because we would want to know where our function
    // is defined and if it is a part of the scope. Without the 
    // full path, we'll end up with an ambigous call:
    //
    // use front_of_house::hosting::add_to_waitlist;
    //
    // add_to_waitlist();
    hosting::add_to_waitlist();
}
