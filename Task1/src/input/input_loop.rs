use std::error::Error;

use super::reading;
use super::task_info::TaskInfo;
use super::validators::validate_section;

pub fn run_input_loop() -> TaskInfo {
    print_invitation();
    let task_number = run_task_number_reading_loop();

    print_method_numbers();
    let method_number = run_method_number_reading_loop();

    let (start_section, end_section) = loop {
        println!("Введите начало отрезка (А):");
        let start_section = run_section_reading_loop();
        println!("Введите конец отрезка (В):");
        let end_section = run_section_reading_loop();

        match validate_section(start_section, end_section) {
            Ok(_) => break (start_section, end_section),
            Err(err) => {
                print_error_message(err);
            }
        }
    };

    TaskInfo::new(task_number, method_number, start_section, end_section)
}

pub fn run_usize_reading_loop() -> usize {
    loop {
        match reading::read_usize() {
            Ok(number) => break number,
            Err(err) => {
                print_error_message(err);
            }
        };
    }
}

pub fn run_number_of_partitions_reading_loop() -> usize {
    loop {
        match reading::read_number_of_partitions() {
            Ok(number) => break number,
            Err(err) => {
                print_error_message(err);
            }
        };
    }
}

fn run_section_reading_loop() -> f64 {
    loop {
        match reading::read_f64() {
            Ok(number) => break number,
            Err(err) => {
                print_error_message(err);
            }
        };
    }
}

fn run_method_number_reading_loop() -> usize {
    loop {
        match reading::read_method_number() {
            Ok(number) => break number,
            Err(err) => {
                print_error_message(err);
            }
        }
    }
}

fn run_task_number_reading_loop() -> usize {
    loop {
        match reading::read_task_number() {
            Ok(number) => break number,
            Err(err) => {
                print_error_message(err);
            }
        };
    }
}

pub fn run_epsilon_reading_loop() -> f64 {
    loop {
        match reading::read_epsilon() {
            Ok(number) => break number,
            Err(err) => {
                print_error_message(err);
            }
        };
    }
}

fn print_method_numbers() {
    println!("Введите номер метода решения:");
    println!("1 - Метод бисекции");
    println!("2 - Метод ньютона (касательных)");
    println!("3 - Модифицированный метод Ньютона");
    println!("4 - Метод секущих");
}

fn print_invitation() {
    println!("Выберите задачу:");
    println!("1 - Тестовая задача");
    println!("2 - Погружение шара");
}

pub fn print_error_message(err: Box<dyn Error>) {
    println!("Ошибка ввода: {}", err);
    println!("Попробуйте ещё раз:");
}
