use crate::test_task::function_utils::{validate_function_value, validate_sign_change};
use std::error::Error;

const MAX_ITERATIONS: usize = 1000;

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

        let c = (a + b) / 2.0;
        let f_c = f(c);

        validate_function_value(f_c)?;

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
        initial_approximation: format!("({}; {})", initial_left, initial_right),
        iterations,
        root: x,
        difference: last_interval_length,
        residual: f64::abs(f(x)),
    })
}

pub struct MethodResult {
    pub name: &'static str,
    pub initial_approximation: String,
    pub iterations: usize,
    pub root: f64,
    pub difference: f64,
    pub residual: f64,
}

impl MethodResult {
    pub fn print(&self) {
        println!("{}", self.name);
        println!("  Начальное приближение: {}", self.initial_approximation);
        println!("  Количество шагов: {}", self.iterations);
        println!("  Приближенный корень: {:.16}", self.root);
        println!("  |xm - xm-1|: {:e}", self.difference);
        println!("  Невязка |f(xm)|: {:e}", self.residual);
    }
}
