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
    
    mutable_reference_data_race();
} 

fn reference_arg_func(s: &String) -> usize { // s is a reference to a String. In our case this String is s1
    s.len() 
} // As s was only a reference, the value of s1 does not have to be dropped here. This is because we never 
  // had ownership of this value, we just had a pointer to it's representation on the stack. 

fn mutable_reference(s: &mut String) {
    s.push_str(", world");
} 

fn mutable_reference_data_race() {
    // Once you have a mutable reference to a value,
    // you no longer can have any other references to it
    // simultaneously 
    // let mut s = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{}, {}", r1, r2);
    // This compilation error is used to prevent a data race.
    // We can prevent this data race from occuring by declaring
    // a new scope, such that we create mutabke references to our value,
    // just not all at once
    let mut s3 = String::from("hello");
    {
        let r1 = &mut s3;
        println!("value of our mutably borrowed reference to s3 in new scope: {}", r1);
    } // r1 goes out of scope here, sucht at we can now make another mutable reference to s3
    let r2 = &mut s3;

    println!("value of our mutably borrowed reference to s3 in orginal scope: {}", r2);

    // we also cannot have immutable references to a variable at the same time as
    // mutable references. This is because we don't want the value the immutable
    // reference has borrowed from to change
    // let mut s4 = String::from("hello");
    // let r11 = &s4; 
    // let r22 = &s4; 
    // let r33 = &mut s4; 

    // println!("{},{},{}", r11, r22, r33);
}
