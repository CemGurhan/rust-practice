fn main() {
    patterns_that_bind_to_values();
    let five = Some(5);
    let absent_number : Option<i32> = None;
    let _six = matching_options(five);
    let _absent = matching_options(absent_number);
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

fn patterns_that_bind_to_values() -> u8 {
    // Let's say that quarters can have a specific US state
    // printed on one side.
    #[derive(Debug)] // so we can print out and debug later
    enum State {
        Alabama,
        Alaska,
        // etc.
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(State),
    }
    let coin = Coin::Quarter(State::Alabama);
    // We can then bind the state of a quarter to a value in
    // our match expression's arm
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Our coins state is: {:?}", state);
            25
        },
    }
    // This provides us with a means of extracting the values stored 
    // in enums. We binded the value of the quarter `State::Alabama`
    // into the variable `state` which we can then use in our code.
}

fn matching_options(x: Option<i32>) -> Option<i32> {
    // We can use match statements to work with Option<T> enums 
    // and their variants.
    // We can use match statements pattern binding to bind to a
    // value insise the Some(T) variant of an Option<T> enum.
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
    // We can bind the pattern Some(i) to whatever `Some(i32)`
    // enters our match expression, such that we can execute logic on 
    // i32 value that the entering Some(i32) variant contained.
}