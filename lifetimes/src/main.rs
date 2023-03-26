pub mod struct_lifetimes;

// As we can see, the lifetime of `result` is less than the overlap of x and y's lifetimes ('a)
fn main() {
    let string1 = String::from("hi");      // ------------------------- x 
    {                                                                       //  |  
        let string2 = String::from("bye"); // ------------'a  y     //  |  
        let result = longest(&string1, &string2);  //   |  |     //  |  
        println!("Longest string: {}", result);               //   |  |     //  |   
    } // -------------------------------------------------------------      //  |  
}  // --------------------------------------------------------------------------- 

// We can use the lifetime annotation syntax to define
// the relationship between the lifetimes of multiple 
// references. 
// We can use the lifetime annotation syntax in a function
// with a generic lifetime parameter
//
// In this function, we are saying that the parameters to and
// return from this function must all be valid for as long as 
// at least the lifetime `'a`. 
// The lifetime `'a` is as long as the overlap between the lifetimes
// of x and y. The return value must have a lifetime smaller than this
// overlap
fn longest<'a, T: PartialOrd + ExactSizeIterator>(x: &'a T, y: &'a T) -> &'a T {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
