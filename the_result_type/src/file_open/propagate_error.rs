use std::{fs, fs::File, io::Read};

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

// We can shorten this code using the `?` operator. The `?` 
// operator will either return a value from the operation to
// a variable if an `Ok` is found, or will return from the 
// entire function with an error if an `Err` is found
pub fn read_username_from_file_shortened_one() -> Result<String, std::io::Error> {
    let mut username_file = File::open("username.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// We can shorten this even further using chaining
pub fn read_username_from_file_shortened_two() -> Result<String, std::io::Error> {
    let mut username = String::new();
    File::open("username.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// We can shorten this even further using fs::read_to_string,
// which opens a file, creates a new string and reads the file
// to the new string
pub fn read_username_from_file_fs_function() -> Result<String, std::io::Error> {
    fs::read_to_string("username.txt")
}



