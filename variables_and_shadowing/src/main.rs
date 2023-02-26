fn main() {
    // Shadowing allows us to create a new instance of a variable with the same name as before.
    // It allows us to assign new functionality to a variable whilst also retaining it's immutability
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

    // We can use variable shadowing to change the type of a variable with the same name.
    // This is useful, as we dont have to change the name of a variable just to change it's type
    let space_value = "    ";
    let space_value = space_value.len();

    println!("Amount of space in original shadowed variable: {}", space_value);
} 