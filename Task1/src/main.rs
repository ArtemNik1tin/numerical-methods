mod constants;
mod input;
mod test_task;

use crate::constants::{
    sphere::{DEFAULT_EPSILON as SPHERE_DEFAULT_EPSILON, WOOD_MATERIALS},
    test_function::{df as test_function_derivative, f as test_function},
};
use crate::input::{
    input_loop::{
        print_error_message, run_epsilon_reading_loop, run_number_of_partitions_reading_loop,
        run_usize_reading_loop,
    },
    task_info::TaskInfo,
};
use crate::test_task::{
    bisection::bisection,
    method_result::{print_results_table, print_task_info},
    newton::{modified_newton, newton, secant},
    separation_of_roots::find_number_of_segments_with_sign_change,
    sphere::{calculate_depth_for_materials, print_results_table as print_sphere_results},
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
    let f = test_function;
    let df = test_function_derivative;

    let x0 = (start_section + end_section) / 2.0;

    print_task_info(
        "УТОЧНЕНИЕ КОРНЯ НА ОТРЕЗКЕ",
        start_section,
        end_section,
        epsilon,
        "f(x) = x - 10*sin(x)",
    );

    let mut results = Vec::new();

    match method_number {
        1 => match bisection(start_section, end_section, epsilon, f) {
            Ok(result) => results.push(result),
            Err(err) => println!("Ошибка: {}", err),
        },
        2 => match newton(x0, epsilon, f, df) {
            Ok(result) => results.push(result),
            Err(err) => println!("Ошибка: {}", err),
        },
        3 => match modified_newton(x0, epsilon, f, df) {
            Ok(result) => results.push(result),
            Err(err) => println!("Ошибка: {}", err),
        },
        4 => match secant(start_section, end_section, epsilon, f) {
            Ok(result) => results.push(result),
            Err(err) => println!("Ошибка: {}", err),
        },
        _ => {
            println!("Invalid method number");
        }
    }

    if !results.is_empty() {
        print_results_table(&results);
    }
}

fn run_find_segment_loop(start_section: f64, end_section: f64) -> (f64, f64) {
    loop {
        let (counter, segments, h) =
            match find_segments_with_sign_change(start_section, end_section) {
                Ok(result) => result,
                Err(_) => continue,
            };

        if counter == 0 {
            println!("Отрезки с переменой знака не найдены. Попробуйте другое число разбиений.");
            continue;
        }

        println!("Найдено {counter} отрезков перемены знака с шагом h={h}");

        if let Some(segment) = select_segment(&segments, counter) {
            return segment;
        }
    }
}

fn find_segments_with_sign_change(
    start_section: f64,
    end_section: f64,
) -> Result<(usize, Vec<(f64, f64)>, f64), Box<dyn std::error::Error>> {
    println!("Введите число разбиений (N):");
    let number_of_partitions = run_number_of_partitions_reading_loop();
    let h = (end_section - start_section) / number_of_partitions as f64;

    match find_number_of_segments_with_sign_change(
        start_section,
        end_section,
        number_of_partitions,
        h,
        test_function,
    ) {
        Ok((counter, segments)) => Ok((counter, segments, h)),
        Err(err) => {
            print_error_message(err);
            Err("Ошибка при поиске отрезков".into())
        }
    }
}

fn select_segment(segments: &[(f64, f64)], counter: usize) -> Option<(f64, f64)> {
    loop {
        print_segments(segments);

        println!("Запросить новое число разбиений N -- 1");
        println!("Перейти к уточнению корней -- 2");

        let user_choose = run_usize_reading_loop();

        match user_choose {
            1 => return None,
            2 => {
                let segment_number = run_usize_reading_loop();
                if segment_number < 1 || segment_number > counter {
                    println!("Неверный номер отрезка. Допустимые значения: 1..={counter}");
                    continue;
                }
                return Some(segments[segment_number - 1]);
            }
            _ => {
                println!("Неверный выбор. Попробуйте снова.");
            }
        }
    }
}

fn print_segments(segments: &[(f64, f64)]) {
    for (i, (start, end)) in segments.iter().enumerate() {
        println!("Отрезок {}: ({}, {})", i + 1, start, end);
    }
}

fn run_sphere_task() {
    println!("ЗАДАЧА О ПОГРУЖЕНИИ ШАРА");
    println!("Введите радиус шара в метрах:");

    let radius = loop {
        match input::reading::read_f64() {
            Ok(r) if r > 0.0 => break r,
            Ok(_) => println!("Радиус должен быть положительным. Попробуйте снова:"),
            Err(err) => print_error_message(err),
        }
    };

    let materials: Vec<(String, f64)> = WOOD_MATERIALS
        .iter()
        .map(|(name, density)| (name.to_string(), *density))
        .collect();
    let epsilon = SPHERE_DEFAULT_EPSILON;

    match calculate_depth_for_materials(radius, &materials, epsilon) {
        Ok(results) => print_sphere_results(&results, radius),
        Err(err) => println!("Ошибка: {}", err),
    }
}
