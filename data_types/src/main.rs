use std::any::Any;

fn main() {
    // If running on debug mode, Rust will check for integer overflow and panic.
    // If on release mode, this check will not occur. Instead, two's
    // complement wrapping will occur, causing the value of the overflowed integer
    // to wrap around the minimum value it can take. E.g a u8 with 256 asigned to it will
    // be transfered to 0, 257 to 1 and so on.
    // You can use various methods from the std lib to account for this.

    // You can use `checked_add` to decide what occurs if integer overflow occurs after an addition. 
    // If overflow occurs, a `None` is returned.
    let first_int: u8 = 254;
    let second_int: u8 = 1;

    let no_overflow_checked_add = first_int.checked_add(second_int);
    println!("Output of checked_add without overflow is {:?}", no_overflow_checked_add);

    let overflow_checked_add = no_overflow_checked_add.unwrap().checked_add(22);
    println!("Output of checked_add with overflow is: {:?}", overflow_checked_add);

    // Integer division in Rust is truncated to zero to the nearest integer
    let quotient = -5.0/3.0;
    let truncated = -5/3;

    println!("Quotient value: {} | Truncated value: {}", quotient, truncated);

    // Values in a tuple can be accessed via their indexes
    let tup = (22,"hello", 3.3);
    println!("First value in tuple is {}", tup.0);

    // Tuples can be destructered into new variables using pattern matching
    let (x, y, z) = tup;
    println!("Tuple destructered: {}, {}, {}", x, y, z);

    // Arrays can be instantiated with square brackets and comma seperated values.
    // Values must all be of the same type, and arrays cannot change size after declaration.
    let array1 = [1,2,3,4,5];
    println!("First value of array 1: {}",  array1[0]);
    // Arrays can be typed as follows
    let array2 : [&str; 4] = ["1","2","3","4"];
    println!("Third value of array 2: {}", array2[2]);
    // You can instantiate arrays of one value repeated multiple times 
    let array3 = [3; 5];
    println!("Array 3: {:?}", array3);
}
