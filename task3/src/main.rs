use std::io::{self, Write};

struct TestFunction {
    name: String,
    f: fn(f64) -> f64,
    df: fn(f64) -> f64,
    d2f: fn(f64) -> f64,
}

fn main() {
    let functions = vec![
        TestFunction {
            name: "f1(x) = 2x^2 - 3x + 1".to_string(),
            f: |x| 2.0 * x * x - 3.0 * x + 1.0,
            df: |x| 4.0 * x - 3.0,
            d2f: |_x| 4.0,
        },
        TestFunction {
            name: "f2(x) = x^3 - 2x^2 + x - 5".to_string(),
            f: |x| x.powi(3) - 2.0 * x.powi(2) + x - 5.0,
            df: |x| 3.0 * x * x - 4.0 * x + 1.0,
            d2f: |x| 6.0 * x - 4.0,
        },
        TestFunction {
            name: "f3(x) = e^(4x)".to_string(),
            f: |x| (4.0 * x).exp(),
            df: |x| 4.0 * (4.0 * x).exp(),
            d2f: |x| 16.0 * (4.0 * x).exp(),
        },
        TestFunction {
            name: "f4(x) = sin(2x) - 1.25x^2 + 0.35".to_string(),
            f: |x| (2.0 * x).sin() - 1.25 * x * x + 0.35,
            df: |x| 2.0 * (2.0 * x).cos() - 2.5 * x,
            d2f: |x| -4.0 * (2.0 * x).sin() - 2.5,
        },
    ];

    'main: loop {
        for (i, func) in functions.iter().enumerate() {
            println!("{}. {}", i + 1, func.name);
        }
        println!("0. Выход");
        print!("Выберите номер функции (1-4): ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: usize = match choice.trim().parse() {
            Ok(0) => break,
            Ok(n) if n >= 1 && n <= functions.len() => n - 1,
            _ => {
                println!("Неверный ввод. Попробуйте снова.");
                continue;
            }
        };

        let selected_func = &functions[choice];

        'table: loop {
            print!("Введите количество точек в таблице (m + 1), где m >= 4: ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let num_points: usize = match input.trim().parse() {
                Ok(n) if n >= 5 => n,
                _ => {
                    println!("Ошибка: для формул O(h^4) требуется как минимум 5 точек (m >= 4).");
                    continue;
                }
            };
            let m = num_points - 1;

            print!("Введите начальную точку x0: ");
            io::stdout().flush().unwrap();
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let x0: f64 = match input.trim().parse() {
                Ok(val) => val,
                _ => {
                    println!("Некорректное число.");
                    continue;
                }
            };

            print!("Введите шаг h > 0: ");
            io::stdout().flush().unwrap();
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let h: f64 = match input.trim().parse() {
                Ok(val) if val > 0.0 => val,
                _ => {
                    println!("Шаг должен быть положительным числом.");
                    continue;
                }
            };

            // Построение таблицы
            let mut x_vals = Vec::with_capacity(num_points);
            let mut y_vals = Vec::with_capacity(num_points);
            for k in 0..num_points {
                let x = x0 + (k as f64) * h;
                x_vals.push(x);
                y_vals.push((selected_func.f)(x));
            }

            println!("\nТаблица значений функции");
            println!("{:<5} | {:<12} | {:<12}", "k", "x_k", "y_k");
            println!("{:-<34}", "-");
            for k in 0..num_points {
                println!("{:<5} | {:<12.5} | {:<12.5}", k, x_vals[k], y_vals[k]);
            }

            let mut df_h2 = vec![0.0; num_points];
            let mut df_h4 = vec![0.0; num_points];
            let mut d2f_h2 = vec![0.0; num_points];

            //  Первая производная O(h^2): (4), (3), (5)
            df_h2[0] = (-3.0 * y_vals[0] + 4.0 * y_vals[1] - y_vals[2]) / (2.0 * h); // (4)
            for k in 1..m {
                df_h2[k] = (y_vals[k + 1] - y_vals[k - 1]) / (2.0 * h); // (3)
            }
            df_h2[m] = (3.0 * y_vals[m] - 4.0 * y_vals[m - 1] + y_vals[m - 2]) / (2.0 * h); // (5)

            // Первая производная O(h^4): формулы (7) - (11)
            df_h4[0] = (-25.0 * y_vals[0] + 48.0 * y_vals[1] - 36.0 * y_vals[2] + 16.0 * y_vals[3]
                - 3.0 * y_vals[4])
                / (12.0 * h); // (7)
            df_h4[1] = (-3.0 * y_vals[0] - 10.0 * y_vals[1] + 18.0 * y_vals[2] - 6.0 * y_vals[3]
                + y_vals[4])
                / (12.0 * h); // (8)

            for i in 2..=m - 2 {
                df_h4[i] = (y_vals[i - 2] - 8.0 * y_vals[i - 1] + 8.0 * y_vals[i + 1] - y_vals[i + 2])
                    / (12.0 * h); // (9)
            }

            df_h4[m - 1] = (3.0 * y_vals[m] + 10.0 * y_vals[m - 1] - 18.0 * y_vals[m - 2]
                + 6.0 * y_vals[m - 3]
                - y_vals[m - 4])
                / (12.0 * h); // (10)
            df_h4[m] = (25.0 * y_vals[m] - 48.0 * y_vals[m - 1] + 36.0 * y_vals[m - 2]
                - 16.0 * y_vals[m - 3]
                + 3.0 * y_vals[m - 4])
                / (12.0 * h); // (11)

            // Вторая производная O(h^2) для внутренних точек (6)
            for k in 1..m {
                d2f_h2[k] = (y_vals[k + 1] - 2.0 * y_vals[k] + y_vals[k - 1]) / (h * h);
            }

            println!("\nРезультаты");
            println!(
                "{:<4} | {:<9} | {:<9} | {:<9} | {:<9} | {:<9} | {:<9} | {:<9} | {:<9} | {:<9}",
                "k", "x_k", "f_T'", "f'_h2", "err_h2", "f'_h4", "err_h4", "f_T''", "f''_h2", "err_d2"
            );
            println!("{:-<105}", "-");

            for k in 0..num_points {
                let x = x_vals[k];
                let ft_1 = (selected_func.df)(x);
                let ft_2 = (selected_func.d2f)(x);

                let val_h2 = df_h2[k];
                let err_h2 = (ft_1 - val_h2).abs();

                let val_h4 = df_h4[k];
                let err_h4 = (ft_1 - val_h4).abs();

                if k > 0 && k < m {
                    let val_d2 = d2f_h2[k];
                    let err_d2 = (ft_2 - val_d2).abs();
                    println!(
                        "{:<4} | {:<9.3} | {:<9.3} | {:<9.3} | {:<9.3e} | {:<9.3} | {:<9.3e} | {:<9.3} | {:<9.3} | {:<9.3e}",
                        k, x, ft_1, val_h2, err_h2, val_h4, err_h4, ft_2, val_d2, err_d2
                    );
                } else {
                    // Для граничных точек (6) не применима
                    println!(
                        "{:<4} | {:<9.3} | {:<9.3} | {:<9.3} | {:<9.3e} | {:<9.3} | {:<9.3e} | {:<9.3} | {:<9} | {:<9}",
                        k, x, ft_1, val_h2, err_h2, val_h4, err_h4, ft_2, "-", "-"
                    );
                }
            }

            println!("1. Выбрать другую функцию");
            println!("2. Создать новую таблицу");
            println!("0. Выйти из программы");
            let mut choice = String::new();
            io::stdin().read_line(&mut choice).unwrap();
            match choice.trim().parse() {
                Ok(0) => break 'main,
                Ok(1) => break 'table,
                Ok(2) => continue 'table,
                _ => {
                    println!("Неверный ввод. Попробуйте снова.");
                    continue;
                }
            };
        }
    }
}
