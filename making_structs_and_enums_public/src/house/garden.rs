// Once an enum is marked as public using the `pub`
// keyword, all of it's variants will be automatically
// public.

#[derive(Debug)]
pub enum Garden {
    Big(u32),
    Small(u8)
}
