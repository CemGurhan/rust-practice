fn main() {
    defining_enums();
    enums_vs_structs();
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
    // enum fields can become a function that constructs a 
    // new instance of that enum. The function cosntructor here
    // takes in a String and then constructs a new instance of
    // IpAddr

    // Another advantage is, we can have different types and amounts
    // of data associated with an enum field. We can have an IpAddr
    // enum that has one field have a constructor that takes in only
    // u8's (V4) and another a String (V6)
     enum IpAddr_2 {
        V4(u8,u8,u8,u8),
        V6(String)
     }

     let home_3 = IpAddr_2::V4(127,0,0,1);
     let loopback_3 = IpAddr_2::V6(String::from("::1"));
}
