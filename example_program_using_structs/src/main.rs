fn main() {
    let rectangle1 = Rectangle { width: 10, height: 30 };

    println!("Rectangle area: {}", calculate_area(&rectangle1));

    let rectangle2 = (200, 10);

    println!("Rectangle area with: {}", calculate_area_tuple(rectangle2));
    debug_display();
}

struct Rectangle {
    width: u64,
    height: u64,
}

// Using a rectangle struct there helps group together
// our length and width values together descriptively.
// This can be very helpful when attempting to better understand the code,
// as we can accurately convey the relationship between the width and height values

// Taking an immutable reference to the rectangle allows us to 
// call upon it's fields without moving it's values
// Taking an immutable reference also allows us to continue using
// the Rectangle struct in main.
fn calculate_area(rectangle: &Rectangle) -> u64 {
    rectangle.height * rectangle.width
}

// using the Rectangle struct is much clearer and more descriptive
// than using a tuple. With tuples we can't name fields, so have
// to do our best to understand what they represent:
fn calculate_area_tuple(tuple: (u32, u32)) -> u32 {
    tuple.0 * tuple.1
}

fn debug_display() {
    // You cannot print out a struct in the following way:
    // println!("{}", struct);
    // This is because structs do not implement `Display`.
    // `Display` is the default output type for the `println!`
    // macro once it uses the `{}` placeholder.
    // There are multiple ways one could format the display
    // of a struct and Rust would not be able to guess what
    // the user wants.

    // Instead, you can use the `Debug` trait, which can be 
    // used with the `println!` macro with the placeholder
    // `{:?}`. In order to use a struct with this trait,
    // you need to add the attribute `#[derive(Debug)]` once
    // it's instantiated.

    #[derive(Debug)]
    struct Hobbit {
        name : String,
        surname: String,
        age: u32
    }

    let hobbit1 = Hobbit {
        name : String::from("Bilbo"),
        surname : String::from("Baggins"),
        age : 111
    };

    println!("Printing struct with the `Debug` trait: {:?}", hobbit1);

    // We can make the format nicer by using the following placeholder
    // {:#?}
    println!("Printing struct prettier with the `Debug` trait: {:#?}", hobbit1);
}

fn debug_macro() {
    // We can use the `dbg!` macro when we want to utilize the 
    // `Debug` format. It will take ownership of an expression, 
    // print out the file in which it was called, the line number,
    // and then return ownership. 
    // It prints to the standard error stream `stderr` as opposed
    // to the standard output stream `stdout`.
    #[derive(Debug)]
    struct Hobbit{
        name : String,
        surname : String,
        age : u32
    }

    let multiplier = 2;

    let hobbit2 = Hobbit {
        name : String::from("Frodo"),
        surname : String::from("Baggins"),
        age : dbg!(25 * multiplier)
    };

    dbg!(&hobbit2);
}