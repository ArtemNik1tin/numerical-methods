use crate::constants::validation::MAX_ITERATIONS;
use crate::test_task::method_result::MethodResult;
use std::error::Error;

pub fn newton(
    x0: f64,
    epsilon: f64,
    f: impl Fn(f64) -> f64,
    df: impl Fn(f64) -> f64,
) -> Result<MethodResult, Box<dyn Error>> {
    let mut x_prev = x0;
    let mut iterations = 0;

    loop {
        iterations += 1;
        if iterations > MAX_ITERATIONS {
            return Err("Метод Ньютона не сошёлся за максимальное число итераций".into());
        }

        let f_x = f(x_prev);
        let df_x = df(x_prev);

        if df_x == 0.0 {
            return Err("Производная равна нулю. Метод Ньютона неприменим.".into());
        }

        let x_next = x_prev - f_x / df_x;

        if (x_next - x_prev).abs() <= epsilon {
            let residual = f64::abs(f(x_next));
            return Ok(MethodResult::new(
                "Метод Ньютона",
                format!("{:.6}", x0),
                iterations,
                x_next,
                (x_next - x_prev).abs(),
                residual,
            ));
        }

        x_prev = x_next;
    }
}

pub fn modified_newton(
    x0: f64,
    epsilon: f64,
    f: impl Fn(f64) -> f64,
    df: impl Fn(f64) -> f64,
) -> Result<MethodResult, Box<dyn Error>> {
    let df_x0 = df(x0);

    if df_x0 == 0.0 {
        return Err("Производная в начальной точке равна нулю. Модифицированный метод Ньютона неприменим.".into());
    }

    let mut x_prev = x0;
    let mut iterations = 0;

    loop {
        iterations += 1;
        if iterations > MAX_ITERATIONS {
            return Err("Модифицированный метод Ньютона не сошёлся за максимальное число итераций".into());
        }

        let f_x = f(x_prev);

        let x_next = x_prev - f_x / df_x0;

        if (x_next - x_prev).abs() <= epsilon {
            let residual = f64::abs(f(x_next));
            return Ok(MethodResult::new(
                "Модифицированный метод Ньютона",
                format!("{:.6}", x0),
                iterations,
                x_next,
                (x_next - x_prev).abs(),
                residual,
            ));
        }

        x_prev = x_next;
    }
}

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
            return Ok(MethodResult::new(
                "Метод секущих",
                format!("({:.6}; {:.6})", x0, x1),
                iterations,
                x_next,
                (x_next - x_curr).abs(),
                residual,
            ));
        }

        x_prev = x_curr;
        x_curr = x_next;
    }
}
