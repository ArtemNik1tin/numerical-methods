use std::error::Error;

const WATER_DENSITY: f64 = 1000.0; // кг/м³

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
        4.0 * rho * r.powi(3) - WATER_DENSITY * d.powi(2) * (3.0 * r - d)
    };

    let a = 0.0;
    let b = 2.0 * r;

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

pub fn get_wood_materials() -> Vec<(String, f64)> {
    vec![
        ("Пробка".to_string(), 250.0),
        ("Бамбук".to_string(), 400.0),
        ("Сосна (белая)".to_string(), 500.0),
        ("Кедр".to_string(), 550.0),
        ("Дуб".to_string(), 700.0),
        ("Бук".to_string(), 750.0),
        ("Красное дерево".to_string(), 800.0),
        ("Тиковое дерево".to_string(), 850.0),
        ("Парафин".to_string(), 900.0),
        ("Полиэтилен".to_string(), 920.0),
        ("Пчелиный воск".to_string(), 950.0),
    ]
}

pub fn print_results_table(results: &[(String, f64, f64, f64, usize)], radius: f64) {
    println!("\n{}", "=".repeat(80));
    println!("ЗАДАЧА О ПОГРУЖЕНИИ ШАРА");
    println!("{}", "=".repeat(80));
    println!("Радиус шара: {:.2} м", radius);
    println!("Плотность воды: {} кг/м³", WATER_DENSITY);
    println!("Точность: 1e-6");
    println!();

    println!(
        "{:<20} {:>12} {:>16} {:>12} {:>12}",
        "Материал", "Плотность", "Глубина (м)", "Шаги", "Невязка"
    );
    println!("{}", "-".repeat(80));

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

    println!("{}", "=".repeat(80));
}
