use crate::Node;

pub fn calculate_newton(
    selected_nodes: &[Node],
    x_target: f64,
    func: fn(f64) -> f64,
) -> (f64, f64, Vec<Vec<f64>>) {
    let n = selected_nodes.len() - 1;

    let mut dd_table = vec![vec![0.0; n + 1]; n + 1];

    for i in 0..=n {
        dd_table[i][0] = selected_nodes[i].fx;
    }

    for j in 1..=n {
        for i in 0..=(n - j) {
            let x_i = selected_nodes[i].x;
            let x_i_plus_j = selected_nodes[i + j].x;

            // f(x_i, ..., x_{i+j}) = (f(x_{i+1}, ...) - f(x_i, ...)) / (x_{i+j} - x_i)
            dd_table[i][j] = (dd_table[i + 1][j - 1] - dd_table[i][j - 1]) / (x_i_plus_j - x_i);
        }
    }

    let mut p_n_n = dd_table[0][0];
    let mut product_term = 1.0;

    for k in 1..=n {
        product_term *= x_target - selected_nodes[k - 1].x;
        p_n_n += dd_table[0][k] * product_term;
    }

    let exact_value = func(x_target);
    let absolute_error = (exact_value - p_n_n).abs();

    (p_n_n, absolute_error, dd_table)
}

pub fn print_divided_differences(selected_nodes: &[Node], dd_table: &[Vec<f64>]) {
    let n = selected_nodes.len() - 1;
    println!("\nТаблица разделенных разностей (форма Ньютона)");
    println!(
        "{:<10} | {:<12} | Коэффициенты и РР по порядку:",
        "x_i", "f(x_i)"
    );
    println!("--------------------------------------------------------------------------------");

    for i in 0..=n {
        print!(
            "{:<10.5} | {:<12.5} | ",
            selected_nodes[i].x, selected_nodes[i].fx
        );
        for j in 1..=(n - i) {
            print!("{:<12.5} ", dd_table[i][j]);
        }
        println!();
    }
}
