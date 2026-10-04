use crate::bisection::*;
use crate::modified_newton::modified_newton;
use crate::newton::newton;
use crate::reading_loops::*;
use crate::secant::secant;

/// Уравнение задачи о погружении шара:
/// f(d) = d^2 * (3r - d) - 4 * rho * r^3 = 0
/// где d — глубина погружения, r — радиус шара, rho — плотность материала
pub fn ball_f(d: f64, r: f64, rho: f64) -> f64 {
    d * d * (3.0 * r - d) - 4.0 * rho * r * r * r
}

/// Производная уравнения шара по d:
/// f'(d) = 3d(2r - d)
pub fn ball_df(d: f64, r: f64, _rho: f64) -> f64 {
    3.0 * d * (2.0 * r - d)
}

pub fn solve_ball_problem() {
    loop {
        println!("Введите радиус шара (r > 0, в метрах):");
        let r = run_f64_reading_loop();
        if r <= 0.0 {
            println!("Радиус должен быть положительным.");
            continue;
        }

        println!("Введите плотность материала шара (0 < rho < 1, в долях плотности воды):");
        let rho = run_f64_reading_loop();
        if rho <= 0.0 || rho >= 1.0 {
            println!("Плотность должна быть в интервале (0, 1).");
            continue;
        }

        let epsilon = run_epsilon_reading_loop();

        // Отрезок поиска: d ∈ [0, 2r]
        let a = 0.0;
        let b = 2.0 * r;

        let f = |d: f64| ball_f(d, r, rho);
        let df = |d: f64| ball_df(d, r, rho);

        let x0 = (a + b) / 2.0;

        let mut results = Vec::new();

        match bisection(a, b, epsilon, f) {
            Ok(result) => results.push(result),
            Err(err) => println!("Error: {}", err),
        }
        match newton(x0, epsilon, f, df) {
            Ok(result) => results.push(result),
            Err(err) => println!("Error: {}", err),
        }
        match modified_newton(x0, epsilon, f, df) {
            Ok(result) => results.push(result),
            Err(err) => println!("Error: {}", err),
        }
        match secant(a, b, epsilon, f) {
            Ok(result) => results.push(result),
            Err(err) => println!("Error: {}", err),
        }

        println!("\nРезультаты (r = {} м, rho = {}, epsilon = {}):", r, rho, epsilon);
        for result in &results {
            println!("{result}");
        }

        if let Some(best) = results.iter().min_by(|a, b| a.iterations.cmp(&b.iterations)) {
            println!(
                "\nСамый быстрый метод: {} ({} итераций)",
                best.name, best.iterations
            );
        }

        println!("\nРешить для новых значений радиуса? (2 — да, 1 — выход):");
        let choice = run_list_reading_loop(2);
        if choice == 1 {
            break;
        }
    }
}
