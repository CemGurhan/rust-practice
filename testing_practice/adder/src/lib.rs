#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

pub fn add_two(number: i32) -> i32 {
    number + 2
}

fn greet(name: &str) -> String {
    format!("Hello {}!", name)
}

// Rust will create a test runner binary for us, which will run 
// functions with the `test` attributes attached to them
// Attributes are like metadata we attach to a function
#[cfg(test)]
mod tests {
    // As we need items from the outer modules, we can use the 
    // `glob operator` with the `super` keyword to pull everything
    // out from the outer scope
    use super::*;

    #[test]
    fn large_can_hold_small() {
        let larger = Rectangle { width: 10, height: 12 };
        let smaller = Rectangle { width: 8, height: 7 };
        // the `assert!` macro will fail if the condition supplied to
        // it is `false` and pass if it is `true`
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn small_cannot_hold_large() {
        let larger = Rectangle { width: 10, height: 12 };
        let smaller = Rectangle { width: 8, height: 7 };
        assert!(!smaller.can_hold(&larger));
    }

    // We can use the `assert_eq!` macro to assert to values are the same
    // We can use the `assert_ne!` macro to assert that two values are 
    // not the same. This can be useful when we dont know exactly what 
    // a value will be, but we know exactly what it won't be
    #[test]
    fn should_add_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn cannot_be_negative_one() {
        assert_ne!(-1, add_two(10));
    }
    // Both `assert_eq!` and `assert_ne!` use `==` and `!=` under the 
    // hood respectively. The both also need to use `debug formatting`
    // to print out why they may have failed.
    // Hence, any items we use in these assertions must implement the
    // derivable traits `Debug` and `PartialOrd`

    // We can add messages to our assertions to better explain why theyve 
    // failed. This can be done as further arguments to assertion macros.
    // This is because assert macros can take `format!` as a further optional
    // argument
    #[test]
    fn greeting_should_contain_name() {
        let result = greet("test_name");
        assert!(
            result.contains("test_name"),
            "Greeting did not contain expected name: {}",
            result
        );
    }

    // We can use the `should_panic` attribute to assert that our tested code
    // panics
    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    // `should_panic` can be imprecise, as a the code we're testing could panic for 
    // any reason outside of the reason we were expecting it to panic for.
    // We can use the `expected` paramter and supply it with a string or substring of
    // the message we're expecting the code to panic with, to be more precise
    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100_precise() {
        Guess::new(200);
    }

    // You can aslo return a `Result<T, E>` enum in a test if the test failure doesnt
    // require a panic. This will allow you to use the `?` operator
    #[test]
    fn return_a_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("2 + 2 != 4"))
        }
    }

    // When testing for an `Err` variant, don't return a Result<T,E>, instead use the 
    // `assert!` macro:
    // #[test]
    // fn no_result_enum() {
    //     assert!(value.is_err())
    // }
}
