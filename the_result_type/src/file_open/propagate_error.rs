use std::{fs::File, io::Read};

// We can use error propagation to return errors back
// to the caller of a function. This way, the error can
// get handled by the caller. The caller may have more 
// context or info surrounding this error, so may be able
// to handle it better
pub fn read_username_from_file() -> Result<String, std::io::Error> {
    let username_file_result = File::open("username.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e), // return early from function explicitly, instead of returning to variable
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e)
    }
}