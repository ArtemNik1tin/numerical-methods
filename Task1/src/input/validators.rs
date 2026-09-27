use crate::input::input_error::InputError;
use std::error::Error;

pub fn validate_section(start: f64, end: f64) -> Result<(), Box<dyn Error>> {
    if start.is_nan() || end.is_nan() {
        return Err(Box::new(InputError::InvalidSection));
    }
    if start >= end {
        return Err(Box::new(InputError::InvalidSection));
    }
    Ok(())
}

pub fn validate_number_of_partitions(n: usize) -> Result<(), Box<dyn Error>> {
    if n < 2 {
        return Err(Box::new(InputError::InvalidNumberOfPartitions));
    }
    Ok(())
}

pub fn validate_epsilon(epsilon: f64) -> Result<(), Box<dyn Error>> {
    if epsilon <= 0.0 {
        return Err(Box::new(InputError::InvalidEpsilon));
    }
    if epsilon < f64::EPSILON {
        return Err(Box::new(InputError::InvalidEpsilon));
    }
    Ok(())
}

pub fn validate_method_number(method: usize) -> Result<(), Box<dyn Error>> {
    if method == 0 || method >= 5 {
        return Err(Box::new(InputError::InvalidChoice));
    }
    Ok(())
}

pub fn validate_task_number(task: usize) -> Result<(), Box<dyn Error>> {
    if task != 1 && task != 2 {
        return Err(Box::new(InputError::InvalidChoice));
    }
    Ok(())
}
