fn main() {
    defining_structs();    
    struct_mutability();
    struct_return(String::from("I am"), String::from("an email"));
    struct_update_syntax();
    tuple_structs();
    unit_like_structs();
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

// We can use fields from an old struct instance in a new 
// struct instance using the struct update syntax.
// We must specify the fields we want new, and any remaining
// fields we want from the previous struct we can use by
// doing `..old_struct_name`. The struct update syntax
// must come last, as we only want to use it for remaining fields
fn struct_update_syntax() {
    let user1 = User { 
        name: String::from("username"), 
        email: String::from("an_email"), 
        sign_in_count: 1 };

    let user2 = User {
        email: String::from("a new email"),
        ..user1
    };

    // in this example, we can no longer use the user1 struct. This is
    // because user2 = is an assignment, and hence, we have moved 
    // values from user1 to user2, invalidating user1 (user2 is now
    // the only valid stack data pointing to the heap memmory component
    // of the old fields)

    // However, if we chose to create a new field for email and name in
    // user2, we would still be able to use user1. This is because no
    // move occurs. The fields that are being used via the struct update syntax 
    // all implement the Copy trait and have no heap component (`sign_in_count`).
}

fn tuple_structs() {
    // tuple structs are similar to tuples. They behave like standard
    // structs, except you cannot name each field present, only assign a type.
    struct Color(u32, u32, u32);
    struct Point(u32, u32, u32);

    let _black = Color(0,0,0);
    let _origin = Point(0,0,0);

    // They can be useful when:
    // 1) You want to give the whole tuple a name
    // 2) You want a tuple to be of a particular type
    // 3) Naming each field in your struct is verbose/redundant
}

fn unit_like_structs() {
    // We can define structs with no fields
    struct AlwaysEqual;
    let _subject = AlwaysEqual;
}
