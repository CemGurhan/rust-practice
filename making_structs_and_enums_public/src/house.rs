// If you make a struct inside a child module public
// using the `pub` keyword, it's field's will still
// remain private. You can make struct fields public
// or private on a case by case basis.
// Notice that you have to create an associate function
// for this struct that acts as a constructor (doesn't
// take in `self`). This is because the private field
// `door` cannot be accessed in the super module, hence,
// you cannot instantiate a struct in the super module
// as you cannot edit this field.
pub mod garden;

use garden::Garden;

#[derive(Debug)]
pub struct HouseStruct {
    pub window : String,
    door : String,
    pub garden_size : Garden
}

// The associate function must be public so it can be
// accessed in the super scope. The fucntion paramter
// must be o type &str as String::from() takes in 
// &str as it's paramter type
impl HouseStruct {
    pub fn create_house(window_name: &str, garden_size: u32) -> HouseStruct {
        HouseStruct { 
            window: String::from(window_name), 
            door: String::from("A hardcoded door") ,
            garden_size: Garden::Big(garden_size),
        }
    }
}
