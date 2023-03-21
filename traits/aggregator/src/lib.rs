use std::iter::Sum;

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

// The `impl trait` parameter is shorthand for the `trait
// bound` syntax
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