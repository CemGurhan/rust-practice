// We can have generic type parameters on methods.
// We have to declare the generic type paramter on
// the imply as well as the item, as this will let Rust
// know that the type in the intems angle brackets is 
// a generic one and not concrete. Both these types can
// also be different in name, but it is convention to keep
// them the same
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// We can also constrain type parameters by having methods
// be implemented for types with a specific value in place 
// of their generic type
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        self.x
    }
}
// The method `distance_from_origin` can only be used on
// types with `f32` in place of their generic type

// You can have a set of generic type parameters be relevent 
// to an item and have another set be relevent to a method on
// that item
struct Wow<X1,Y1>{
    woah: X1,
    wowz: Y1,
}

impl<X1,Y1> Wow<X1,Y1>{
    fn changes<X2,Y2>(self,other: Wow<X2, Y2>) -> Wow<X1, Y2> {
        Wow {
            woah: self.woah, 
            wowz: other.wowz
        }
    }
}
