use std::fmt::Display;

pub struct MethodResult {
    pub name: &'static str,
    pub initial_guess: f64,
    pub iterations: usize,
    pub root: f64,
    pub difference: f64,
    pub residual: f64,
}

impl Display for MethodResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}: начальное приближение = {:.6}, шагов = {}, корень = {:.10}, |x_m - x_(m-1)| = {:.6e}, |f(x_m)| = {:.6e}",
            self.name, self.initial_guess, self.iterations, self.root, self.difference, self.residual
        ))
    }
}
