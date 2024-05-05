use std::error::Error;
use std::fmt;


#[derive(Debug)]
struct ParsingError {
    message: String,
}

impl Error for ParsingError{}


impl fmt::Display for ParsingError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.mensaje)
    }
}


