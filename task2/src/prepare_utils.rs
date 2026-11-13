use rand::Rng;
use crate::input_helpers::{read_f64, read_usize};
use crate::Node;

pub fn select_function() -> fn(f64) -> f64 {
    println!("\nВыберите вариант функции:");
    println!("1. f(x) = sin(x) - x^2 / 2  (Вариант 1)");
    println!("2. f(x) = ln(1 + x) - 2 * x (Вариант 2)");
    println!("3. f(x) = exp(x) - x        (Вариант 3)");
    println!("4. f(x) = x^2 - x - 2       (Тестовый многочлен)");
    print!("Ваш выбор (1-4): ");

    let choice = read_usize();
    match choice {
        1 => |x| x.sin() - (x.powi(2) / 2.0),
        2 => |x| (1.0 + x).ln() - 2.0 * x,
        3 => |x| x.exp() - x,
        4 => |x| x.powi(2) - x - 2.0,
        _ => {
            println!("Неверный выбор, используется функция по умолчанию (Вариант 1).");
            |x| x.sin() - (x.powi(2) / 2.0)
        }
    }
}

pub fn input_boundaries() -> (f64, f64) {
    println!("\nВведите левую границу отрезка a:");
    let a = read_f64();

    println!("Введите правую границу отрезка b (b > a):");
    let b = read_f64();
    if b <= a {
        println!("Предупреждение: правая граница меньше или равна левой. Убедитесь в корректности данных.");
    }
    (a, b)
}

pub fn input_table_size() -> usize {
    println!("Введите общее количество точек в таблице (m + 1):");
    let m_plus_1 = read_usize();
    if m_plus_1 < 2 {
        panic!("Ошибка: количество точек должно быть не менее 2.");
    }
    m_plus_1
}

pub fn input_node_type() -> usize {
    println!("\nВыберите характер узлов:");
    println!("1. Равноотстоящие");
    println!("2. Случайные (рандомные)");
    println!("3. Ручной ввод");
    print!("Ваш выбор (1-3): ");
    read_usize()
}

pub fn generate_table(func: fn(f64) -> f64, a: f64, b: f64, m_plus_1: usize, node_type: usize) -> Vec<Node> {
    let mut table = Vec::with_capacity(m_plus_1);
    let m = m_plus_1 - 1;

    match node_type {
        1 => {
            for k in 0..m_plus_1 {
                let z_k = a + ((b - a) / (m as f64)) * (k as f64);
                table.push(Node { x: z_k, fx: func(z_k) });
            }
        },
        2 => {
            let mut rng = rand::thread_rng();
            let mut count = 0;
            while count < m_plus_1 {
                let z_k: f64 = rng.gen_range(a..=b);
                let is_unique = table.iter().all(|node| (node.x - z_k).abs() > 1e-12);

                if is_unique || count == 0 {
                    table.push(Node { x: z_k, fx: func(z_k) });
                    count += 1;
                }
            }
            table.sort_by(|n1, n2| n1.x.total_cmp(&n2.x));
        },
        3 => {
            println!("\nВведите {} попарно различных значений узлов x:", m_plus_1);
            let mut count = 0;
            while count < m_plus_1 {
                print!("Введите x_{} = ", count);
                let z_k = read_f64();

                let is_unique = table.iter().all(|node| (node.x - z_k).abs() > 1e-12);
                if is_unique || count == 0 {
                    table.push(Node { x: z_k, fx: func(z_k) });
                    count += 1;
                } else {
                    println!("Ошибка: такой узел уже введен. Узлы должны быть попарно разными!");
                }
            }
        },
        _ => {
            println!("Неверный выбор, используются равноотстоящие узлы по умолчанию.");
            for k in 0..m_plus_1 {
                let z_k = a + ((b - a) / (m as f64)) * (k as f64);
                table.push(Node { x: z_k, fx: func(z_k) });
            }
        }
    }
    table
}

pub fn print_table(title: &str, table: &[Node]) {
    println!("\n{}:", title);
    println!("{:<5} | {:<15} | {:<15}", "k", "z_k", "f(z_k)");
    println!("-----------------------------------------");
    for (k, node) in table.iter().enumerate() {
        println!("{:<5} | {:<15.5} | {:<15.5}", k, node.x, node.fx);
    }
}