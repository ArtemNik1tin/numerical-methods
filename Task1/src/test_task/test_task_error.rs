use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum TestTaskError {
    InvalidSection,
    InvalidNumberOfPartitions,
}

impl fmt::Display for TestTaskError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            TestTaskError::InvalidSection => write!(f, "Отрезок составлен некорректно"),
            TestTaskError::InvalidNumberOfPartitions => write!(f, "Число отрезков разбиения должно быть больше 1"),
        }
    }
}

impl Error for TestTaskError {}