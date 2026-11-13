mod Lagrange;
mod Newton;
mod input_helpers;
mod prepare_utils;
mod solution;

use crate::input_helpers::read_usize;
use Lagrange::*;
use Newton::*;
use prepare_utils::*;
use solution::*;

#[derive(Debug, Clone)]
struct Node {
    x: f64,
    fx: f64,
}

fn main() {
    'main_menu: loop {
        // Этап 1
        let func = select_function();

        let (a, b) = input_boundaries();

        let m_plus_1 = input_table_size();
        let m = m_plus_1 - 1;

        let node_type = input_node_type();
        let mut table = generate_table(func, a, b, m_plus_1, node_type);

        print_table("Исходная таблица значений z_k | f(z_k)", &table);

        'calculation_loop: loop {
            // Этап 2
            let x_target = input_x_target();
            let n = input_polynomial_degree(m);

            sort_table_by_distance(&mut table, x_target);
            print_sorted_table_with_selection(&table, x_target, n);

            // Вычисляем по Лагранжу
            let selected_nodes = &table[..=n];
            let (p_n_l, sum_l, lebesgue_val, err_l) =
                calculate_lagrange(selected_nodes, x_target, func);

            println!("\nРезультаты вычислений в форме Лагранжа:");
            println!("Значение многочлена Лагранжа P_n^L(x) = {:.16}", p_n_l);
            println!(
                "Точное значение функции f(x)             = {:.16}",
                func(x_target)
            );

            println!("\nСумма лагранжевых коэффициентов: {:.16}", sum_l);
            println!("Значение функции Лебега lambda_n(x): {:.16}", lebesgue_val);

            println!("Абсолютная погрешность e_n^L(f)          = {:.16}", err_l);

            // Вычисления по методу Ньютона
            let (p_n_n, err_n, dd_table) = calculate_newton(selected_nodes, x_target, func);

            print_divided_differences(selected_nodes, &dd_table);

            println!("\nРезультаты вычислений в форме Ньютона:");
            println!("Значение многочлена Ньютона P_n^N(x) = {:.16}", p_n_n);
            println!("Абсолютная погрешность e_n^N(f)          = {:.16}", err_n);

            let diff_lagrange_newton = (p_n_l - p_n_n).abs();
            println!(
                "\nРазность между Лагранжем и Ньютоном |P_n^L(x) - P_n^N(x)| = {:.2e}",
                diff_lagrange_newton
            );

            println!("1. Ввести новую точку x и степень n (с текущими узлами)");
            println!("2. Вернуться в главное меню (сменить функцию/узлы)");
            println!("0. Выйти из программы");
            print!("Ваш выбор (0-2): ");

            let post_choice = read_usize();
            match post_choice {
                1 => {
                    println!("\n> Переход к вводу новых параметров x и n...");
                    continue 'calculation_loop;
                }
                2 => {
                    println!("\n> Возврат в главное меню...\n");
                    continue 'main_menu;
                }
                _ => {
                    println!("Завершение работы программы. До свидания!");
                    break 'main_menu;
                }
            }
        }
    }
}
