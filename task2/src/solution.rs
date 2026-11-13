use crate::input_helpers::{read_f64, read_usize};
use crate::Node;

pub fn input_x_target() -> f64 {
    println!("Введите точку интерполирования x:");
    read_f64()
}

pub fn input_polynomial_degree(m: usize) -> usize {
    loop {
        println!("Введите степень интерполяционного многочлена n (n <= {}):", m);
        let n = read_usize();

        if n > m {
            // Строго по методичке: сообщаем об ошибке и просим ввести заново
            println!("«Введено недопустимое значение n». Пожалуйста, введите n <= {}.", m);
        } else {
            return n;
        }
    }
}

pub fn sort_table_by_distance(table: &mut [Node], x_target: f64) {
    table.sort_by(|node_a, node_b| {
        let dist_a = (node_a.x - x_target).abs();
        let dist_b = (node_b.x - x_target).abs();
        dist_a.total_cmp(&dist_b)
    });
}

pub fn print_sorted_table_with_selection(table: &[Node], x_target: f64, n: usize) {
    let required_nodes_count = n + 1;
    println!("\nТаблица после сортировки по удалению от точки x = {:.5}:", x_target);
    println!("{:<5} | {:<12} | {:<12} | {:<15}", "Индекс", "x_k", "f(x_k)", "Расстояние до x");
    println!("---------------------------------------------------------");

    for (i, node) in table.iter().enumerate() {
        let dist = (node.x - x_target).abs();
        let marker = if i < required_nodes_count { "*" } else { " " };
        println!("{:<5} | {:<12.5} | {:<12.5} | {:<15.5} {}", i, node.x, node.fx, dist, marker);
    }
    println!("(* — первые {} узлов, выбранные для построения полинома степени {})", required_nodes_count, n);
}