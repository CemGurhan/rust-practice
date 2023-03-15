use std::collections::HashMap;

fn main() {
    new_hashmap();
    accessing_values_in_hashmaps();
    iterating_over_hashmaps();
}

fn new_hashmap() {
// HashMaps are key value pairs stored on the heap,
// where each key is mapped to a value using a hashing
// function
let mut scores = HashMap::new();
scores.insert(String::from("my team"), 1_000_000);
}

fn accessing_values_in_hashmaps() {
    let mut scores = HashMap::new();
    let your_team_name = String::from("Your team");

    scores.insert(&your_team_name, 10);

    // When you access a value from a hashMap key with `get`, 
    // an Option<&V> is returned. If the value isnt present, you
    // will be returned an `Option::None` variant. Here, we
    // use `copied()` to convert our Option<&i32> returned 
    // to Option<i32>. We than use `unwrap_or(0)` to convert
    // this `None` to 0 if the value for that key doesnt exist.
   let your_socre = scores.get(&your_team_name).copied().unwrap_or(0);
   println!("Your teams score: {}", your_socre);
}

fn iterating_over_hashmaps() {
    // we can iterate of HashMap values using for loops
    let mut scores = HashMap::new();
    scores.insert("wow", 20);
    scores.insert("woah", 25);

    for (key, value) in scores {
        println!("{key}: {value}");
    }
}