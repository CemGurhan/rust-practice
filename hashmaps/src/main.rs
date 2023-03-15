use std::collections::HashMap;

fn main() {
    new_hashmap();
    accessing_values_in_hashmaps();
    iterating_over_hashmaps();
    updating_hashmaps();
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
    // we can iterate over HashMap values using for loops
    let mut scores = HashMap::new();
    scores.insert("wow", 20);
    scores.insert("woah", 25);

    for (key, value) in scores {
        println!("{key}: {value}");
    }
}

fn updating_hashmaps() {
    // We can overwrite a value at a key by inserting the same
    // key with a different value
    let mut scores = HashMap::new();
    
    scores.insert("wow", 20);
    scores.insert("wow", 345);

    println!("scores: {:?}", scores);

    // We can insert a new key-value pair only if that key in the
    // HashMap doesnt have a value yet by using the `entry` API.
    // The `entry` API returns an `Entry` enum, which represents
    // whether a key already exists or not
    scores.entry("wow").or_insert(222); // Checks to see if the key `wow` has a value, if not, insert 222
    scores.entry("wowzer").or_insert(9827); // Checks to see if the key `wowzer` has a value, if not, insert 9827
    // `or_insert` will add a value if the key doesnt already have 
    // that value, or wont add that value if the key does have it.
    // In both cases, a mutable reference to the value is returned.
    println!("updated scores: {:?}", scores);
    
    // We can update a value based on old values as such.
    // *count is a dereference of the mutable reference to the value
    // added to a key in the Hashmap by or_insert. We can then alter
    // this value in the HashMap using this derefernced count.
    let mut map = HashMap::new();
    let text = "hello world wonderful world";
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}