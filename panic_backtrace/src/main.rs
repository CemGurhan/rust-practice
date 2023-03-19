// Run with `RUST_BACKTRACE=1 cargo run` for a list of all function 
// calls made until this panic
fn main() {
    let v = vec![1,2,3,4,5];
    v[99];
}
