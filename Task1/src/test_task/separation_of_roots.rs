use crate::constants::validation::{MIN_PARTITIONS, SIGN_CHANGE_THRESHOLD};
use crate::test_task::test_task_error::TestTaskError;
use crate::test_task::function_utils::validate_function_value;
use std::error::Error;

pub fn find_number_of_segments_with_sign_change(
    start_section: f64,
    end_section: f64,
    number_of_partitions: usize,
    step: f64,
    f: fn(f64) -> f64,
) -> Result<(usize, Vec<(f64, f64)>), Box<dyn Error>> {
    validate_section(start_section, end_section)?;
    validate_number_of_partitions(number_of_partitions)?;
    validate_step(step, start_section, end_section)?;

    let mut counter = 0;
    let mut x1 = start_section;
    let mut x2 = x1 + step;
    let mut y1 = f(x1);

    validate_function_value(y1)?;

    let mut segments: Vec<(f64, f64)> = Vec::new();
    while x2 <= end_section {
        let y2 = f(x2);
        validate_function_value(y2)?;

        if y1 * y2 <= SIGN_CHANGE_THRESHOLD {
            counter += 1;
            segments.push((x1, x2));
        }
        x1 = x2;
        x2 = x1 + step;
        y1 = y2;
    }

    Ok((counter, segments))
}

fn validate_section(start: f64, end: f64) -> Result<(), Box<dyn Error>> {
    if start >= end {
        return Err(Box::new(TestTaskError::InvalidSection));
    }
    Ok(())
}

fn validate_number_of_partitions(n: usize) -> Result<(), Box<dyn Error>> {
    if n < MIN_PARTITIONS {
        return Err(Box::new(TestTaskError::InvalidNumberOfPartitions));
    }
    Ok(())
}

fn validate_step(step: f64, start: f64, end: f64) -> Result<(), Box<dyn Error>> {
    if step <= 0.0 || step > (end - start) {
        return Err(Box::new(TestTaskError::InvalidStep));
    }
    Ok(())
}
