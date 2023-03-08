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
