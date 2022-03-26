use std::fs::File;
use std::io::{self, Read};
use std::num;

#[derive(Debug)]
struct AppError {
    kind: String,
    message: String,
}

// Implement std::convert::From for AppError; from io::Error
impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError {
            kind: String::from("io"),
            message: error.to_string(),
        }
    }
}

// Implement std::convert::From for AppError; from num::ParseIntError
impl From<num::ParseIntError> for AppError {
    fn from(error: num::ParseIntError) -> Self {
        AppError {
            kind: String::from("parse"),
            message: error.to_string(),
        }
    }
}

fn main() -> Result<(), AppError> {
    //let mut file = File::open("hello_world.txt")?; // generates an io::Error, if can not open the file and converts to an AppError
    let mut file = File::open("hello_world.txt")?; // generates an io::Error, if can not open the file and converts to an AppError

    let mut content = String::new();
    file.read_to_string(&mut content)?; // generates an io::Error, if can not read file content and converts to an AppError

    let _number: usize;
    _number = content.parse()?; // generates num::ParseIntError, if can not convert file content to usize and converts to an AppError

    Ok(())
}

