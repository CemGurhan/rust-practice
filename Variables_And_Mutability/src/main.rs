// Constants are valid for the entire time a program runs in the cope in which they are declared,
// making them useful as global variables that the entire program may need to know about
const GLOBAL_CONSTANT_EXAMPLE : &str = "I am a global constant";

fn main() {
    // VARIABLES:
    // variables are immutable by default, but can be made mutable with the `mut` keyword
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // CONSTANTS:
    // Constants are like variables in that they are immutable by default, but cannot be made muttable 
    // You must always declare a constants type
    // constants may be declared in any scope
    // Rusts naming convention states that constants must be all upper case with underscores between words
    const THREE_HOURS_IN_SECONDS : u32 = 3 * 60 * 60; 
    println!("Three hours in seconds: {}", THREE_HOURS_IN_SECONDS);
    println!("From Global: {}", GLOBAL_CONSTANT_EXAMPLE);
}
