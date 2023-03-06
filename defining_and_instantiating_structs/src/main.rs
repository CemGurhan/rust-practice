fn main() {
    defining_structs();    
}

fn defining_structs() {
    // A struct is a custom data type that allows you to group
    // together different types of data in the form of related values 
    

    // structs are similar to tuples. They can both store values with
    // different data types together. However, they are unlike each other 
    // in the sense that you must name each piece of data inside of a struct

    struct User {
        name : String,
        email : String,
        sign_in_count : u64,
    }

    let user1 = User { 
        name: String::from("username"), 
        email: String::from("an_email"), 
        sign_in_count: 1 };

}
