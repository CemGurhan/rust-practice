fn main() {
    println!("Hello, world!");
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn coin_match(coin: Coin) -> u8 {
    // The match control flow construct allows you to compare a
    // value to multiple patterns, and then execute code on the
    // pattern that matches.
    //
    // Each arm of this match expression contains a pattern which
    // the value of `coin` is checked against. Once a value matches
    // the code to be executed occurs after the `=>` operator.
    // This code is an expression which returns a value for the
    // match expression.
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn coin_2(coin: Coin) -> u8 {
    // You can use curly brackets in match expressions if your
    // code runs over multiple lines. Commas here are then 
    // optional
    match coin {
        Coin::Penny => {
            println!("A lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}