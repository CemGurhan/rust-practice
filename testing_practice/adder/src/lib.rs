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

fn add_two(number: i32) -> i32 {
    number + 2
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

    fn large_can_hold_small() {
        let larger = Rectangle { width: 10, height: 12 };
        let smaller = Rectangle { width: 8, height: 7 };
        // the `assert!` macro will fail if the condition supplied to
        // it is `false` and pass if it is `true`
        assert!(larger.can_hold(&smaller));
    }

    fn small_cannot_hold_large() {
        let larger = Rectangle { width: 10, height: 12 };
        let smaller = Rectangle { width: 8, height: 7 };
        assert!(!smaller.can_hold(&larger));
    }

    // We can use the `assert_eq!` macro to assert to values are the same
    // We can use the `assert_ne!` macro to assert that two values are 
    // not the same. This can be useful when we dont know exactly what 
    // a value will be, but we know exactly what it won't be
    fn should_add_two() {
        assert_eq!(4, add_two(2));
    }

    fn cannot_be_negative_one() {
        assert_ne!(-1, add_two(10));
    }
    // Both `assert_eq!` and `assert_ne!` use `==` and `!=` under the 
    // hood respectively. The both also need to use `debug formatting`
    // to print out why they may have failed.
    // Hence, any items we use in these assertions must implement the
    // derivable traits `Debug` and `PartialOrd`
}
