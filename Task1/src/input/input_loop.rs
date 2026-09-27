use crate::input::input_error::InputError;
use crate::input::task_info::TaskInfo;
use crate::input::validators::{
    validate_epsilon, validate_method_number, validate_number_of_partitions,
    validate_section, validate_task_number,
};
use std::error::Error;
use std::io;

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

pub fn run_number_of_partitions_reading_loop() -> usize {
    loop {
        match read_number_of_partitions() {
            Ok(number) => break number,
            Err(err) => {
                print_error_message(err);
            }
        };
    }
}

fn run_section_reading_loop() -> f64 {
    loop {
        match read_f64() {
            Ok(number) => break number,
            Err(err) => {
                print_error_message(err);
            }
        };
    }
}

fn run_method_number_reading_loop() -> usize {
    loop {
        match read_method_number() {
            Ok(number) => break number,
            Err(err) => {
                print_error_message(err);
            }
        }
    }
}

fn run_task_number_reading_loop() -> usize {
    loop {
        match read_task_number() {
            Ok(number) => break number,
            Err(err) => {
                print_error_message(err);
            }
        };
    }
}

pub fn run_epsilon_reading_loop() -> f64 {
    loop {
        match read_epsilon() {
            Ok(number) => break number,
            Err(err) => {
                print_error_message(err);
            }
        };
    }
}

fn read_epsilon() -> Result<f64, Box<dyn Error>> {
    let epsilon = read_f64()?;
    validate_epsilon(epsilon)?;
    Ok(epsilon)
}

fn read_method_number() -> Result<usize, Box<dyn Error>> {
    let method_number = read_usize()?;
    validate_method_number(method_number)?;
    Ok(method_number)
}

fn read_task_number() -> Result<usize, Box<dyn Error>> {
    let task_number = read_usize()?;
    validate_task_number(task_number)?;
    Ok(task_number)
}

fn read_number_of_partitions() -> Result<usize, Box<dyn Error>> {
    let number_of_partitions = read_usize()?;
    validate_number_of_partitions(number_of_partitions)?;
    Ok(number_of_partitions)
}

pub fn read_usize() -> Result<usize, Box<dyn Error>> {
    let mut raw_number = String::new();
    io::stdin().read_line(&mut raw_number)?;

    let trimmed = raw_number.trim();
    if trimmed.is_empty() {
        return Err(Box::new(InputError::EmptyInput));
    }

    let number = trimmed.parse()?;
    Ok(number)
}

pub fn read_f64() -> Result<f64, Box<dyn Error>> {
    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer)?;

    let trimmed = buffer.trim();
    if trimmed.is_empty() {
        return Err(Box::new(InputError::EmptyInput));
    }

    let number = trimmed.parse()?;

    Ok(number)
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
