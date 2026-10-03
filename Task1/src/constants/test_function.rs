pub const AMPLITUDE: f64 = 10.0;

pub fn f(x: f64) -> f64 {
    x - AMPLITUDE * x.sin()
}

pub fn df(x: f64) -> f64 {
    1.0 - AMPLITUDE * x.cos()
}
