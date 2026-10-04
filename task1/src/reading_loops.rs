use std::error::Error;
use std::io;
use crate::constants::MIN_EPSILON;

pub fn run_epsilon_reading_loop() -> f64 {
    loop {
        println!("Укажите точность (epsilon > 0):");
        let epsilon = run_f64_reading_loop();
        if epsilon <= MIN_EPSILON {
            println!("epsilon должно быть больше нуля");
            continue;
        }
        if epsilon < f64::EPSILON {
            println!("epsilon должно быть больше нуля");
            continue;
        }
        return epsilon;
    }
}

pub fn run_section_reading_loop() -> (f64, f64) {
    loop {
        println!("Введите начало отрезка (A)");
        let start_section = run_f64_reading_loop();
        println!("Введите конец отрезка (B)");
        let end_section = run_f64_reading_loop();
        if start_section > end_section {
            println!("Конец отрезка больше чем его начало");
            continue;
        } else {
            return (start_section, end_section);
        }
    }
}

pub fn run_f64_reading_loop() -> f64 {
    loop {
        match read_f64() {
            Ok(number) => {
                return number;
            }
            Err(error) => {
                println!("Error: {}", error);
                continue;
            }
        }
    }
}

pub fn run_usize_reading_loop() -> usize {
    loop {
        match read_usize() {
            Ok(number) => {
                return number;
            }
            Err(error) => {
                println!("Error: {}", error);
                continue;
            }
        }
    }
}

pub fn run_list_reading_loop(list_size: usize) -> usize {
    loop {
        match read_usize() {
            Ok(number) => {
                if number > list_size || number == 0 {
                    println!("Error: Число не содержится в списке");
                    continue;
                } else {
                    return number;
                }
            }
            Err(error) => {
                println!("Error: {}", error);
                continue;
            }
        }
    }
}

pub fn read_usize() -> Result<usize, Box<dyn Error>> {
    let mut raw_number = String::new();
    io::stdin().read_line(&mut raw_number)?;

    let trimmed = raw_number.trim();
    if trimmed.is_empty() {
        return Err("Ввод не может быть пустым".into());
    }

    let number = trimmed.parse()?;
    Ok(number)
}

pub fn read_f64() -> Result<f64, Box<dyn Error>> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    let trimmed = buffer.trim();
    if trimmed.is_empty() {
        return Err("Ввод не может быть пустым".into());
    }

    let number = trimmed.parse()?;
    Ok(number)
}
