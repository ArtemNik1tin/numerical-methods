use std::fmt::{Debug, Display};

pub struct MethodResult {
    pub name: &'static str,
    pub iterations: usize,
    pub root: f64,
    pub difference: f64,
    pub residual: f64,
}

impl Debug for MethodResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MethodResult")
            .field("name", &self.name)
            .field("root", &self.root)
            .field("difference", &self.difference)
            .field("residual", &self.residual)
            .finish()
    }
}

impl Display for MethodResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} {}", self.name, self.iterations))
    }
}
