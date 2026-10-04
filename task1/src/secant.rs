use crate::constants::MAX_ITERATIONS;
use crate::method_result::MethodResult;
use std::error::Error;

pub fn secant(
    x0: f64,
    x1: f64,
    epsilon: f64,
    f: impl Fn(f64) -> f64,
) -> Result<MethodResult, Box<dyn Error>> {
    let mut x_prev = x0;
    let mut x_curr = x1;
    let mut iterations = 0;

    loop {
        iterations += 1;
        if iterations > MAX_ITERATIONS {
            return Err("Метод секущих не сошёлся за максимальное число итераций".into());
        }

        let f_prev = f(x_prev);
        let f_curr = f(x_curr);

        if f_curr == f_prev {
            return Err("Занменатель равен нулю. Метод секущих неприменим.".into());
        }

        let x_next = x_curr - f_curr * (x_curr - x_prev) / (f_curr - f_prev);

        if (x_next - x_curr).abs() <= epsilon {
            let residual = f64::abs(f(x_next));
            return Ok(MethodResult {
                name: "Метод секущих",
                initial_guess: x0,
                iterations,
                root: x_next,
                difference: (x_next - x_curr).abs(),
                residual,
            });
        }

        x_prev = x_curr;
        x_curr = x_next;
    }
}
