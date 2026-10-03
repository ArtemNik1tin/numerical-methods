use std::error::Error;

use crate::constants::sphere::{
    DEFAULT_EPSILON as SPHERE_DEFAULT_EPSILON, MAX_DEPTH_FACTOR, SUBMERSION_COEFFICIENT,
    TABLE_WIDTH, VOLUME_COEFFICIENT, WATER_DENSITY, WOOD_MATERIALS,
};

pub struct SphereParams {
    pub radius: f64,           // метры
    pub density: f64,          // кг/м³
}

impl SphereParams {
    pub fn new(radius: f64, density: f64) -> Result<Self, Box<dyn Error>> {
        if radius <= 0.0 {
            return Err("Радиус шара должен быть положительным".into());
        }
        if density <= 0.0 {
            return Err("Плотность шара должна быть положительной".into());
        }
        Ok(SphereParams { radius, density })
    }
}

pub fn find_submersion_depth(
    params: &SphereParams,
    epsilon: f64,
) -> Result<(f64, f64, usize), Box<dyn Error>> {
    let r = params.radius;
    let rho = params.density;

    let f = |d: f64| -> f64 {
        VOLUME_COEFFICIENT * rho * r.powi(3)
            - WATER_DENSITY * d.powi(2) * (SUBMERSION_COEFFICIENT * r - d)
    };

    let a = 0.0;
    let b = MAX_DEPTH_FACTOR * r;

    let result = crate::test_task::bisection::bisection(a, b, epsilon, f)?;

    Ok((result.root, result.residual, result.iterations))
}

pub fn calculate_depth_for_materials(
    radius: f64,
    materials: &[(String, f64)],
    epsilon: f64,
) -> Result<Vec<(String, f64, f64, f64, usize)>, Box<dyn Error>> {
    let mut results = Vec::new();

    for (name, density) in materials {
        let params = SphereParams::new(radius, *density)?;
        let (depth, residual, iterations) = find_submersion_depth(&params, epsilon)?;
        results.push((name.clone(), *density, depth, residual, iterations));
    }

    Ok(results)
}

#[allow(dead_code)]
pub fn get_wood_materials() -> Vec<(String, f64)> {
    WOOD_MATERIALS
        .iter()
        .map(|(name, density)| (name.to_string(), *density))
        .collect()
}

pub fn print_results_table(results: &[(String, f64, f64, f64, usize)], radius: f64) {
    println!("\n{}", "=".repeat(TABLE_WIDTH));
    println!("ЗАДАЧА О ПОГРУЖЕНИИ ШАРА");
    println!("{}", "=".repeat(TABLE_WIDTH));
    println!("Радиус шара: {:.2} м", radius);
    println!("Плотность воды: {} кг/м³", WATER_DENSITY);
    println!("Точность: {:e}", SPHERE_DEFAULT_EPSILON);
    println!();

    println!(
        "{:<20} {:>12} {:>16} {:>12} {:>12}",
        "Материал", "Плотность", "Глубина (м)", "Шаги", "Невязка"
    );
    println!("{}", "-".repeat(TABLE_WIDTH));

    for (name, density, depth, residual, iterations) in results {
        println!(
            "{:<20} {:>12.0} {:>16.6} {:>12} {:>12.2e}",
            name,
            density,
            depth,
            iterations,
            residual
        );
    }

    println!("{}", "=".repeat(TABLE_WIDTH));
}
