fn main() {
    let x = 22;
    let x = 23;
    {
        let x = 23 * 2; 
        println!("The value of x in inner scope is: {}", x);
    }
    println!("The value of x in outer scope is: {}", x);
} 