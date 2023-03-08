fn main() {
    let rectangle1 = Rectangle {
        width: 50,
        height: 70
    };
    println!("area of rectangle: {}", rectangle1.area());
    // Rust can automatically reference and dereference based on
    // self. For example, here, we called rectangle1.area(), when
    // area() expected the reciever to be a reference.
    // Rust automatically referenced for us, so these two are the same:
    //                    rectangle1.area()
    //                    (&rectangle1).area()
}

#[derive(Debug)]
struct Rectangle {
    width: u64,
    height: u64
}

// We can use impl blocks to organize everything that
// relates to a particular struct
// `self` is an alias for whatever struct the impl block
// was created for. It is shorthand for self: Self 
// (in our case self: &Self, which is analagous to
// rectangle: &Rectangle)
// we can take ownership of self, immutably borrow 
// or mutably morrow from self
impl Rectangle {
    fn area(&self) -> u64 {
        self.height * self.width
    }
}


