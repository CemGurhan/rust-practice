// We can add bounds to a generic type paramter,
// restricting what methods are allowed on specific 
// types

struct Pair<T> {
    x: T,
    y: T,
}


// For example a standard set of methods implemented 
// for Pair<T> :
impl<T> Pair<T> {
    fn new(x : T, y: T) -> Self {
        Self { x, y }
    }
}

// We can now implement a method on a `Pair<T>` only if
// it's inner generic type implements specif traits by
// introducing a trait bound
// For example, the method `display_me` will only be 
// present for `Pair<T>` if it's inner values implement
// `Display` or `PartialOrd` (such as an i32)
impl<T: std::fmt::Display + PartialOrd> Pair<T> {
    fn display_me(&self) {
        if self.x > self.y {
            println!("Im bigger: {}", self.x)
        } else {
            println!("Im bigger: {}", self.y)
        }
    }
}

pub fn conditional_trait_bound_example() {
    // Standard `Pair<T>`
    let p = Pair::new(2, 3);

    // Bounded `Pair<T>`
    p.display_me();
}

