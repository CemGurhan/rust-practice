struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// We can call other trait methods from inside a default
// implementation. Calling `self` here refers to the trait
// `SummaryExtraExample` and its methods
trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

fn _summarize_example_two() {
    let n = NewsArticle{ headline: "todo!()".to_string(), 
                                        location: "todo!()".to_string(), 
                                        author: "todo!()".to_string(), 
                                        content: "todo!()".to_string() 
                                    };
    println!("{}",n.summarize());
}