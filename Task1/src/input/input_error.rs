use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum InputError {
    InvalidChoice,
    InvalidNumberOfPartitions,
    InvalidEpsilon,
    EmptyInput,
    InvalidSection,
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            InputError::InvalidChoice => write!(f, "Номер не содержится в списке"),
            InputError::InvalidNumberOfPartitions => {
                write!(f, "Число отрезков разбиения должно быть больше 1")
            }
            InputError::InvalidEpsilon => write!(f, "Epsilon должен быть больше 0"),
            InputError::EmptyInput => write!(f, "Ввод не может быть пустым"),
            InputError::InvalidSection => write!(f, "Отрезок составлен некорректно"),
        }
    }
}

impl Error for InputError {}
