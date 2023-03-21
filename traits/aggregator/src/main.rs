use aggregator::{Summary, NewsArticle};

fn main() {
    let n = NewsArticle{ headline: "todo!()".to_string(), 
                                        location: "todo!()".to_string(), 
                                        author: "todo!()".to_string(), 
                                        content: "todo!()".to_string() 
                                    };
    println!("{}",n.summarize());
}
