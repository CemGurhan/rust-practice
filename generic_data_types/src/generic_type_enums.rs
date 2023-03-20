// We can have an enum be generic over one or
// more types

enum OneEnum<T> {
    Hello(T), 
    ByeBye,
}

enum TwoEnum<T, U> {
    Hello(T),
    ByeBye(U),
}