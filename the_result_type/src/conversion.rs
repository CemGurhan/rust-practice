use std::{io::Error, fs};

// We can convert an `Option` to `Result` with `ok_or`
pub fn option_to_result() -> i32 {
    let v = vec![1,33,21,-10];
    let vector_index_option = v.get(1).copied();
    let vector_index = vector_index_option.ok_or("Unable to retrieve value at index");
    match vector_index {
        Ok(number) => number,
        Err(e) => panic!("{}", e)
    }
}

// We can convert from `Result` to `Option` with `ok`

pub fn result_to_option() -> Option<String> {
    let converted = fs::read_to_string("username.txt").ok();
    converted
}
