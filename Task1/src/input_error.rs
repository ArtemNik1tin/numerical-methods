use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum InputError {
    InvalidChoice,
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            InputError::InvalidChoice => write!(f, "Номер не содержится в списке"),
        }
    }
}

impl Error for InputError {}