// When a function’s implementation calls something that might fail, instead of handling the error within the function itself, 
// you can return the error to the calling code so that it can decide what to do. This is known as propagating the error and gives more control to the calling code, 
// where there might be more information or logic that dictates how the error should be handled than what you have available in the context of your code.

use std::fs::File;
use std::io::{self, Read};

//create a function which returns error if something goes wrong
fn read_username_from_file(file_name: &str) -> Result<String, io::Error> {
    let username_file_result = File::open(file_name);

    let mut user_name = String::new();

    let mut user_name_file = match username_file_result {
        Ok(file) => file,
        Err(error) => return Err(error),        
    };
    match user_name_file.read_to_string(&mut user_name) {
        Ok(_) => Ok(user_name),
        Err(e) => Err(e),
        
    }
}


fn main() {
    // println!("Hello, world!");
    let file_name = read_username_from_file("hello.txt");
    println!("The username is {:?}", file_name);
}
