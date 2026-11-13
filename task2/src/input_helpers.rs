use std::io;

pub fn read_f64() -> f64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка чтения ввода");
    input.trim().parse().expect("Введите корректное число с плавающей точкой")
}

pub fn read_usize() -> usize {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка чтения ввода");
    input.trim().parse().expect("Введите корректное целое число")
}