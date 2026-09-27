use std::error::Error;
use std::io;
use crate::input::input_error::InputError;
use crate::input::task_info::TaskInfo;

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
        if end_section > start_section {
            break (start_section, end_section);
        } else {
            println!("Конец отрезка должен быть больше начала");
        }
    };

    println!("Введите число разбиений (N):");
    let number_of_partitions = run_number_of_partitions_reading_loop();

    TaskInfo::new(
        task_number,
        method_number,
        start_section,
        end_section,
        number_of_partitions,
    )
}

fn run_number_of_partitions_reading_loop() -> usize {
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

fn read_method_number() -> Result<usize, Box<dyn Error>> {
    let method_number = read_usize()?;
    if method_number >= 5 {
        return Err(Box::new(InputError::InvalidChoice));
    }
    Ok(method_number)
}

fn read_task_number() -> Result<usize, Box<dyn Error>> {
    let task_number = read_usize()?;
    if (task_number != 1) && (task_number != 2) {
        return Err(Box::new(InputError::InvalidChoice));
    }
    Ok(task_number)
}

fn read_number_of_partitions() -> Result<usize, Box<dyn Error>> {
    let number_of_partitions = read_usize()?;
    if number_of_partitions == 1 {
        return Err(Box::new(InputError::InvalidNumberOfPartitions));
    }
    Ok(number_of_partitions)
}

fn read_usize() -> Result<usize, Box<dyn Error>> {
    let mut raw_number = String::new();
    io::stdin().read_line(&mut raw_number)?;

    let number = raw_number.trim().parse()?;
    Ok(number)
}

fn read_f64() -> Result<f64, Box<dyn Error>> {
    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer)?;
    let number = buffer.trim().parse()?;

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

fn print_error_message(err: Box<dyn Error>) {
    println!("Ошибка ввода: {}", err);
    println!("Попробуйте ещё раз:");
}
