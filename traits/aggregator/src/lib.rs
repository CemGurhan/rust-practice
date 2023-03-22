use std::iter::Sum;

pub mod conditional_bounds;
pub mod blanket_implementations;

// A trait allows us to define some functionality that
// can be shared amongst different types. traits are 
// very similar to interfaces.
pub trait Summary {
    fn summarize(&self) -> String;
}

// We can implement a trait on a type in a way similar
// to when we implement some methods on a type.
// In Rust, you can only implement a trait on a type
// if at least one is local to the crate. You cannot
// implement an external trait on an external type.
// This is due to a property known as coherance, which
// has a rule know as the orphan rule. No value can be
// made parentless. Without this restriction, there'd be
// a chance of you code breaking someone elses or vice versa,
// as you could have a trait implementing a type in two places at once.
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

// We can have functions accept only items that implement
// a specific trait and then do stuff with those items
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// `impl trait` is a trait bound.
// The `impl trait` parameter is shorthand for the `trait
// bound on genric types` syntax
pub fn notify_two<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// The `trait bound` syntax should be used instead of `impl trait`
// parameters if we have multiple paramtes of the same `impl trait`
// type
pub fn notify_wrong(item1: &impl Summary, impl2: &impl Summary) {   // This 
    // logic                                                        //  |    
}                                                                   //  |  
                                                                    //  |   
pub fn notify_right<T: Summary>(item1: &T, item2: &T) {             // Should be this
    // logic 
}     

// We can have multiple trait bounds as a function parameter by using
// `+`
// This way, notify can call `summarize` and use `{}` to format item
pub fn notfiy_two(item1: &(impl Summary + std::fmt::Display)) { 
    // some logic
}

// You can do the same with trait bounds on generic types
pub fn notify_two_generic<T: Summary + std::fmt::Display>(item1: &T) {
    
}

// We can simplify a function signature down using the `where` clause. This
// can be useful when we have multiple arguments to a function with trait 
// bounds, making the function hard to read.

// For example, this:
pub fn bad_signature<T: std::fmt::Display + Clone, U: std::fmt::Debug + Clone>(item1: &T, item2: &U) -> i32{
    3
}
// Can become this:
pub fn good_signature<T, U>(item1: T, item2: U) -> i32 
where 
    T: std::fmt::Display + Clone,
    U: std::fmt::Debug + Clone, 
{
    4
}

// We can also return types that implement specific traits
pub fn returner() -> impl Summary {
    NewsArticle { headline: todo!(), location: todo!(), author: todo!(), content: todo!() }
}

// We can only return one type that implements a trait, and not either of two.
// This would fail, even though `Tweet` implements `Summary`:

// pub fn returner_switch(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle { headline: todo!(), location: todo!(), author: todo!(), content: todo!() }
//     } else {
//         Tweet {}
//     }
// }