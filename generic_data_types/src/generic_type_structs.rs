// You can also have structs be generic over types
pub struct Point<T> {
    x: T,
    y: T
}
// Here, we'd say that `Point<T>` is generic over type `T`
// Only thing is, you can only have both fields be of the
// same type, as they both are of type `T`
fn do_stuff_with_point() {
    let one = Point {x: 4, y: 7};
    let two = Point {x: 12.4, y: 13.9};
}

// We can have both fields be of different type whilst
// also being generic as such:
pub struct Point<T, U> {
    x: T,
    y: U,
}
// We'd say `Point<T, U>` is generic over types `T` and `U`.
// Remember, `Point<T>` and `Point<T,U>` are different types now.
fn do_stuff_with_new_point() {
    let xx = Point {x: 12, y: 3.1};
    let yy = Point {x: 1.1, y: 2};
}

