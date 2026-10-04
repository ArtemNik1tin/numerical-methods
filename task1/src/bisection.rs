use std::error::Error;

use crate::constants::MAX_ITERATIONS;
use crate::method_result::MethodResult;

pub fn bisection(
    a: f64,
    b: f64,
    epsilon: f64,
    f: impl Fn(f64) -> f64,
) -> Result<MethodResult, Box<dyn Error>> {
    let mut a = a;
    let mut b = b;

    let f_a = f(a);

    let mut iterations = 0;

    loop {
        iterations += 1;
        if iterations > MAX_ITERATIONS {
            return Err("Метод бисекции не сошёлся за максимальное число итераций".into());
        }

        let c = (a + b) / 2.0;
        let f_c = f(c);

        if f_a * f_c <= 0.0 {
            b = c;
        } else {
            a = c;
        }

        if b - a <= 2.0 * epsilon {
            break;
        }
    }

    let x = (a + b) / 2.0;
    let last_interval_length = b - a;

    Ok(MethodResult {
        name: "Метод бисекции",
        iterations,
        root: x,
        difference: last_interval_length,
        residual: f64::abs(f(x)),
    })
}
