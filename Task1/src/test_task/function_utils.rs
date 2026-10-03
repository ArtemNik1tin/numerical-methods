use crate::constants::validation::SIGN_CHANGE_THRESHOLD;
use std::error::Error;

pub fn validate_function_value(value: f64) -> Result<(), Box<dyn Error>> {
    if value.is_nan() || value.is_infinite() {
        return Err("Функция вернула NaN или Infinity".into());
    }
    Ok(())
}

pub fn validate_sign_change(f_a: f64, f_b: f64) -> Result<(), Box<dyn Error>> {
    if f_a * f_b > SIGN_CHANGE_THRESHOLD {
        return Err("Функция должна иметь разные знаки на концах отрезка".into());
    }
    Ok(())
}
