use crate::constants::validation::{
    MAX_METHOD_NUMBER, MIN_EPSILON, MIN_METHOD_NUMBER, MIN_PARTITIONS, TASK_SPHERE, TASK_TEST,
};
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
    if n < MIN_PARTITIONS {
        return Err(Box::new(InputError::InvalidNumberOfPartitions));
    }
    Ok(())
}

pub fn validate_epsilon(epsilon: f64) -> Result<(), Box<dyn Error>> {
    if epsilon <= MIN_EPSILON {
        return Err(Box::new(InputError::InvalidEpsilon));
    }
    if epsilon < f64::EPSILON {
        return Err(Box::new(InputError::InvalidEpsilon));
    }
    Ok(())
}

pub fn validate_method_number(method: usize) -> Result<(), Box<dyn Error>> {
    if method < MIN_METHOD_NUMBER || method > MAX_METHOD_NUMBER {
        return Err(Box::new(InputError::InvalidChoice));
    }
    Ok(())
}

pub fn validate_task_number(task: usize) -> Result<(), Box<dyn Error>> {
    if task != TASK_TEST && task != TASK_SPHERE {
        return Err(Box::new(InputError::InvalidChoice));
    }
    Ok(())
}
