pub struct MethodResult {
    pub name: &'static str,
    pub initial_approximation: String,
    pub iterations: usize,
    pub root: f64,
    pub difference: f64,
    pub residual: f64,
}

impl MethodResult {
    pub fn new(
        name: &'static str,
        initial_approximation: String,
        iterations: usize,
        root: f64,
        difference: f64,
        residual: f64,
    ) -> Self {
        MethodResult {
            name,
            initial_approximation,
            iterations,
            root,
            difference,
            residual,
        }
    }
}

pub fn print_results_table(results: &[MethodResult]) {
    println!("\n{:-^80}", "");
    println!("{:^80}", "РЕЗУЛЬТАТЫ УТОЧНЕНИЯ КОРНЯ");
    println!("{:-^80}", "");
    println!(
        "{:<20} {:>12} {:>16} {:>12} {:>12}",
        "Метод", "Шаги", "Корень", "|xm-xm-1|", "Невязка"
    );
    println!("{:-^80}", "");

    for result in results {
        println!(
            "{:<20} {:>12} {:>16.14} {:>12.2e} {:>12.2e}",
            result.name,
            result.iterations,
            result.root,
            result.difference,
            result.residual
        );
    }

    println!("{:-^80}", "");
    println!("Начальные приближения:");
    for result in results {
        println!("  {}: {}", result.name, result.initial_approximation);
    }

    println!("{:-^80}", "");
}

pub fn print_task_info(
    task_name: &str,
    start_section: f64,
    end_section: f64,
    epsilon: f64,
    function_str: &str,
) {
    println!("\n{:=^80}", "");
    println!("{:^80}", task_name);
    println!("{:=^80}", "");
    println!("Функция: {}", function_str);
    println!("Отрезка: [{}; {}]", start_section, end_section);
    println!("Точность: {:e}", epsilon);
    println!();
}
