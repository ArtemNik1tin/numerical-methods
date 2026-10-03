use std::error::Error;
use std::io;

use super::input_error::InputError;
use super::validators::{
    validate_epsilon, validate_method_number, validate_number_of_partitions, validate_task_number,
};

pub fn read_epsilon() -> Result<f64, Box<dyn Error>> {
    let epsilon = read_f64()?;
    validate_epsilon(epsilon)?;
    Ok(epsilon)
}

pub fn read_method_number() -> Result<usize, Box<dyn Error>> {
    let method_number = read_usize()?;
    validate_method_number(method_number)?;
    Ok(method_number)
}

pub fn read_task_number() -> Result<usize, Box<dyn Error>> {
    let task_number = read_usize()?;
    validate_task_number(task_number)?;
    Ok(task_number)
}

pub fn read_number_of_partitions() -> Result<usize, Box<dyn Error>> {
    let number_of_partitions = read_usize()?;
    validate_number_of_partitions(number_of_partitions)?;
    Ok(number_of_partitions)
}

pub fn read_usize() -> Result<usize, Box<dyn Error>> {
    let mut raw_number = String::new();
    io::stdin().read_line(&mut raw_number)?;

    let trimmed = raw_number.trim();
    if trimmed.is_empty() {
        return Err(Box::new(InputError::EmptyInput));
    }

    let number = trimmed.parse()?;
    Ok(number)
}

pub fn read_f64() -> Result<f64, Box<dyn Error>> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    let trimmed = buffer.trim();
    if trimmed.is_empty() {
        return Err(Box::new(InputError::EmptyInput));
    }

    let number = trimmed.parse()?;
    Ok(number)
}
