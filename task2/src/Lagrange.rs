use crate::Node;

pub fn calculate_lagrange(
    selected_nodes: &[Node],
    x_target: f64,
    func: fn(f64) -> f64,
) -> (f64, f64, f64, f64) {
    let n = selected_nodes.len() - 1;
    let mut p_n_l = 0.0;
    let mut sum_l = 0.0;
    let mut sum_abs_l = 0.0;

    // Считаем l_kn(x_target)
    for k in 0..=n {
        let mut l_kn = 1.0;
        let x_k = selected_nodes[k].x;

        for j in 0..=n {
            if j != k {
                let x_j = selected_nodes[j].x;
                l_kn *= (x_target - x_j) / (x_k - x_j);
            }
        }

        // Суммируем для полинома Лагранжа
        p_n_l += l_kn * selected_nodes[k].fx;
        // Должно быть равно 1
        sum_l += l_kn;
        // функция Лебега
        sum_abs_l += l_kn.abs();
    }

    let exact_value = func(x_target);
    let absolute_error = (exact_value - p_n_l).abs();

    (p_n_l, sum_l, sum_abs_l, absolute_error)
}
