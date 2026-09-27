use std::error::Error;
use crate::test_task::test_task_error::TestTaskError;

fn find_number_of_segments_with_sign_change(
    start_section: f64,
    end_section: f64,
    number_of_partitions: usize,
    step: f64,
    f: fn(f64) -> f64,
) -> Result<usize, Box<dyn Error>> {
    if start_section > end_section {
        return Err(Box::new(TestTaskError::InvalidSection))
    }
    if number_of_partitions == 1 {
        return Err(Box::new(TestTaskError::InvalidNumberOfPartitions));
    }

    let mut counter = 0;
    let mut x1 = start_section;
    let mut x2 = x1 + step;
    let mut y1 = f(x1);

    while x2 <= end_section {
        let y2 = f(x2);
        if y1*y2 <= 0.0 {
            counter += 1;
            println!("[{x1}, {x2}]");

        }
        x1 = x2;
        x2 = x1 + step;
        y1 = y2;
    }

    println!("{counter}");
    Ok(counter)
}
