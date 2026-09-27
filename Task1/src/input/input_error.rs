use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum InputError {
    InvalidChoice,
    InvalidNumberOfPartitions,
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            InputError::InvalidChoice => write!(f, "Номер не содержится в списке"),
            InputError::InvalidNumberOfPartitions => write!(f, "Число отрезков разбиения должно быть больше 1"),
        }
    }
}

impl Error for InputError {}