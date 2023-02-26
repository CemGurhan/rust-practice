fn main() {
    // Shadowing allows us to create a new instance of a variable with the same name as before.
    // It allows us to assign new functionality to a variable
    let x = 22;
    let x = 23;
    // The value of a shadowed variable is scoped, such that once this inner scope ends, 
    // the variable shadowing the previous on from the outer scope is dropped and
    // the value returns back to what it was before the inner scope
    {
        let x = 23 * 2; 
        println!("The value of x in inner scope is: {}", x);
    }
    println!("The value of x in outer scope is: {}", x);
} 