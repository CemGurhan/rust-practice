// We can implement a trait on any type that 
// implements a specific type using trait bounds.
// This is called `Blanket Implementation`

pub trait PrintMe {
    fn print_me(&self);
}

impl<T: std::fmt::Display> PrintMe for T {
    fn print_me(&self) {
        println!("{}", self);
    }
}

pub fn example_blanket_impl() {
    PrintMe::print_me(&2);
}