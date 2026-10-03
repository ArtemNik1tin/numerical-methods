use crate::constants::validation::{
    MAX_ITERATIONS, MIDPOINT_DIVISOR, SIGN_CHANGE_THRESHOLD,
};
use crate::test_task::function_utils::{validate_function_value, validate_sign_change};
use crate::test_task::method_result::MethodResult;
use std::error::Error;

pub fn bisection(
    a: f64,
    b: f64,
    epsilon: f64,
    f: impl Fn(f64) -> f64,
) -> Result<MethodResult, Box<dyn Error>> {
    let mut a = a;
    let mut b = b;

    let f_a = f(a);
    let f_b = f(b);

    validate_function_value(f_a)?;
    validate_function_value(f_b)?;
    validate_sign_change(f_a, f_b)?;

    let initial_left = a;
    let initial_right = b;
    let mut iterations = 0;

    loop {
        iterations += 1;
        if iterations > MAX_ITERATIONS {
            return Err("Метод бисекции не сошёлся за максимальное число итераций".into());
        }

        let c = (a + b) / MIDPOINT_DIVISOR;
        let f_c = f(c);

        validate_function_value(f_c)?;

        if f_a * f_c <= SIGN_CHANGE_THRESHOLD {
            b = c;
        } else {
            a = c;
        }

        if b - a <= MIDPOINT_DIVISOR * epsilon {
            break;
        }
    }

    let x = (a + b) / MIDPOINT_DIVISOR;
    let last_interval_length = b - a;

    Ok(MethodResult::new(
        "Метод бисекции",
        format!("({}; {})", initial_left, initial_right),
        iterations,
        x,
        last_interval_length,
        f64::abs(f(x)),
    ))
}
