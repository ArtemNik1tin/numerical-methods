use crate::ball_problem::solve_ball_problem;
use crate::bisection::*;
use crate::modified_newton::modified_newton;
use crate::newton::newton;
use crate::reading_loops::*;
use crate::secant::secant;
use crate::separation_of_roots::*;

mod ball_problem;
mod bisection;
mod constants;
mod method_result;
mod modified_newton;
mod newton;
mod reading_loops;
mod secant;
mod separation_of_roots;

fn main() {
    loop {
        println!("\nВыберите задачу:");
        println!("  1. Тестовая задача (x - 10*sin(x))");
        println!("  2. Задача о погружении шара");
        let task_number = run_list_reading_loop(2);
        match task_number {
            1 => solve_test_problem(),
            2 => solve_ball_problem(),
            0 => break,
            _ => println!("Неверный выбор."),
        }
    }
}

fn solve_test_problem() {
    let mut interval_len;
    let mut intervals;
    let (start_section, end_section) = run_section_reading_loop();
    println!("Введите число разбиений (N):");
    let number_of_partitions = run_usize_reading_loop();
    let (start_section, end_section) = loop {
        let h = (end_section - start_section) / number_of_partitions as f64;
        intervals = find_segments_with_sign_change(start_section, end_section, h, constants::f);

        interval_len = intervals.len();
        for (i, interval) in intervals.iter().enumerate() {
            println!("{}. [{}, {}]", i + 1, interval.0, interval.1);
        }
        println!("Перейти к уточнению корня или выбрать новое число разбиений?");
        println!("1. Уточнить");
        println!("2. Новое число разбиений");
        let choice = run_list_reading_loop(2);
        match choice {
            1 => {
                println!("Выберите интервал:");
                let num = run_list_reading_loop(intervals.len());
                break intervals[num - 1]
            }
            2 => {
                continue;
            }
            _ => {
                panic!();
            }
        }
    };

    let epsilon = run_epsilon_reading_loop();
    let x0 = (start_section + end_section) / 2.0;

    let mut results = Vec::new();

    match bisection(start_section, end_section, epsilon, constants::f) {
        Ok(result) => results.push(result),
        Err(err) => println!("Error: {}", err),
    }
    match newton(x0, epsilon, constants::f, constants::df) {
        Ok(result) => results.push(result),
        Err(err) => println!("Error: {}", err),
    }
    match modified_newton(x0, epsilon, constants::f, constants::df) {
        Ok(result) => results.push(result),
        Err(err) => println!("Error: {}", err),
    }
    match secant(start_section, end_section, epsilon, constants::f) {
        Ok(result) => results.push(result),
        Err(err) => println!("Error: {}", err),
    }

    for result in results {
        println!("{result}");
    }

    loop {
        for (i, interval) in intervals.iter().enumerate() {
            println!("{}. [{}, {}]", i + 1, interval.0, interval.1);
        }

        println!("Вернутся в главное меню или выбрать другой отрезок?");
        println!("1. Другой отрезок");
        println!("2. Главное меню");
        let chose = run_list_reading_loop(2);
        match chose {  
            1 => {
                println!("Выберите интервал:");
                let num = run_list_reading_loop(interval_len);
                let (start_section, end_section) = intervals[num - 1];
                let epsilon = run_epsilon_reading_loop();
                let x0 = (start_section + end_section) / 2.0;

                let mut results = Vec::new();

                match bisection(start_section, end_section, epsilon, constants::f) {
                    Ok(result) => results.push(result),
                    Err(err) => println!("Error: {}", err),
                }
                match newton(x0, epsilon, constants::f, constants::df) {
                    Ok(result) => results.push(result),
                    Err(err) => println!("Error: {}", err),
                }
                match modified_newton(x0, epsilon, constants::f, constants::df) {
                    Ok(result) => results.push(result),
                    Err(err) => println!("Error: {}", err),
                }
                match secant(start_section, end_section, epsilon, constants::f) {
                    Ok(result) => results.push(result),
                    Err(err) => println!("Error: {}", err),
                }

                for result in results {
                    println!("{result}");
                }
                continue;
            }
            2 => {
                break;
            }
            _ => {
                panic!();
            }
        }
    }
}
