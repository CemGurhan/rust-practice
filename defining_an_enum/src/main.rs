fn main() {
    defining_enums();
    enums_vs_structs();
    enums_and_structs();
    enums_and_methods();
}

fn defining_enums() {
    // Enums allow you to define a type by 
    // enumerating it's possible variants.
    // An enum can be one type that has a
    // set of possible values
    enum IpAddrKind {
        V4,
        V6
    }

    // We can then access each variant of our enum
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    // Notice how each variant is namespaced by it's enum
    // type. This is useful, as now we know that both are of the
    // same type: IpAddrKind
}

fn enums_vs_structs() {
    // Lets say we wanted to store some data associated
    // with our IP addresses. We could use a struct
    enum IpAddrkind {
        V4,
        V6
    }

    struct IpAddr_struct {
        kind : IpAddrkind,
        address : String
    }

    let home_1 = IpAddr_struct {
        kind : IpAddrkind::V4,
        address : String::from("127.0.0.1")
    };

    let loopback_1 = IpAddr_struct {
        kind : IpAddrkind::V6,
        address : String::from("::1")
    };

    // however, using enums can be more advantagous.
    // One advantage is, they are more concise
    enum IpAddr {
        V4(String),
        V6(String)
    }

    let home_2 = IpAddr::V4(String::from("127.0.0.1"));
    let loopback_one = IpAddr::V6(String::from("::1"));
    // enum variants can become a function that constructs a 
    // new instance of that enum. The function cosntructor here
    // takes in a String and then constructs a new instance of
    // IpAddr

    // Another advantage is, we can have different types and amounts
    // of data associated with an enum variant. We can have an IpAddr
    // enum that has one variant have a constructor that takes in only
    // u8's (V4) and another a String (V6)
     enum IpAddr_2 {
        V4(u8,u8,u8,u8),
        V6(String)
     }

     let home_3 = IpAddr_2::V4(127,0,0,1);
     let loopback_3 = IpAddr_2::V6(String::from("::1"));
}

fn enums_and_structs() {
    enum Message {
        Quit,
        Move {x : i32, y : i32},
        Write(String),
        ChangeColour(i32,i32,i32)
    }
    // We can write this enum as a set of structs
    struct Quit; // unit struct
    struct Move {
        x : i32,
        y : i32
    }
    struct Write(String); // tuple struct
    struct ChangeColour(i32, i32, i32); // tuple struct
    // Each of these structs has their own type,
    // whereas with the enum all variants were namespaced 
    // under the same type.
}

fn enums_and_methods() {
    // We can define methods for an enum using impl
    enum Message {
        Quit,
        Move {x : i32, y : i32},
        Write(String),
        ChangeColour(i32,i32,i32)
    }

    impl Message {
        fn call(&self) {
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
    // here, self represents the variant that called the method,
    // so is represented by Message::Write(String::from("hello"))
}

fn option_type() {
    // The Option<T> enum allows you to encode the scenario
    // of having a value being absent or present. It consists
    // of two variants, `Some(T)` and None. Both these variants
    // are present in the Rust prelude, along with the entire 
    // Option<T> enum.
    // The Option enum takes a generic type parameter `T`,
    // which can then be used as an argument to the `Some` 
    // function constructor as a means of constructing an 
    // Option<T> enum of type T.
    // `Option<T>` is better than `Null` due to the fact that
    // the `T` and `Option<T>` are two different types. 
    // Rust won't let you use `Option<T>` as if it were a valid
    // value. You can't add an Option<i8> to an i8 for example.
    // This is because there is always the case that a value is
    // absent when using `Option<T>`, so we must handle that case
    // before we can safely use the value `T`.

    let some_number = Some(5);
    let some_char = Some('c');
    
    let absent_number : Option<i32> = None;
    // We must specify the type of absent number as `Option<i32>`,
    // as Rust wont know what the value of `T` should be for 
    // the `Some(T)` value of this `Option<T>` if we instantiate it 
    // as `None`.
    
}