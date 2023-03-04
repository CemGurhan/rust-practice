fn main() {
    scope();
}

fn scope() {
       { // s is not valid here or before here
        let s = "am I valid?"; // s is valid here onwards as it comes into scope
        println!("{} Yes!", s);        
    }   // s is not valid from this point on 
}
