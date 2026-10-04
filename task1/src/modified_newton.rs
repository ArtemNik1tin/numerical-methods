use crate::constants::MAX_ITERATIONS;
use crate::method_result::MethodResult;
use std::error::Error;

pub fn modified_newton(
    x0: f64,
    epsilon: f64,
    f: impl Fn(f64) -> f64,
    df: impl Fn(f64) -> f64,
) -> Result<MethodResult, Box<dyn Error>> {
    let df_x0 = df(x0);

    if df_x0 == 0.0 {
        return Err(
            "Производная в начальной точке равна нулю. Модифицированный метод Ньютона неприменим."
                .into(),
        );
    }

    let mut x_prev = x0;
    let mut iterations = 0;

    loop {
        iterations += 1;
        if iterations > MAX_ITERATIONS {
            return Err(
                "Модифицированный метод Ньютона не сошёлся за максимальное число итераций".into(),
            );
        }

        let f_x = f(x_prev);

        let x_next = x_prev - f_x / df_x0;

        if (x_next - x_prev).abs() <= epsilon {
            let residual = f64::abs(f(x_next));
            return Ok(MethodResult {
                name: "Метод Ньютона",
                iterations,
                root: x_next,
                difference: (x_next - x_prev).abs(),
                residual,
            });
        }

        x_prev = x_next;
    }
}
