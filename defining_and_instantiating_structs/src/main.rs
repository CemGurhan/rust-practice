fn main() {
    defining_structs();    
    struct_mutability();
    struct_return(String::from("I am"), String::from("an email"));
}

struct User {
    name : String,
    email : String,
    sign_in_count : u64,
}

fn defining_structs() {
    // A struct is a custom data type that allows you to group
    // together different related values. These values can also
    // be of different data types.
    

    // structs are similar to tuples. They can both store values with
    // different data types together. However, they are unlike each other 
    // in the sense that you must name each piece of data inside of a struct
    let _user1 = User { 
        name: String::from("username"), 
        email: String::from("an_email"), 
        sign_in_count: 1 };
}

fn struct_mutability() {
    // struct instances are immutable by default. In order to make an instance
    // mutable we must use the `mut` keyword upon declaration. 
    // You cannot make individual fields mutable in a struct instance, you must
    // make the entire instance mutable instead. 
    struct User {
        name : String,
        email : String,
        sign_in_count : u64,
    }

    let mut mutable_user = User { 
        name: String::from("Change me!"), 
        email: String::from("Change me!"), 
        sign_in_count: 1 };
    
    println!("Old email: {}", mutable_user.email);

    mutable_user.email = String::from("new_email@email.com");

    println!("New email: {}", mutable_user.email);
}

// You can return a struct instance by declaring it as a final
// expression.
// Here, as the email and name fields are the same as the function
// parameters, we can use the field init shorthand in the struct
// to avoid repition.
fn struct_return(name: String, email: String) -> User {
    User { 
        name, // field init shorthand
        email, // field init shorthand
        sign_in_count: 1 }
}
