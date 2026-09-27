mod input;
mod test_task;

use crate::{
    input::{
        input_loop::run_epsilon_reading_loop,
        input_loop::{print_error_message, read_usize, run_number_of_partitions_reading_loop},
        task_info::TaskInfo,
    },
    test_task::{
        bisection::bisection,
        newton::{modified_newton, newton, secant},
        separation_of_roots::find_number_of_segments_with_sign_change,
        sphere::{calculate_depth_for_materials, get_wood_materials, print_results_table},
    },
};

fn main() {
    let TaskInfo {
        task_number,
        method_number,
        start_section,
        end_section,
    } = input::input_loop::run_input_loop();
    match_task_number(task_number, start_section, end_section, method_number)
}

fn match_task_number(
    task_number: usize,
    start_section: f64,
    end_section: f64,
    method_number: usize,
) {
    match task_number {
        1 => {
            let (start, end) = run_find_segment_loop(start_section, end_section);
            println!("Выбранный отрезок: ({}, {})", start, end);
            println!("Укажите точность (epsilon > 0):");
            let epsilon = run_epsilon_reading_loop();
            match_method_number(method_number, start, end, epsilon);
        }
        2 => {
            run_sphere_task();
        }
        _ => {
            println!("Invalid task number");
        }
    }
}

fn match_method_number(method_number: usize, start_section: f64, end_section: f64, epsilon: f64) {
    let f = |x: f64| x - 10.0 * x.sin();
    let df = |x: f64| 1.0 - 10.0 * x.cos();

    let x0 = (start_section + end_section) / 2.0;

    println!(
        "УТОЧНЕНИЕ КОРНЯ НА ОТРЕЗКЕ [{}; {}]",
        start_section, end_section
    );
    println!("Точность: {}", epsilon);

    match method_number {
        1 => match bisection(start_section, end_section, epsilon, f) {
            Ok(result) => result.print(),
            Err(err) => println!("Ошибка: {}", err),
        },
        2 => match newton(x0, epsilon, f, df) {
            Ok(result) => result.print(),
            Err(err) => println!("Ошибка: {}", err),
        },
        3 => match modified_newton(x0, epsilon, f, df) {
            Ok(result) => result.print(),
            Err(err) => println!("Ошибка: {}", err),
        },
        4 => match secant(start_section, end_section, epsilon, f) {
            Ok(result) => result.print(),
            Err(err) => println!("Ошибка: {}", err),
        },
        _ => {
            println!("Invalid method number");
        }
    }
}

fn run_find_segment_loop(start_section: f64, end_section: f64) -> (f64, f64) {
    loop {
        println!("Введите число разбиений (N):");
        let number_of_partitions = run_number_of_partitions_reading_loop();
        let h = (end_section - start_section) / number_of_partitions as f64;

        match find_number_of_segments_with_sign_change(
            start_section,
            end_section,
            number_of_partitions,
            h,
            |x| x - 10.0 * x.sin(),
        ) {
            Ok((counter, segments)) => {
                if counter == 0 {
                    println!(
                        "Отрезки с переменой знака не найдены. Попробуйте другое число разбиений."
                    );
                    continue;
                }

                println!("Найдено {counter} отрезков перемены знака с шагом h={h}");

                loop {
                    print_segments(&segments);

                    println!("Запросить новое число разбиений N -- 1");
                    println!("Перейти к уточнению корней -- 2");

                    let user_choose = read_clean_usize();

                    match user_choose {
                        1 => break,
                        2 => {
                            let segment_number = read_clean_usize();
                            if segment_number < 1 || segment_number > counter {
                                println!(
                                    "Неверный номер отрезка. Допустимые значения: 1..={counter}"
                                );
                                continue;
                            }
                            return segments[segment_number - 1];
                        }
                        _ => {
                            println!("Неверный выбор. Попробуйте снова.");
                        }
                    }
                }
            }
            Err(err) => {
                print_error_message(err);
            }
        }
    }
}

fn print_segments(segments: &[(f64, f64)]) {
    for (i, (start, end)) in segments.iter().enumerate() {
        println!("Отрезок {}: ({}, {})", i + 1, start, end);
    }
}

fn read_clean_usize() -> usize {
    loop {
        match read_usize() {
            Ok(segment_number) => break segment_number,
            Err(err) => {
                print_error_message(err);
            }
        };
    }
}

fn run_sphere_task() {
    println!("ЗАДАЧА О ПОГРУЖЕНИИ ШАРА");
    println!("Введите радиус шара в метрах:");

    let radius = loop {
        match read_f64_input() {
            Ok(r) if r > 0.0 => break r,
            Ok(_) => println!("Радиус должен быть положительным. Попробуйте снова:"),
            Err(err) => print_error_message(err),
        }
    };

    let materials = get_wood_materials();
    let epsilon = 1e-6;

    match calculate_depth_for_materials(radius, &materials, epsilon) {
        Ok(results) => print_results_table(&results, radius),
        Err(err) => println!("Ошибка: {}", err),
    }
}

fn read_f64_input() -> Result<f64, Box<dyn std::error::Error>> {
    use std::io;
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let trimmed = buffer.trim();
    if trimmed.is_empty() {
        return Err("Ввод не может быть пустым".into());
    }
    Ok(trimmed.parse()?)
}
