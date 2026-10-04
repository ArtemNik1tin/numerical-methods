pub fn f(x: f64) -> f64 {
    x - 10.0 * x.sin()
}

pub fn df(x: f64) -> f64 {
    1.0 - 10.0 * x.cos()
}

pub const MAX_ITERATIONS: usize = 1000;
pub const MIN_EPSILON: f64 = 0.0;
