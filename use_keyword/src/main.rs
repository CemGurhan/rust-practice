mod front_of_house;

// It is idiomatic to bring in a struct, enum or other type
// with the full path
use std::collections::HashMap;
// if two items have the same name, then it is idiomatic to 
// specify the full path
use std::io;
use std::fmt;
// You can also use the `as` keyword to create a new name or
// 'alias' for the type. This is also considered idiomatic
use std::io::Error;
use std::fmt::Error as FmtError;

fn an_example_of_idiomatic() {
    let r1 = io::Result::Ok(());
    let r2 = fmt::Result::Ok(());
}

use front_of_house::hosting;

// We can use `pub use` to re-export an item. This makes it
// seem as if the module `back_rooms` was not defined
// in `hosting`, but instead `front_of_house`
use front_of_house::back_rooms;

// We can use `nested paths` to bring multiple items from the 
// same crate together into one `use` line. For example:
// use std::cmp::Ordering;
// use std::arch;
// Can become:
use std::{arch, cmp::Ordering};

// `nested paths` can also be used when importing items from the
// same path:
// use std::backtrace;
// use std::backtrace::Backtrace;
// Can be written as:
use std::backtrace::{self, Backtrace};

// We can use a `glob operator` to import everything from a path.
use std::collections::*;
// This can be bad however, as we have no idea where an item in this scope 
// was defined, or what names are in scope.



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

    back_rooms::enter_the_backrooms();

    
}
