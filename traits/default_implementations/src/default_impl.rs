// We can create a `trait` with default implementation
// for the `summarize` method. 
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Learn more ...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// We can then use this default implementation for 
// NewsArticle without having to specify anything in
// our curly brackets
impl Summary for NewsArticle {}

fn _summarize_example() {
    let n = NewsArticle{ headline: "todo!()".to_string(), 
                                        location: "todo!()".to_string(), 
                                        author: "todo!()".to_string(), 
                                        content: "todo!()".to_string() 
                                    };
    n.summarize(); // `sumamrize` is still available to us
}