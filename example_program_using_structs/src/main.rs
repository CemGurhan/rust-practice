fn main() {
    let rectangle1 = Rectangle { width: 10, height: 30 };

    println!("Rectangle area: {}", calculate_area(&rectangle1));
}

struct Rectangle {
    width: u64,
    height: u64,
}

// Using a rectangle struct there helps group together
// our length and width values together descriptively.
// This can be very helpful when attempting to better understand the code,
// as we can accurately convey the relationship between the width and height values

// Taking an immutable reference to the rectangle allows us to 
// call upon it's fields without moving it's values
// Taking an immutable reference also allows us to continue using
// the Rectangle struct in main.
fn calculate_area(rectangle: &Rectangle) -> u64 {
    rectangle.height * rectangle.width
}