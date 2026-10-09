pub fn find_segments_with_sign_change(
    start_section: f64,
    end_section: f64,
    step: f64,
    f: fn(f64) -> f64,
) -> Vec<(f64, f64)> {
    let mut x1 = start_section;
    let mut x2 = x1 + step;
    let mut y1 = f(x1);

    let mut segments: Vec<(f64, f64)> = Vec::new();
    while x2 <= end_section {
        let y2 = f(x2);

        if y1 * y2 <= 0.0 {
            segments.push((x1, x2));
        }
        x1 = x2;
        x2 = x1 + step;
        y1 = y2;
    }

    segments
}
